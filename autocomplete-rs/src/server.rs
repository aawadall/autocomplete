use std::net::SocketAddr;
use tonic::{transport::Server as TonicServer, Request, Response, Status};
use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::IntoResponse,
    Json,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::autocomplete::{Autocomplete, Autocomplete2};
use crate::graphql::{create_schema, AppSchema};

pub mod autocomplete_proto {
    tonic::include_proto!("autocomplete");
}

use autocomplete_proto::{
    autocomplete_service_server::{AutocompleteService, AutocompleteServiceServer},
    CompleteRequest, CompleteResponse, Completion,
    InitRequest, InitResponse,
    StatsRequest, StatsResponse,
};

pub struct AutocompleteServiceImpl {
    autocomplete: Autocomplete,
}

#[tonic::async_trait]
impl AutocompleteService for AutocompleteServiceImpl {
    async fn complete(
        &self,
        request: Request<CompleteRequest>,
    ) -> Result<Response<CompleteResponse>, Status> {
        let req = request.into_inner();
        let completions = self.autocomplete.complete(&req.prefix);
        
        let response = CompleteResponse {
            completions: completions.into_iter()
                .map(|(text, score)| Completion {
                    text,
                    score,
                })
                .collect(),
        };
        
        Ok(Response::new(response))
    }

    async fn init(
        &self,
        request: Request<InitRequest>,
    ) -> Result<Response<InitResponse>, Status> {
        let req = request.into_inner();
        let strings: Vec<(String, f32)> = req.strings
            .into_iter()
            .map(|s| (s.text, s.score))
            .collect();
            
        match self.autocomplete.init(&strings) {
            Ok(_) => Ok(Response::new(InitResponse {
                success: true,
                error: String::new(),
            })),
            Err(e) => Ok(Response::new(InitResponse {
                success: false,
                error: e.to_string(),
            })),
        }
    }

    async fn get_stats(
        &self,
        _request: Request<StatsRequest>,
    ) -> Result<Response<StatsResponse>, Status> {
        let response = StatsResponse {
            num_terms: self.autocomplete.num_terms() as i32,
            memory_bytes: self.autocomplete.bytes() as i64,
        };
        
        Ok(Response::new(response))
    }
}

async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
    )
}

pub async fn run_server(grpc_addr: &str, graphql_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let autocomplete = Autocomplete::new();
    let schema = create_schema(autocomplete.clone());
    
    // Create gRPC service
    let grpc_service = AutocompleteServiceImpl {
        autocomplete: autocomplete.clone(),
    };

    // Create GraphQL router
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/playground", get(graphql_playground))
        .with_state(schema);

    // Start both servers
    let grpc_addr = grpc_addr.parse()?;
    let graphql_addr = graphql_addr.parse()?;

    println!("gRPC server listening on {}", grpc_addr);
    println!("GraphQL server listening on {}", graphql_addr);

    tokio::join!(
        TonicServer::builder()
            .add_service(AutocompleteServiceServer::new(grpc_service))
            .serve(grpc_addr),
        axum::Server::bind(&graphql_addr).serve(app.into_make_service())
    );

    Ok(())
} 
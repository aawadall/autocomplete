use tonic::{transport::Server, Request, Response, Status};
use crate::autocomplete::{Autocomplete, Autocomplete2};

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

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;
    let service = AutocompleteServiceImpl {
        autocomplete: Autocomplete::new(),
    };

    println!("Autocomplete server listening on {}", addr);

    Server::builder()
        .add_service(AutocompleteServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
} 
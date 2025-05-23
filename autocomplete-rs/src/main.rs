use std::error::Error;
use clap::Parser;

mod autocomplete;
mod graphql;
mod server;
mod string_pool;
mod trie;
mod types;

/// Autocomplete service with gRPC and GraphQL support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// gRPC server address
    #[arg(short, long, default_value = "[::1]:50051")]
    grpc_addr: String,

    /// GraphQL server address
    #[arg(short, long, default_value = "[::1]:8000")]
    graphql_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Starting Autocomplete Service...");
    println!("gRPC server will listen on: {}", args.grpc_addr);
    println!("GraphQL server will listen on: {}", args.graphql_addr);
    println!("GraphQL Playground available at: http://{}/playground", args.graphql_addr);

    // Start both servers
    server::run_server(&args.grpc_addr, &args.graphql_addr).await?;

    Ok(())
}

use autocomplete_proto::{
    autocomplete_service_client::AutocompleteServiceClient,
    CompleteRequest, InitRequest, StringScore,
};

pub mod autocomplete_proto {
    tonic::include_proto!("autocomplete");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AutocompleteServiceClient::connect("http://[::1]:50051").await?;

    // Initialize with some test data
    let init_request = InitRequest {
        strings: vec![
            StringScore { text: "hello".to_string(), score: 1.0 },
            StringScore { text: "help".to_string(), score: 0.8 },
            StringScore { text: "hell".to_string(), score: 0.6 },
        ],
    };

    let response = client.init(init_request).await?;
    println!("INIT RESPONSE: {:?}", response);

    // Get completions
    let request = CompleteRequest {
        prefix: "hel".to_string(),
        max_results: 10,
    };

    let response = client.complete(request).await?;
    println!("COMPLETE RESPONSE: {:?}", response);

    Ok(())
} 
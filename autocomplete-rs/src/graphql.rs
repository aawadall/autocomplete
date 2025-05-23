use async_graphql::{Object, Schema, SimpleObject, InputObject, EmptySubscription};
use crate::autocomplete::Autocomplete;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(SimpleObject)]
struct Completion {
    text: String,
    score: f32,
}

#[derive(SimpleObject)]
struct CompleteResponse {
    completions: Vec<Completion>,
}

#[derive(SimpleObject)]
struct Stats {
    num_terms: i32,
    memory_bytes: i64,
}

#[derive(SimpleObject)]
struct InitResponse {
    success: bool,
    error: Option<String>,
}

#[derive(InputObject)]
struct StringScoreInput {
    text: String,
    score: f32,
}

pub struct QueryRoot {
    autocomplete: Arc<Mutex<Autocomplete>>,
}

#[Object]
impl QueryRoot {
    async fn complete(&self, prefix: String, _max_results: Option<i32>) -> CompleteResponse {
        let autocomplete = self.autocomplete.lock().await;
        let completions = autocomplete.complete(&prefix);
        let completions = completions.into_iter()
            .map(|(text, score)| Completion { text, score })
            .collect();
        
        CompleteResponse { completions }
    }

    async fn stats(&self) -> Stats {
        let autocomplete = self.autocomplete.lock().await;
        Stats {
            num_terms: autocomplete.num_terms() as i32,
            memory_bytes: autocomplete.bytes() as i64,
        }
    }
}

pub struct MutationRoot {
    autocomplete: Arc<Mutex<Autocomplete>>,
}

#[Object]
impl MutationRoot {
    async fn init(&self, strings: Vec<StringScoreInput>) -> InitResponse {
        let strings: Vec<(String, f32)> = strings
            .into_iter()
            .map(|s| (s.text, s.score))
            .collect();
            
        let mut autocomplete = self.autocomplete.lock().await;
        match autocomplete.init(&strings) {
            Ok(_) => InitResponse {
                success: true,
                error: None,
            },
            Err(e) => InitResponse {
                success: false,
                error: Some(e.to_string()),
            },
        }
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(autocomplete: Arc<Mutex<Autocomplete>>) -> AppSchema {
    Schema::build(
        QueryRoot { autocomplete: autocomplete.clone() },
        MutationRoot { autocomplete },
        EmptySubscription,
    )
    .finish()
} 
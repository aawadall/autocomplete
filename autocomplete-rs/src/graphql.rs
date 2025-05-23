use async_graphql::{Object, Schema, SimpleObject, InputObject};
use crate::autocomplete::Autocomplete;

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
    autocomplete: Autocomplete,
}

#[Object]
impl QueryRoot {
    async fn complete(&self, prefix: String, max_results: Option<i32>) -> CompleteResponse {
        let completions = self.autocomplete.complete(&prefix);
        let completions = completions.into_iter()
            .map(|(text, score)| Completion { text, score })
            .collect();
        
        CompleteResponse { completions }
    }

    async fn stats(&self) -> Stats {
        Stats {
            num_terms: self.autocomplete.num_terms() as i32,
            memory_bytes: self.autocomplete.bytes() as i64,
        }
    }
}

pub struct MutationRoot {
    autocomplete: Autocomplete,
}

#[Object]
impl MutationRoot {
    async fn init(&self, strings: Vec<StringScoreInput>) -> InitResponse {
        let strings: Vec<(String, f32)> = strings
            .into_iter()
            .map(|s| (s.text, s.score))
            .collect();
            
        match self.autocomplete.init(&strings) {
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

pub type AppSchema = Schema<QueryRoot, MutationRoot>;

pub fn create_schema(autocomplete: Autocomplete) -> AppSchema {
    Schema::build(
        QueryRoot { autocomplete: autocomplete.clone() },
        MutationRoot { autocomplete },
        async_graphql::EmptySubscription,
    )
    .finish()
} 
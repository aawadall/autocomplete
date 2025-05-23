use crate::types::ScoreType;
use crate::trie::Trie;
use crate::dictionary::Dictionary;

#[derive(Clone)]
pub struct Autocomplete {
    trie: Trie,
    dictionary: Dictionary,
}

impl Autocomplete {
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            dictionary: Dictionary::new(),
        }
    }

    pub fn init(&mut self, strings: &[(String, ScoreType)]) -> Result<(), String> {
        for (string, score) in strings {
            let id = self.dictionary.insert(string.clone());
            self.trie.insert(string, id, *score);
        }
        Ok(())
    }

    pub fn complete(&self, prefix: &str) -> Vec<(String, ScoreType)> {
        let completions = self.trie.complete(prefix);
        completions
            .into_iter()
            .filter_map(|(id, score)| {
                self.dictionary.get(id).map(|text| (text.to_string(), score))
            })
            .collect()
    }

    pub fn num_terms(&self) -> usize {
        self.dictionary.len()
    }

    pub fn bytes(&self) -> usize {
        // TODO: Implement actual memory usage calculation
        0
    }
} 
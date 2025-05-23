use std::collections::HashMap;
use crate::types::{IdType, ByteRange, global};
use crate::string_pool::ScoredStringPool;
use crate::trie::CompletionTrie;
use crate::dictionary::{FCDictionary, IntegerFCDictionary};
use crate::index::{BlockedInvertedIndex, CompactVector, BitVector};

const BLOCK_SIZE: usize = 1024;

/// Main autocomplete implementation
pub struct Autocomplete {
    string_pool: ScoredStringPool,
    trie: CompletionTrie,
    dictionary: FCDictionary,
    index: BlockedInvertedIndex,
    term_to_id: HashMap<String, IdType>,
    id_to_term: Vec<String>,
    num_terms: usize,
}

impl Autocomplete {
    /// Create a new autocomplete instance
    pub fn new() -> Self {
        Self {
            string_pool: ScoredStringPool::new(),
            trie: CompletionTrie::new(),
            dictionary: FCDictionary::new(),
            index: BlockedInvertedIndex::new(BLOCK_SIZE),
            term_to_id: HashMap::new(),
            id_to_term: Vec::new(),
            num_terms: 0,
        }
    }

    /// Initialize the autocomplete system
    pub fn init(&mut self, strings: &[String], scores: &[IdType]) {
        assert_eq!(strings.len(), scores.len());
        
        // Build string pool
        self.string_pool = ScoredStringPool::new();
        let mut offsets = Vec::with_capacity(strings.len() + 1);
        let mut all_scores = Vec::with_capacity(strings.len());
        let mut total_bytes = 0;
        offsets.push(0);
        for (string, &score) in strings.iter().zip(scores) {
            total_bytes += string.len();
            offsets.push(total_bytes);
            all_scores.push(score);
        }
        self.string_pool.set_offsets(offsets);
        self.string_pool.set_scores(all_scores);
        self.string_pool.set_data(strings.iter().flat_map(|s| s.as_bytes()).cloned().collect());

        // Build dictionary
        self.dictionary.build(strings);

        // Build term mappings
        self.term_to_id.clear();
        self.id_to_term.clear();
        for (i, string) in strings.iter().enumerate() {
            self.term_to_id.insert(string.clone(), (i + 1) as IdType);
            self.id_to_term.push(string.clone());
        }
        self.num_terms = strings.len();

        // Build trie
        self.trie.clear();
        for (i, string) in strings.iter().enumerate() {
            self.trie.insert(string, (i + 1) as IdType);
        }

        // Build index
        self.index.clear();
        for (i, _string) in strings.iter().enumerate() {
            let term_id = (i + 1) as IdType;
            self.index.add_doc(term_id, term_id);
        }
    }

    /// Find completions for a prefix
    pub fn complete(&self, prefix: &str) -> Vec<(String, IdType)> {
        let mut results = Vec::new();
        
        // Get completion IDs from trie
        let completion_ids = self.trie.complete(prefix);
        
        // Look up strings and scores
        for &id in &completion_ids {
            if let Some(string) = self.dictionary.lookup(id) {
                let scored_range = self.string_pool.get(id as usize);
                results.push((string, scored_range.score));
            }
        }

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

    /// Get the number of terms
    pub fn num_terms(&self) -> usize {
        self.num_terms
    }

    /// Get the size in bytes
    pub fn bytes(&self) -> usize {
        self.string_pool.bytes() +
        self.trie.num_nodes() * std::mem::size_of::<char>() +
        self.dictionary.bytes() +
        self.index.num_blocks() * std::mem::size_of::<IdType>() +
        self.term_to_id.capacity() * std::mem::size_of::<(String, IdType)>() +
        self.id_to_term.capacity() * std::mem::size_of::<String>()
    }
}

/// Integer-based autocomplete implementation
pub struct Autocomplete2 {
    string_pool: ScoredStringPool,
    trie: CompletionTrie,
    dictionary: IntegerFCDictionary,
    index: BlockedInvertedIndex,
    term_to_id: HashMap<String, IdType>,
    id_to_term: Vec<String>,
    num_terms: usize,
}

impl Autocomplete2 {
    /// Create a new integer-based autocomplete instance
    pub fn new() -> Self {
        Self {
            string_pool: ScoredStringPool::new(),
            trie: CompletionTrie::new(),
            dictionary: IntegerFCDictionary::new(),
            index: BlockedInvertedIndex::new(BLOCK_SIZE),
            term_to_id: HashMap::new(),
            id_to_term: Vec::new(),
            num_terms: 0,
        }
    }

    /// Initialize the autocomplete system
    pub fn init(&mut self, strings: &[String], scores: &[IdType]) {
        assert_eq!(strings.len(), scores.len());
        
        // Build string pool
        self.string_pool = ScoredStringPool::new();
        let mut offsets = Vec::with_capacity(strings.len() + 1);
        let mut all_scores = Vec::with_capacity(strings.len());
        let mut total_bytes = 0;
        offsets.push(0);
        for (string, &score) in strings.iter().zip(scores) {
            total_bytes += string.len();
            offsets.push(total_bytes);
            all_scores.push(score);
        }
        self.string_pool.set_offsets(offsets);
        self.string_pool.set_scores(all_scores);
        self.string_pool.set_data(strings.iter().flat_map(|s| s.as_bytes()).cloned().collect());

        // Build dictionary
        self.dictionary.build(strings);

        // Build term mappings
        self.term_to_id.clear();
        self.id_to_term.clear();
        for (i, string) in strings.iter().enumerate() {
            self.term_to_id.insert(string.clone(), (i + 1) as IdType);
            self.id_to_term.push(string.clone());
        }
        self.num_terms = strings.len();

        // Build trie
        self.trie.clear();
        for (i, string) in strings.iter().enumerate() {
            self.trie.insert(string, (i + 1) as IdType);
        }

        // Build index
        self.index.clear();
        for (i, _string) in strings.iter().enumerate() {
            let term_id = (i + 1) as IdType;
            self.index.add_doc(term_id, term_id);
        }
    }

    /// Find completions for a prefix
    pub fn complete(&self, prefix: &str) -> Vec<(String, IdType)> {
        let mut results = Vec::new();
        let mut completion = Vec::new();
        
        // Get completion IDs from trie
        let completion_ids = self.trie.complete(prefix);
        
        // Look up strings and scores
        for &id in &completion_ids {
            let len = self.dictionary.extract(id, &mut completion);
            if len > 0 {
                let scored_range = self.string_pool.get(id as usize);
                let string = String::from_utf8_lossy(&completion).into_owned();
                results.push((string, scored_range.score));
            }
        }

        // Sort by score (descending)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

    /// Get the number of terms
    pub fn num_terms(&self) -> usize {
        self.num_terms
    }

    /// Get the size in bytes
    pub fn bytes(&self) -> usize {
        self.string_pool.bytes() +
        self.trie.num_nodes() * std::mem::size_of::<char>() +
        self.index.num_blocks() * std::mem::size_of::<IdType>() +
        self.term_to_id.capacity() * std::mem::size_of::<(String, IdType)>() +
        self.id_to_term.capacity() * std::mem::size_of::<String>()
    }
} 
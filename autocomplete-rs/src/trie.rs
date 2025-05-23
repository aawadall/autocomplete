use std::collections::HashMap;
use crate::types::IdType;

#[derive(Default, Clone)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    id: Option<IdType>,
    score: f32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            id: None,
            score: 0.0,
        }
    }

    fn is_terminal(&self) -> bool {
        self.id.is_some()
    }
}

#[derive(Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, completion: &str, id: IdType, score: f32) {
        let mut current = &mut self.root;
        let chars: Vec<char> = completion.chars().collect();
        
        for &c in &chars {
            current = current.children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }
        
        current.id = Some(id);
        current.score = score;
    }

    pub fn remove(&mut self, completion: &str) -> bool {
        let mut path = Vec::new();
        let mut current = &mut self.root;
        
        // First pass: find the path to the node
        for c in completion.chars() {
            if let Some(next) = current.children.get_mut(&c) {
                path.push(c);
                current = next;
            } else {
                return false; // String not found
            }
        }
        
        // If the node is not a terminal, the string wasn't in the trie
        if !current.is_terminal() {
            return false;
        }
        
        // Remove the terminal marker
        current.id = None;
        current.score = 0.0;
        
        // Second pass: remove empty nodes
        let mut current = &mut self.root;
        for &c in &path[..path.len()-1] {
            current = current.children.get_mut(&c).unwrap();
        }
        
        // Remove the last node if it's empty
        if current.children.is_empty() && !current.is_terminal() {
            current.children.remove(&path[path.len()-1]);
        }
        
        true
    }

    pub fn complete(&self, prefix: &str) -> Vec<(IdType, f32)> {
        let mut current = &self.root;
        
        // Navigate to the prefix node
        for c in prefix.chars() {
            if let Some(next) = current.children.get(&c) {
                current = next;
            } else {
                return Vec::new(); // Prefix not found
            }
        }
        
        // Collect all completions from this node
        let mut results = Vec::new();
        self.collect_completions(current, &mut results);
        results
    }

    fn collect_completions(&self, node: &TrieNode, results: &mut Vec<(IdType, f32)>) {
        if let Some(id) = node.id {
            results.push((id, node.score));
        }
        
        for child in node.children.values() {
            self.collect_completions(child, results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_complete() {
        let mut trie = Trie::new();
        trie.insert("hello", 1, 1.0);
        trie.insert("help", 2, 0.8);
        trie.insert("world", 3, 0.5);
        
        let completions = trie.complete("hel");
        assert_eq!(completions.len(), 2);
        assert!(completions.contains(&(1, 1.0)));
        assert!(completions.contains(&(2, 0.8)));
    }

    #[test]
    fn test_trie_remove() {
        let mut trie = Trie::new();
        trie.insert("hello", 1, 1.0);
        trie.insert("help", 2, 0.8);
        
        assert!(trie.remove("hello"));
        assert!(!trie.remove("hello")); // Already removed
        assert!(trie.remove("help"));
        
        let completions = trie.complete("hel");
        assert_eq!(completions.len(), 0);
    }
} 
use std::collections::HashMap;
use crate::types::{IdType, CompletionType};

/// A node in the completion trie
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_terminal: bool,
    completion_ids: Vec<IdType>,
}

impl TrieNode {
    /// Create a new trie node
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_terminal: false,
            completion_ids: Vec::new(),
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, c: char) -> &mut TrieNode {
        self.children.entry(c).or_insert_with(TrieNode::new)
    }

    /// Get a child node
    pub fn get_child(&self, c: char) -> Option<&TrieNode> {
        self.children.get(&c)
    }

    /// Check if this is a terminal node
    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    /// Set this node as terminal
    pub fn set_terminal(&mut self) {
        self.is_terminal = true;
    }

    /// Add a completion ID
    pub fn add_completion_id(&mut self, id: IdType) {
        self.completion_ids.push(id);
    }

    /// Get completion IDs
    pub fn completion_ids(&self) -> &[IdType] {
        &self.completion_ids
    }
}

/// A trie for prefix-based completion
pub struct CompletionTrie {
    root: TrieNode,
    num_nodes: usize,
    num_completions: usize,
}

impl CompletionTrie {
    /// Create a new completion trie
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            num_nodes: 1,
            num_completions: 0,
        }
    }

    /// Insert a completion string
    pub fn insert(&mut self, completion: &str, id: IdType) {
        let mut node = &mut self.root;
        for c in completion.chars() {
            node = node.add_child(c);
            self.num_nodes += 1;
        }
        node.set_terminal();
        node.add_completion_id(id);
        self.num_completions += 1;
    }

    /// Find all completions for a prefix
    pub fn complete(&self, prefix: &str) -> Vec<IdType> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.get_child(c) {
                Some(next) => node = next,
                None => return Vec::new(),
            }
        }
        self.collect_completions(node)
    }

    /// Collect all completion IDs from a node and its children
    fn collect_completions(&self, node: &TrieNode) -> Vec<IdType> {
        let mut completions = Vec::new();
        self.collect_completions_recursive(node, &mut completions);
        completions
    }

    /// Recursive helper for collecting completions
    fn collect_completions_recursive(&self, node: &TrieNode, completions: &mut Vec<IdType>) {
        if node.is_terminal() {
            completions.extend_from_slice(node.completion_ids());
        }
        for child in node.children.values() {
            self.collect_completions_recursive(child, completions);
        }
    }

    /// Remove a completion string
    pub fn remove(&mut self, completion: &str) -> bool {
        let mut chars: Vec<char> = completion.chars().collect();
        if chars.is_empty() {
            return false;
        }

        // First, find if the completion exists and build the path
        let mut path = Vec::new();
        let mut current = &self.root;
        
        for &c in &chars {
            match current.get_child(c) {
                Some(next) => {
                    path.push(c);
                    current = next;
                }
                None => return false,
            }
        }

        if !current.is_terminal() {
            return false;
        }

        // Now remove it by traversing the path again
        let mut current = &mut self.root;
        let mut parent = None;
        
        for &c in &path {
            if let Some(next) = current.children.get_mut(&c) {
                parent = Some((c, current));
                current = next;
            }
        }

        // Remove the completion
        current.completion_ids.clear();
        current.is_terminal = false;
        self.num_completions -= 1;

        // Clean up empty nodes
        while let Some((c, p)) = parent {
            if current.children.is_empty() && !current.is_terminal() {
                p.children.remove(&c);
                self.num_nodes -= 1;
                current = p;
                parent = None;
            } else {
                break;
            }
        }

        true
    }

    /// Clear the trie
    pub fn clear(&mut self) {
        self.root = TrieNode::new();
        self.num_nodes = 1;
        self.num_completions = 0;
    }

    /// Get the number of nodes
    pub fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    /// Get the number of completions
    pub fn num_completions(&self) -> usize {
        self.num_completions
    }
} 
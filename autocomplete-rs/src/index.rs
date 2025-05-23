use std::collections::HashMap;
use crate::types::{IdType, global};

/// Block in the inverted index
struct Block {
    term_id: IdType,
    num_docs: usize,
    docs: Vec<IdType>,
}

impl Block {
    /// Create a new block
    fn new(term_id: IdType) -> Self {
        Self {
            term_id,
            num_docs: 0,
            docs: Vec::new(),
        }
    }

    /// Add a document to the block
    fn add_doc(&mut self, doc_id: IdType) {
        self.docs.push(doc_id);
        self.num_docs += 1;
    }

    /// Get the number of documents in the block
    fn size(&self) -> usize {
        self.num_docs
    }
}

/// Blocked inverted index for efficient document retrieval
pub struct BlockedInvertedIndex {
    blocks: Vec<Block>,
    term_to_block: HashMap<IdType, usize>,
    block_size: usize,
}

impl BlockedInvertedIndex {
    /// Create a new blocked inverted index
    pub fn new(block_size: usize) -> Self {
        Self {
            blocks: Vec::new(),
            term_to_block: HashMap::new(),
            block_size,
        }
    }

    /// Add a document to the index
    pub fn add_doc(&mut self, term_id: IdType, doc_id: IdType) {
        let block_idx = self.term_to_block.entry(term_id).or_insert_with(|| {
            self.blocks.push(Block::new(term_id));
            self.blocks.len() - 1
        });

        let block = &mut self.blocks[*block_idx];
        block.add_doc(doc_id);

        // If block is full, create a new one
        if block.size() >= self.block_size {
            self.blocks.push(Block::new(term_id));
            *block_idx = self.blocks.len() - 1;
        }
    }

    /// Get documents for a term
    pub fn get_docs(&self, term_id: IdType) -> Vec<IdType> {
        let mut docs = Vec::new();
        
        // Find all blocks for the term
        let mut current_idx = self.term_to_block.get(&term_id).copied();
        while let Some(idx) = current_idx {
            let block = &self.blocks[idx];
            docs.extend_from_slice(&block.docs);
            
            // Check if there's a next block for the same term
            current_idx = if idx + 1 < self.blocks.len() && self.blocks[idx + 1].term_id == term_id {
                Some(idx + 1)
            } else {
                None
            };
        }

        docs
    }

    /// Get the number of blocks
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// Get the total number of documents
    pub fn num_docs(&self) -> usize {
        self.blocks.iter().map(|b| b.size()).sum()
    }

    /// Clear the index
    pub fn clear(&mut self) {
        self.blocks.clear();
        self.term_to_block.clear();
    }
}

/// Compact vector for efficient storage
pub struct CompactVector {
    data: Vec<u8>,
    element_size: usize,
    num_elements: usize,
}

impl CompactVector {
    /// Create a new compact vector
    pub fn new(element_size: usize) -> Self {
        Self {
            data: Vec::new(),
            element_size,
            num_elements: 0,
        }
    }

    /// Add an element to the vector
    pub fn push(&mut self, element: &[u8]) {
        assert_eq!(element.len(), self.element_size);
        self.data.extend_from_slice(element);
        self.num_elements += 1;
    }

    /// Get an element from the vector
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        if index >= self.num_elements {
            return None;
        }
        let start = index * self.element_size;
        let end = start + self.element_size;
        Some(&self.data[start..end])
    }

    /// Get the number of elements
    pub fn size(&self) -> usize {
        self.num_elements
    }

    /// Get the size in bytes
    pub fn bytes(&self) -> usize {
        self.data.len()
    }
}

/// Bit vector for efficient bit-level operations
pub struct BitVector {
    data: Vec<u8>,
    num_bits: usize,
}

impl BitVector {
    /// Create a new bit vector
    pub fn new(num_bits: usize) -> Self {
        let num_bytes = (num_bits + 7) / 8;
        Self {
            data: vec![0; num_bytes],
            num_bits,
        }
    }

    /// Set a bit
    pub fn set(&mut self, index: usize) {
        if index < self.num_bits {
            let byte_idx = index / 8;
            let bit_idx = index % 8;
            self.data[byte_idx] |= 1 << bit_idx;
        }
    }

    /// Clear a bit
    pub fn clear(&mut self, index: usize) {
        if index < self.num_bits {
            let byte_idx = index / 8;
            let bit_idx = index % 8;
            self.data[byte_idx] &= !(1 << bit_idx);
        }
    }

    /// Test a bit
    pub fn test(&self, index: usize) -> bool {
        if index < self.num_bits {
            let byte_idx = index / 8;
            let bit_idx = index % 8;
            (self.data[byte_idx] & (1 << bit_idx)) != 0
        } else {
            false
        }
    }

    /// Get the number of bits
    pub fn size(&self) -> usize {
        self.num_bits
    }

    /// Get the size in bytes
    pub fn bytes(&self) -> usize {
        self.data.len()
    }
} 
use crate::types::IdType;

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
    blocks: Vec<Vec<IdType>>,
    block_size: usize,
}

impl BlockedInvertedIndex {
    /// Create a new blocked inverted index
    pub fn new(block_size: usize) -> Self {
        Self {
            blocks: Vec::new(),
            block_size,
        }
    }

    /// Add a document to the index
    pub fn insert(&mut self, id: IdType) {
        if self.blocks.is_empty() || self.blocks.last().unwrap().len() >= self.block_size {
            self.blocks.push(Vec::with_capacity(self.block_size));
        }
        self.blocks.last_mut().unwrap().push(id);
    }

    /// Get documents for a term
    pub fn get(&self, block_id: usize) -> Option<&[IdType]> {
        self.blocks.get(block_id).map(|v| v.as_slice())
    }

    /// Get the number of blocks
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// Get the block size
    pub fn block_size(&self) -> usize {
        self.block_size
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
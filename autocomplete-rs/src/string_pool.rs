use crate::types::{ByteRange, IdType};

/// Represents a scored byte range
#[derive(Debug, Clone)]
pub struct ScoredByteRange {
    pub string: ByteRange,
    pub score: IdType,
}

/// Manages a pool of scored strings
pub struct ScoredStringPool {
    scores: Vec<IdType>,
    offsets: Vec<usize>,
    data: Vec<u8>,
}

impl ScoredStringPool {
    /// Create a new empty string pool
    pub fn new() -> Self {
        let mut pool = Self {
            scores: Vec::new(),
            offsets: Vec::new(),
            data: Vec::new(),
        };
        pool.init();
        pool
    }

    /// Initialize the pool
    pub fn init(&mut self) {
        self.push_back_offset(0);
    }

    /// Resize the pool
    pub fn resize(&mut self, num_bytes: usize, k: u32) {
        self.scores.resize(k as usize, 0);
        self.data.resize(num_bytes, 0);
    }

    /// Clear the pool
    pub fn clear(&mut self) {
        self.offsets.clear();
    }

    /// Get the number of strings in the pool
    pub fn size(&self) -> usize {
        assert!(!self.offsets.is_empty());
        self.offsets.len() - 1
    }

    /// Get the total number of bytes used
    pub fn bytes(&self) -> usize {
        self.offsets.last().copied().unwrap_or(0)
    }

    /// Get a mutable reference to the data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Add a new offset
    pub fn push_back_offset(&mut self, offset: usize) {
        self.offsets.push(offset);
    }

    /// Get a mutable reference to the scores
    pub fn scores_mut(&mut self) -> &mut [IdType] {
        &mut self.scores
    }

    /// Get a reference to the scores
    pub fn scores(&self) -> &[IdType] {
        &self.scores
    }

    /// Get a scored byte range at the given index
    pub fn get(&self, i: usize) -> ScoredByteRange {
        assert!(i < self.size());
        ScoredByteRange {
            string: ByteRange {
                begin: unsafe { self.data.as_ptr().add(self.offsets[i]) },
                end: unsafe { self.data.as_ptr().add(self.offsets[i + 1]) },
            },
            score: self.scores[i],
        }
    }

    /// Set the offsets vector
    pub fn set_offsets(&mut self, offsets: Vec<usize>) {
        self.offsets = offsets;
    }

    /// Set the scores vector
    pub fn set_scores(&mut self, scores: Vec<IdType>) {
        self.scores = scores;
    }

    /// Set the data vector
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}

/// Iterator over scored strings in the pool
pub struct ScoredStringPoolIterator<'a> {
    pool: &'a ScoredStringPool,
    pos: usize,
}

impl<'a> ScoredStringPoolIterator<'a> {
    /// Create a new iterator
    pub fn new(pool: &'a ScoredStringPool, pos: usize) -> Self {
        Self { pool, pos }
    }

    /// Check if the iterator is empty
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    /// Get the number of strings
    pub fn size(&self) -> usize {
        self.pool.size()
    }

    /// Get the pool
    pub fn pool(&self) -> &ScoredStringPool {
        self.pool
    }
}

impl<'a> Iterator for ScoredStringPoolIterator<'a> {
    type Item = ScoredByteRange;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.pool.size() {
            let item = self.pool.get(self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl ScoredStringPool {
    /// Get an iterator over the scored strings
    pub fn iter(&self) -> ScoredStringPoolIterator {
        ScoredStringPoolIterator::new(self, 0)
    }
} 
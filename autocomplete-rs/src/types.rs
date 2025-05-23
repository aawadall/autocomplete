use std::ops::Range;

/// Type alias for document and term IDs
pub type IdType = u32;

/// Type alias for completion type (vector of term IDs)
pub type CompletionType = Vec<IdType>;

/// Represents a range of values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueRange {
    pub begin: u64,
    pub end: u64,
}

impl ValueRange {
    /// Check if the range is invalid
    pub fn is_invalid(&self) -> bool {
        self.begin == u64::MAX || self.end == u64::MAX || self.begin > self.end
    }

    /// Check if the range is valid
    pub fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    /// Check if a value is contained in the range
    pub fn contains(&self, val: u64) -> bool {
        val >= self.begin && val <= self.end
    }
}

/// Represents a scored range
#[derive(Debug, Clone)]
pub struct ScoredRange {
    pub range: ValueRange,
    pub min_pos: u32,
    pub min_val: IdType,
}

impl ScoredRange {
    /// Compare two scored ranges
    pub fn greater(l: &ScoredRange, r: &ScoredRange) -> bool {
        l.min_val > r.min_val
    }
}

/// Represents a byte range
#[derive(Debug, Clone, Copy)]
pub struct ByteRange {
    pub begin: *const u8,
    pub end: *const u8,
}

/// Represents a range of 32-bit integers
#[derive(Debug, Clone, Copy)]
pub struct Uint32Range {
    pub begin: *const u32,
    pub end: *const u32,
}

/// Global constants
pub mod global {
    use super::IdType;

    /// Invalid term ID
    pub const INVALID_TERM_ID: IdType = IdType::MAX;
    
    /// Terminator value
    pub const TERMINATOR: IdType = 0;
    
    /// Not found value
    pub const NOT_FOUND: u64 = u64::MAX;
    
    /// Linear scan threshold
    pub const LINEAR_SCAN_THRESHOLD: u64 = 8;
}

/// Convert a string to a byte range
pub fn string_to_byte_range(s: &str) -> ByteRange {
    let begin = s.as_ptr();
    let end = unsafe { begin.add(s.len()) };
    ByteRange { begin, end }
}

/// Convert a completion to a uint32 range
pub fn completion_to_uint32_range(c: &CompletionType) -> Uint32Range {
    Uint32Range {
        begin: c.as_ptr(),
        end: unsafe { c.as_ptr().add(c.len()) },
    }
} 
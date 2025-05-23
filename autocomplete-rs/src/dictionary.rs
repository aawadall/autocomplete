use std::collections::HashMap;
use crate::types::{ByteRange, IdType, global};

/// Front-coded dictionary for string compression
pub struct FCDictionary {
    data: Vec<u8>,
    offsets: Vec<u32>,
    num_strings: usize,
    total_size: usize,
}

impl FCDictionary {
    /// Create a new front-coded dictionary
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offsets: Vec::new(),
            num_strings: 0,
            total_size: 0,
        }
    }

    /// Build the dictionary from a list of strings
    pub fn build(&mut self, strings: &[String]) {
        if strings.is_empty() {
            return;
        }

        self.num_strings = strings.len();
        self.offsets.clear();
        self.data.clear();
        self.total_size = 0;

        // Sort strings for better compression
        let mut sorted_strings: Vec<_> = strings.iter().collect();
        sorted_strings.sort();

        // First string is stored completely
        let first = sorted_strings[0];
        self.offsets.push(0);
        self.data.extend_from_slice(first.as_bytes());
        self.total_size += first.len();

        // Process remaining strings
        for i in 1..sorted_strings.len() {
            let prev = sorted_strings[i - 1];
            let curr = sorted_strings[i];
            
            // Find common prefix
            let lcp = self.longest_common_prefix(prev, curr);
            
            // Store offset and remaining string
            self.offsets.push(self.total_size as u32);
            self.data.push(lcp as u8);
            self.data.extend_from_slice(&curr.as_bytes()[lcp..]);
            self.total_size += 1 + curr.len() - lcp;
        }
    }

    /// Find the longest common prefix between two strings
    fn longest_common_prefix(&self, a: &str, b: &str) -> usize {
        a.bytes()
            .zip(b.bytes())
            .take_while(|(x, y)| x == y)
            .count()
    }

    /// Look up a string in the dictionary
    pub fn lookup(&self, id: IdType) -> Option<String> {
        if id == 0 || id > self.num_strings as IdType {
            return None;
        }

        let id = (id - 1) as usize;
        let offset = self.offsets[id] as usize;
        
        if id == 0 {
            // First string is stored completely
            let end = if id + 1 < self.offsets.len() {
                self.offsets[id + 1] as usize
            } else {
                self.data.len()
            };
            Some(String::from_utf8_lossy(&self.data[offset..end]).into_owned())
        } else {
            // Other strings are front-coded
            let lcp = self.data[offset] as usize;
            let prev = self.lookup(id as IdType - 1)?;
            let mut result = prev[..lcp].to_string();
            let end = if id + 1 < self.offsets.len() {
                self.offsets[id + 1] as usize
            } else {
                self.data.len()
            };
            result.push_str(std::str::from_utf8(&self.data[offset + 1..end]).unwrap());
            Some(result)
        }
    }

    /// Get the number of strings in the dictionary
    pub fn size(&self) -> usize {
        self.num_strings
    }

    /// Get the total size of the compressed data
    pub fn total_size(&self) -> usize {
        self.total_size
    }

    /// Get the size of the dictionary in bytes
    pub fn bytes(&self) -> usize {
        std::mem::size_of_val(&self.num_strings) +
        std::mem::size_of_val(&self.total_size) +
        self.offsets.len() * std::mem::size_of::<u32>() +
        self.data.len()
    }
}

/// Integer-based front-coded dictionary
pub struct IntegerFCDictionary {
    headers: Vec<u8>,
    buckets: Vec<u8>,
    size: usize,
}

impl IntegerFCDictionary {
    /// Create a new integer-based front-coded dictionary
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            buckets: Vec::new(),
            size: 0,
        }
    }

    /// Build the dictionary from a list of strings
    pub fn build(&mut self, strings: &[String]) {
        if strings.is_empty() {
            return;
        }

        self.size = strings.len();
        self.headers.clear();
        self.buckets.clear();

        // Sort strings for better compression
        let mut sorted_strings: Vec<_> = strings.iter().collect();
        sorted_strings.sort();

        // Process strings
        for i in 0..sorted_strings.len() {
            let curr = sorted_strings[i];
            let lcp = if i > 0 {
                self.longest_common_prefix(sorted_strings[i - 1], curr)
            } else {
                0
            };

            // Store header
            self.headers.extend_from_slice(curr.as_bytes());
            
            // Store bucket
            self.buckets.push(lcp as u8);
            self.buckets.push((curr.len() - lcp) as u8);
            self.buckets.extend_from_slice(&curr.as_bytes()[lcp..]);
        }
    }

    /// Find the longest common prefix between two strings
    fn longest_common_prefix(&self, a: &str, b: &str) -> usize {
        a.bytes()
            .zip(b.bytes())
            .take_while(|(x, y)| x == y)
            .count()
    }

    /// Extract a string from the dictionary
    pub fn extract(&self, id: IdType, completion: &mut Vec<u8>) -> u8 {
        if id == 0 || id > self.size as IdType {
            return 0;
        }

        let id = (id - 1) as usize;
        let bucket_start = id * 2;
        let lcp = self.buckets[bucket_start] as usize;
        let remaining = self.buckets[bucket_start + 1] as usize;

        completion.clear();
        completion.extend_from_slice(&self.headers[id..id + lcp]);
        completion.extend_from_slice(&self.buckets[bucket_start + 2..bucket_start + 2 + remaining]);

        (lcp + remaining) as u8
    }

    /// Get the number of strings in the dictionary
    pub fn size(&self) -> usize {
        self.size
    }
} 
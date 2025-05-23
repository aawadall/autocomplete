use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::constants::{MAX_NUM_CHARS_PER_QUERY, MAX_NUM_TERMS_PER_QUERY};

/// Parameters for the autocomplete system
#[derive(Debug, Default)]
pub struct Parameters {
    pub num_terms: u32,
    pub max_string_length: u32,
    pub num_completions: u32,
    pub universe: u32,
    pub num_levels: u32,
    pub nodes_per_level: Vec<u32>,
    pub collection_basename: String,
}

impl Parameters {
    /// Creates a new empty Parameters instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads parameters from a statistics file
    pub fn load(&mut self) -> io::Result<()> {
        let stats_path = if self.collection_basename.ends_with(".mapped.stats") {
            Path::new(&self.collection_basename).to_path_buf()
        } else {
            Path::new(&self.collection_basename).with_extension("mapped.stats")
        };
        
        let file = File::open(stats_path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Read basic statistics
        self.num_terms = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing num_terms"))??
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.max_string_length = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing max_string_length"))??
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.num_completions = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing num_completions"))??
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.universe = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing universe"))??
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.num_levels = lines.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing num_levels"))??
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Validate basic statistics
        if self.num_terms == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "num_terms must be > 0"));
        }
        if self.max_string_length == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "max_string_length must be > 0"));
        }
        if self.num_completions == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "num_completions must be > 0"));
        }
        if self.universe < self.num_completions {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "universe must be >= num_completions"));
        }
        if self.num_levels == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "num_levels must be > 0"));
        }

        // Validate against constants
        if self.max_string_length > MAX_NUM_CHARS_PER_QUERY {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("max_string_length ({}) exceeds MAX_NUM_CHARS_PER_QUERY ({})",
                    self.max_string_length, MAX_NUM_CHARS_PER_QUERY)
            ));
        }
        if self.num_levels > MAX_NUM_TERMS_PER_QUERY {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("num_levels ({}) exceeds MAX_NUM_TERMS_PER_QUERY ({})",
                    self.num_levels, MAX_NUM_TERMS_PER_QUERY)
            ));
        }

        // Read nodes per level
        self.nodes_per_level = Vec::with_capacity(self.num_levels as usize);
        for _ in 0..self.num_levels {
            let count = lines.next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing nodes_per_level data"))??
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            self.nodes_per_level.push(count);
        }

        if self.nodes_per_level.len() != self.num_levels as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File with statistics may be truncated or malformed"
            ));
        }

        Ok(())
    }
} 
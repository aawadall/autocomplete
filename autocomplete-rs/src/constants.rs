// Constants for the autocomplete system
pub const MAX_K: u32 = 15;
pub const MAX_NUM_TERMS_PER_QUERY: u32 = 64;
pub const MAX_NUM_CHARS_PER_QUERY: u32 = 128;
pub const POOL_SIZE: usize = (MAX_K as usize) * (MAX_NUM_CHARS_PER_QUERY as usize);

// Compile-time assertion
const _: () = assert!(MAX_NUM_TERMS_PER_QUERY < 256, "MAX_NUM_TERMS_PER_QUERY must be < 256"); 
use autocomplete_rs::constants::*;

#[test]
fn test_constants() {
    // Test MAX_K
    assert!(MAX_K > 0, "MAX_K should be positive");
    assert!(MAX_K <= 100, "MAX_K should be reasonably small");

    // Test MAX_NUM_TERMS_PER_QUERY
    assert!(MAX_NUM_TERMS_PER_QUERY > 0, "MAX_NUM_TERMS_PER_QUERY should be positive");
    assert!(MAX_NUM_TERMS_PER_QUERY < 256, "MAX_NUM_TERMS_PER_QUERY must be < 256");

    // Test MAX_NUM_CHARS_PER_QUERY
    assert!(MAX_NUM_CHARS_PER_QUERY > 0, "MAX_NUM_CHARS_PER_QUERY should be positive");
    assert!(MAX_NUM_CHARS_PER_QUERY >= MAX_K, "MAX_NUM_CHARS_PER_QUERY should be >= MAX_K");

    // Test POOL_SIZE
    assert!(POOL_SIZE > 0, "POOL_SIZE should be positive");
    assert_eq!(POOL_SIZE, (MAX_K as usize) * (MAX_NUM_CHARS_PER_QUERY as usize), 
        "POOL_SIZE should be MAX_K * MAX_NUM_CHARS_PER_QUERY");
} 
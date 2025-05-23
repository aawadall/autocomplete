use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use autocomplete_rs::parameters::Parameters;
use autocomplete_rs::constants::{MAX_NUM_CHARS_PER_QUERY, MAX_NUM_TERMS_PER_QUERY};

fn create_test_stats_file() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "1000").unwrap();  // num_terms
    writeln!(file, "50").unwrap();    // max_string_length
    writeln!(file, "500").unwrap();   // num_completions
    writeln!(file, "1000").unwrap();  // universe
    writeln!(file, "3").unwrap();     // num_levels
    writeln!(file, "100").unwrap();   // nodes_per_level[0]
    writeln!(file, "200").unwrap();   // nodes_per_level[1]
    writeln!(file, "300").unwrap();   // nodes_per_level[2]
    file
}

#[test]
fn test_parameters_load_valid() {
    let test_file = create_test_stats_file();
    let mut params = Parameters::new();
    let path = test_file.path().to_str().unwrap().to_string();
    println!("Test file path: {}", path);
    params.collection_basename = path;
    
    match params.load() {
        Ok(_) => println!("Load succeeded"),
        Err(e) => println!("Load failed: {}", e),
    }
    
    assert!(params.load().is_ok());
    assert_eq!(params.num_terms, 1000);
    assert_eq!(params.max_string_length, 50);
    assert_eq!(params.num_completions, 500);
    assert_eq!(params.universe, 1000);
    assert_eq!(params.num_levels, 3);
    assert_eq!(params.nodes_per_level, vec![100, 200, 300]);
}

#[test]
fn test_parameters_load_invalid_file() {
    let mut params = Parameters::new();
    params.collection_basename = "nonexistent_file".to_string();
    assert!(params.load().is_err());
}

#[test]
fn test_parameters_load_invalid_data() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "0").unwrap();  // invalid num_terms
    writeln!(file, "50").unwrap();
    writeln!(file, "500").unwrap();
    writeln!(file, "1000").unwrap();
    writeln!(file, "3").unwrap();
    writeln!(file, "100").unwrap();
    writeln!(file, "200").unwrap();
    writeln!(file, "300").unwrap();

    let mut params = Parameters::new();
    params.collection_basename = file.path().to_str().unwrap().to_string();
    assert!(params.load().is_err());
}

#[test]
fn test_parameters_load_invalid_constants() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "1000").unwrap();
    writeln!(file, "{}", MAX_NUM_CHARS_PER_QUERY + 1).unwrap();  // exceeds MAX_NUM_CHARS_PER_QUERY
    writeln!(file, "500").unwrap();
    writeln!(file, "1000").unwrap();
    writeln!(file, "3").unwrap();
    writeln!(file, "100").unwrap();
    writeln!(file, "200").unwrap();
    writeln!(file, "300").unwrap();

    let mut params = Parameters::new();
    params.collection_basename = file.path().to_str().unwrap().to_string();
    assert!(params.load().is_err());
}

#[test]
fn test_parameters_load_truncated() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "1000").unwrap();
    writeln!(file, "50").unwrap();
    writeln!(file, "500").unwrap();
    writeln!(file, "1000").unwrap();
    writeln!(file, "3").unwrap();
    writeln!(file, "100").unwrap();
    // Missing nodes_per_level entries

    let mut params = Parameters::new();
    params.collection_basename = file.path().to_str().unwrap().to_string();
    assert!(params.load().is_err());
} 
# Autocomplete-rs

This project is a Rust port of the original C++ autocomplete system. The goal is to maintain the same functionality while leveraging Rust's safety guarantees and modern tooling.

## Project Status

Currently, we are in the process of porting the core components from C++ to Rust. The following components have been ported:

- Basic constants and configuration
- Parameters management
- Performance measurement probes

## Next Steps

1. Continue porting core components:
   - Scored string pool
   - Completion trie
   - Blocked inverted index
   - Front-coded dictionary

2. Port and adapt unit tests to ensure functionality matches the original implementation

3. Containerize the application using Docker for easy deployment and testing

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with specific test
cargo test test_name -- --nocapture
```

## Original Project

This is a port of the original C++ autocomplete system, which provides efficient string completion functionality. The original implementation can be found in the `archive` directory.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
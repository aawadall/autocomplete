# C++ Code Structure Documentation

This document outlines the structure of the original C++ implementation that is being ported to Rust.

## Core Components

### 1. Constants and Configuration
- **File**: `constants.hpp`
- **Purpose**: Defines system-wide constants and limits
- **Key Constants**:
  - `MAX_K`: Maximum number of completions
  - `MAX_NUM_TERMS_PER_QUERY`: Maximum terms per query
  - `MAX_NUM_CHARS_PER_QUERY`: Maximum characters per query
  - `POOL_SIZE`: Size of the string pool

### 2. Parameters Management
- **File**: `parameters.hpp`
- **Purpose**: Manages system configuration parameters
- **Key Struct**: `parameters`
  - `num_terms`: Total number of terms
  - `max_string_length`: Maximum string length
  - `num_completions`: Number of completions
  - `universe`: Size of the universe
  - `num_levels`: Number of levels in the index
  - `nodes_per_level`: Vector of nodes per level
  - `collection_basename`: Base name for collection files

### 3. Performance Measurement
- **File**: `probe.hpp`
- **Purpose**: Performance measurement and timing
- **Key Structs**:
  - `nop_probe`: No-operation probe
  - `timer_probe`: Timer-based performance measurement

### 4. String Pool Management
- **File**: `scored_string_pool.hpp`
- **Purpose**: Manages a pool of scored strings
- **Key Components**:
  - String storage
  - Score management
  - Pool operations

### 5. Completion Trie
- **File**: `completion_trie.hpp`
- **Purpose**: Implements the completion trie data structure
- **Key Features**:
  - Prefix-based completion
  - Node management
  - Traversal operations

### 6. Blocked Inverted Index
- **File**: `blocked_inverted_index.hpp`
- **Purpose**: Implements blocked inverted indexing
- **Key Components**:
  - Block management
  - Index operations
  - Query processing

### 7. Front-Coded Dictionary
- **File**: `fc_dictionary.hpp`
- **Purpose**: Implements front-coding for dictionary compression
- **Key Features**:
  - String compression
  - Dictionary operations
  - Lookup functionality

## Data Pipeline

1. **Input Processing**
   - Read input completions
   - Sort lexicographically
   - Generate statistics

2. **Index Building**
   - Build front-coded dictionary
   - Construct completion trie
   - Create blocked inverted index

3. **Query Processing**
   - Parse input query
   - Traverse completion trie
   - Search inverted index
   - Return top-k completions

## Key Methods and Operations

### Dictionary Operations
```cpp
// Front-coded dictionary
void build_dictionary();
void compress_strings();
std::string lookup(uint32_t id);
```

### Trie Operations
```cpp
// Completion trie
void insert(const std::string& completion);
std::vector<std::string> complete(const std::string& prefix);
```

### Index Operations
```cpp
// Blocked inverted index
void build_index();
std::vector<uint32_t> search(const std::vector<uint32_t>& terms);
```

### Query Processing
```cpp
// Query handling
std::vector<std::string> process_query(const std::string& query);
void rank_completions(std::vector<std::string>& completions);
```

## Dependencies and Relationships

1. **Core Dependencies**
   - Constants → Parameters
   - Parameters → All major components
   - Probe → Performance measurement

2. **Data Structure Dependencies**
   - Front-coded Dictionary → Completion Trie
   - Completion Trie → Blocked Inverted Index
   - All components → String Pool

3. **Pipeline Dependencies**
   - Input Processing → Index Building
   - Index Building → Query Processing
   - Query Processing → All components

## Porting Strategy

1. **Phase 1: Core Components**
   - Constants and configuration
   - Parameters management
   - Performance measurement

2. **Phase 2: Data Structures**
   - String pool
   - Completion trie
   - Front-coded dictionary

3. **Phase 3: Index and Query**
   - Blocked inverted index
   - Query processing
   - Pipeline integration

4. **Phase 4: Testing and Optimization**
   - Unit tests
   - Integration tests
   - Performance optimization 
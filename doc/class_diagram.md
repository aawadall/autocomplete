# C++ Class Diagram

This document provides a comprehensive view of all classes in the C++ implementation and their relationships.

## Main Class Diagram

```mermaid
classDiagram
    class Parameters {
        +uint32_t num_terms
        +uint32_t max_string_length
        +uint32_t num_completions
        +uint32_t universe
        +uint32_t num_levels
        +vector~uint32_t~ nodes_per_level
        +string collection_basename
        +load()
    }

    class Probe {
        <<interface>>
        +start(id: uint64_t)
        +stop(id: uint64_t)
    }

    class NopProbe {
        +start(id: uint64_t)
        +stop(id: uint64_t)
    }

    class TimerProbe {
        -vector~Timer~ timers
        +start(id: uint64_t)
        +stop(id: uint64_t)
        +get_duration(id: uint64_t)
    }

    class Timer {
        -Instant start_time
        -Duration total_duration
        +start()
        +stop()
        +get_duration()
    }

    class ScoredStringPool {
        -vector~id_type~ m_scores
        -vector~size_t~ m_offsets
        -vector~uint8_t~ m_data
        +init()
        +resize(size_t, uint32_t)
        +clear()
        +size()
        +bytes()
        +data()
        +push_back_offset(size_t)
        +scores()
        +const_scores()
    }

    class ScoredByteRange {
        +byte_range string
        +id_type score
    }

    class TrieNode {
        -unordered_map~char, TrieNode*~ children
        -bool is_terminal
        -vector~uint32_t~ completion_ids
        +add_child(char)
        +get_child(char)
        +is_terminal()
    }

    class CompletionTrie {
        -TrieNode* root
        -size_t num_nodes
        -size_t num_completions
        +insert(string)
        +complete(string)
        +remove(string)
        +clear()
    }

    class FCDictionary {
        -vector~char~ data
        -vector~uint32_t~ offsets
        -size_t num_strings
        -size_t total_size
        +build(vector~string~)
        +lookup(uint32_t)
        +compress()
        +decompress(uint32_t)
    }

    class IntegerFCDictionary {
        -vector~uint32_t~ m_headers
        -vector~uint8_t~ m_buckets
        -size_t m_size
        +build(vector~string~)
        +lookup(uint32_t)
        +extract(id_type, completion_type)
    }

    class Block {
        -vector~uint32_t~ doc_ids
        -uint32_t min_doc_id
        -uint32_t max_doc_id
        +add_doc(uint32_t)
        +get_docs()
        +get_range()
    }

    class InvertedIndex {
        -vector~Block~ blocks
        -unordered_map~string, vector~uint32_t~~ term_to_blocks
        -size_t block_size
        +add_document(uint32_t, vector~string~)
        +search(vector~string~)
        +merge_blocks()
        +clear()
    }

    class CompactVector {
        -vector~uint64_t~ m_bits
        -uint8_t m_width
        -uint64_t m_mask
        +build(vector~uint64_t~)
        +access(uint64_t)
        +size()
    }

    class BitVector {
        -vector~uint64_t~ m_bits
        -size_t m_size
        +build(bit_vector_builder*)
        +size()
        +bytes()
        +operator[](uint64_t)
        +get_bits(uint64_t, uint64_t)
    }

    class MinHeap {
        -vector~T~ m_q
        -Comparator m_comparator
        +reserve(uint64_t)
        +top()
        +push(T)
        +pop()
        +clear()
        +empty()
        +size()
    }

    class Autocomplete {
        -Parameters params
        -ScoredStringPool string_pool
        -CompletionTrie trie
        -FCDictionary dictionary
        -InvertedIndex index
        +build_index(string)
        +complete(string)
        +search(vector~string~)
    }

    class Autocomplete2 {
        -Parameters params
        -ScoredStringPool string_pool
        -CompletionTrie trie
        -FCDictionary dictionary
        -InvertedIndex index
        -CompactVector docid_to_lexid
        +build_index(string)
        +complete(string)
        +search(vector~string~)
    }

    class Autocomplete3 {
        -Parameters params
        -ScoredStringPool string_pool
        -CompletionTrie trie
        -FCDictionary dictionary
        -InvertedIndex index
        -MinHeap min_priority_queue
        +build_index(string)
        +complete(string)
        +search(vector~string~)
    }

    class Autocomplete4 {
        -Parameters params
        -ScoredStringPool string_pool
        -CompletionTrie trie
        -FCDictionary dictionary
        -BlockedInvertedIndex index
        +build_index(string)
        +complete(string)
        +search(vector~string~)
    }

    %% Relationships
    Probe <|-- NopProbe
    Probe <|-- TimerProbe
    TimerProbe *-- Timer
    Autocomplete *-- Parameters
    Autocomplete *-- ScoredStringPool
    Autocomplete *-- CompletionTrie
    Autocomplete *-- FCDictionary
    Autocomplete *-- InvertedIndex
    CompletionTrie *-- TrieNode
    InvertedIndex *-- Block
    ScoredStringPool *-- ScoredByteRange
    Autocomplete2 --|> Autocomplete
    Autocomplete3 --|> Autocomplete
    Autocomplete4 --|> Autocomplete
    Autocomplete3 *-- MinHeap
    Autocomplete2 *-- CompactVector
    Autocomplete4 *-- BlockedInvertedIndex
```

## Component Dependencies

```mermaid
graph TD
    subgraph Core
        Parameters
        Probe
        Timer
    end

    subgraph Data Structures
        ScoredStringPool
        CompletionTrie
        FCDictionary
        IntegerFCDictionary
        InvertedIndex
        BlockedInvertedIndex
        CompactVector
        BitVector
        MinHeap
    end

    subgraph Implementation
        Autocomplete
        Autocomplete2
        Autocomplete3
        Autocomplete4
    end

    %% Dependencies
    Parameters --> ScoredStringPool
    Parameters --> CompletionTrie
    Parameters --> FCDictionary
    Parameters --> InvertedIndex
    Parameters --> IntegerFCDictionary
    
    ScoredStringPool --> Autocomplete
    CompletionTrie --> Autocomplete
    FCDictionary --> Autocomplete
    InvertedIndex --> Autocomplete
    IntegerFCDictionary --> Autocomplete2
    CompactVector --> Autocomplete2
    MinHeap --> Autocomplete3
    BlockedInvertedIndex --> Autocomplete4

    style Core fill:#f9f,stroke:#333,stroke-width:2px
    style Data Structures fill:#9f9,stroke:#333,stroke-width:2px
    style Implementation fill:#99f,stroke:#333,stroke-width:2px
```

## Memory Layout

```mermaid
graph TD
    subgraph Memory Organization
        direction TB
        Stack[Stack Memory] --> Heap[Heap Memory]
        Heap --> Data[Data Structures]
        Data --> Strings[String Pool]
        Data --> Trie[Trie Nodes]
        Data --> Dict[Dictionary]
        Data --> Index[Inverted Index]
        Data --> Compact[Compact Vectors]
        Data --> BitVec[Bit Vectors]
    end

    style Memory Organization fill:#f9f,stroke:#333,stroke-width:2px
```

## Key Features and Methods

### Core Components
- **Parameters**: Configuration management
- **Probe**: Performance measurement interface
- **Timer**: Time tracking implementation

### Data Structures
- **ScoredStringPool**: String and score management
- **CompletionTrie**: Prefix-based completion
- **FCDictionary**: String compression
- **IntegerFCDictionary**: Integer-based dictionary
- **InvertedIndex**: Term-based search
- **BlockedInvertedIndex**: Blocked term-based search
- **CompactVector**: Space-efficient vector
- **BitVector**: Bit-level operations
- **MinHeap**: Priority queue implementation

### Main Implementation
- **Autocomplete**: Base implementation
- **Autocomplete2**: Integer-based optimization
- **Autocomplete3**: Min-heap based optimization
- **Autocomplete4**: Blocked index optimization

## Usage Example

```cpp
// Initialize components
Parameters params;
params.load("config.stats");

ScoredStringPool pool(POOL_SIZE);
CompletionTrie trie;
FCDictionary dict;
InvertedIndex index;

// Build autocomplete system
Autocomplete ac(params, pool, trie, dict, index);
ac.build_index("data.txt");

// Use the system
auto completions = ac.complete("hello");
auto results = ac.search({"hello", "world"});
``` 
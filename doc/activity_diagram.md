# Activity Diagrams

This document provides activity diagrams for the main workflows in the autocomplete system.

## System Initialization and Index Building

```mermaid
graph TD
    Start([Start]) --> LoadParams[Load Parameters]
    LoadParams --> InitComponents[Initialize Components]
    InitComponents --> BuildTrie[Build Completion Trie]
    BuildTrie --> BuildDict[Build Front-Coded Dictionary]
    BuildDict --> BuildIndex[Build Inverted Index]
    BuildIndex --> BuildForwardIndex[Build Forward Index]
    BuildForwardIndex --> End([End])

    style Start fill:#f9f,stroke:#333,stroke-width:2px
    style End fill:#f9f,stroke:#333,stroke-width:2px
```

## Autocomplete Query Processing

```mermaid
graph TD
    Start([Start]) --> InputQuery[Input Query]
    InputQuery --> ParseQuery[Parse Query Terms]
    ParseQuery --> CheckPrefix[Check Prefix in Trie]
    
    CheckPrefix -->|Prefix Found| GetCompletions[Get Completions]
    CheckPrefix -->|No Prefix| ReturnEmpty[Return Empty Results]
    
    GetCompletions --> ScoreCompletions[Score Completions]
    ScoreCompletions --> SortResults[Sort by Score]
    SortResults --> ReturnResults[Return Top-K Results]
    
    ReturnEmpty --> End([End])
    ReturnResults --> End

    style Start fill:#f9f,stroke:#333,stroke-width:2px
    style End fill:#f9f,stroke:#333,stroke-width:2px
```

## Search Operation Flow

```mermaid
graph TD
    Start([Start]) --> InputTerms[Input Search Terms]
    InputTerms --> ParseTerms[Parse Search Terms]
    ParseTerms --> LookupTerms[Lookup Terms in Dictionary]
    
    LookupTerms -->|All Terms Found| GetPostings[Get Posting Lists]
    LookupTerms -->|Terms Not Found| ReturnEmpty[Return Empty Results]
    
    GetPostings --> IntersectLists[Intersect Posting Lists]
    IntersectLists --> ScoreDocs[Score Documents]
    ScoreDocs --> SortResults[Sort by Score]
    SortResults --> ReturnResults[Return Top-K Results]
    
    ReturnEmpty --> End([End])
    ReturnResults --> End

    style Start fill:#f9f,stroke:#333,stroke-width:2px
```

## String Pool Management

```mermaid
graph TD
    Start([Start]) --> CheckCapacity[Check Pool Capacity]
    CheckCapacity -->|Full| RemoveLowest[Remove Lowest Score]
    CheckCapacity -->|Space Available| AddString[Add New String]
    
    RemoveLowest --> AddString
    AddString --> UpdateScores[Update Scores]
    UpdateScores --> SortPool[Sort Pool by Score]
    SortPool --> End([End])

    style Start fill:#f9f,stroke:#333,stroke-width:2px
    style End fill:#f9f,stroke:#333,stroke-width:2px
```

## Blocked Inverted Index Operations

```mermaid
graph TD
    Start([Start]) --> InputDoc[Input Document]
    InputDoc --> ExtractTerms[Extract Terms]
    ExtractTerms --> CheckBlocks[Check Existing Blocks]
    
    CheckBlocks -->|Block Found| UpdateBlock[Update Block]
    CheckBlocks -->|New Block| CreateBlock[Create New Block]
    
    UpdateBlock --> MergeCheck[Check Merge Condition]
    CreateBlock --> MergeCheck
    
    MergeCheck -->|Merge Needed| MergeBlocks[Merge Blocks]
    MergeCheck -->|No Merge| UpdateIndex[Update Index]
    
    MergeBlocks --> UpdateIndex
    UpdateIndex --> End([End])

    style Start fill:#f9f,stroke:#333,stroke-width:2px
    style End fill:#f9f,stroke:#333,stroke-width:2px
```

## Performance Measurement Flow

```mermaid
graph TD
    Start([Start]) --> StartTimer[Start Timer]
    StartTimer --> Operation[Perform Operation]
    Operation --> StopTimer[Stop Timer]
    StopTimer --> RecordMetrics[Record Metrics]
    RecordMetrics --> AnalyzePerformance[Analyze Performance]
    AnalyzePerformance --> End([End])

    style Start fill:#f9f,stroke:#333,stroke-width:2px
    style End fill:#f9f,stroke:#333,stroke-width:2px
```

## Key Operations Description

### System Initialization
1. Load configuration parameters
2. Initialize core components
3. Build data structures
4. Set up indexes

### Query Processing
1. Parse and validate input
2. Check prefix in trie
3. Retrieve and score completions
4. Sort and return results

### Search Operations
1. Process search terms
2. Lookup in dictionary
3. Retrieve and intersect posting lists
4. Score and rank results

### String Pool Management
1. Maintain fixed-size pool
2. Handle insertions and removals
3. Update and sort scores
4. Manage memory efficiently

### Blocked Index Operations
1. Process document updates
2. Manage block structure
3. Handle block merges
4. Maintain index consistency

### Performance Measurement
1. Track operation timing
2. Record performance metrics
3. Analyze system behavior
4. Optimize based on results 
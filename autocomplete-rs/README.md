# Autocomplete Service

A high-performance autocomplete service written in Rust, supporting both gRPC and GraphQL interfaces.

## Features

- **Dual API Support**
  - gRPC interface for high-performance RPC calls
  - GraphQL interface for flexible querying
  - Shared backend implementation for both APIs

- **Core Features**
  - Fast prefix-based autocomplete
  - Score-based ranking of suggestions
  - Memory-efficient string storage
  - Concurrent request handling

- **API Endpoints**
  - gRPC: `[::1]:50051` (configurable)
  - GraphQL: `[::1]:8000/graphql` (configurable)
  - GraphQL Playground: `[::1]:8000/playground`

## Project Status

### Completed
- ✅ Basic autocomplete implementation
- ✅ gRPC server implementation
- ✅ GraphQL server implementation
- ✅ Command-line configuration
- ✅ Shared backend between APIs

### In Progress
- 🔄 Documentation
- 🔄 Testing suite
- 🔄 Performance benchmarks

### Planned
- ⏳ Authentication
- ⏳ Rate limiting
- ⏳ Metrics and monitoring
- ⏳ Docker support
- ⏳ Client examples in multiple languages

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Cargo

### Building
```bash
cargo build --release
```

### Running
```bash
# Default configuration
cargo run

# Custom addresses
cargo run -- --grpc-addr 127.0.0.1:50051 --graphql-addr 127.0.0.1:8000

# Show help
cargo run -- --help
```

## API Usage

### gRPC
```protobuf
service AutocompleteService {
    rpc Complete(CompleteRequest) returns (CompleteResponse);
    rpc Init(InitRequest) returns (InitResponse);
    rpc GetStats(StatsRequest) returns (StatsResponse);
}
```

### GraphQL
```graphql
type Query {
    complete(prefix: String!, maxResults: Int): CompleteResponse!
    stats: StatsResponse!
}

type Mutation {
    init(strings: [StringInput!]!): InitResponse!
}
```

## Project Structure

```
autocomplete-rs/
├── src/
│   ├── main.rs           # Entry point and CLI
│   ├── autocomplete.rs   # Core autocomplete logic
│   ├── graphql.rs        # GraphQL schema and resolvers
│   ├── server.rs         # Server implementations
│   ├── string_pool.rs    # String interning
│   ├── trie.rs          # Trie data structure
│   └── types.rs         # Common types
├── proto/
│   └── autocomplete.proto # gRPC service definition
└── schema/
    └── schema.graphql    # GraphQL schema
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
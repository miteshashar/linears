# Development Guide

## Prerequisites

- Rust toolchain (stable, 1.75+)
- Linear API key for testing (`LINEARS_API_KEY`)

## Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Install globally
cargo install --path .
```

## Testing

The test suite uses `cargo test` and includes:

- **Integration tests** (`tests/cli_integration.rs`): End-to-end CLI testing with httpmock
- **Snapshot tests** (`tests/snapshot_tests.rs`): Codegen drift detection using insta
- **Test factories** (`tests/factories/`): Deterministic data generators
- **Mock server** (`tests/support/`): Linear GraphQL API simulation

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_list_issues_json

# Update snapshots (when schema changes)
INSTA_UPDATE=always cargo test
```

## Linting

```bash
cargo fmt
cargo clippy
```

## Project Structure

```
linears/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library exports for tests
│   ├── cli/                 # Clap command definitions
│   ├── common/              # Shared types, utilities, constants
│   ├── generated/           # Codegen output (never hand-edit)
│   ├── query_builder/       # Query construction
│   ├── mutation_builder/    # Mutation construction
│   ├── client/              # HTTP client, retries
│   ├── render/              # Output formatters
│   └── validate/            # Input validation
├── xtask/                   # Schema sync and codegen
├── schemas/linear/          # Checked-in schema
└── tests/
    ├── cli_integration.rs   # CLI integration tests
    ├── snapshot_tests.rs    # Snapshot tests
    ├── factories/           # Test data factories
    ├── support/             # Mock server helpers
    └── snapshots/           # Approved snapshot files
```

## Schema Management

linears is schema-driven. When Linear updates their API:

```bash
# 1. Sync schema from Linear
cargo xtask schema sync

# 2. Regenerate code
cargo xtask codegen

# 3. Update snapshots
INSTA_UPDATE=always cargo test

# 4. Review and commit
git diff
git add -A && git commit -m "chore: sync Linear schema"
```

### Schema Commands (xtask)

| Command | Description |
|---------|-------------|
| `cargo xtask schema sync` | Fetch latest schema from Linear |
| `cargo xtask schema diff` | Show diff with upstream |
| `cargo xtask codegen` | Regenerate Rust code from schema |

## Contributing

1. Fork and clone
2. Create feature branch
3. Make changes with tests
4. Run `cargo fmt && cargo clippy && cargo test`
5. Submit PR

# linears

A Rust CLI providing complete coverage of Linear's GraphQL API surface area.

## Overview

`linears` is a schema-driven CLI tool that generates query and mutation commands directly from Linear's GraphQL schema. It supports:

- **Discovery**: List all available resources and mutation operations
- **Queries**: List, get, and search entities with filtering and pagination
- **Mutations**: Create, update, delete, archive with helper commands + universal `mutate`
- **Multiple output formats**: table (default), JSON, YAML, NDJSON, text

## Quick Start

```bash
# Clone the repository
git clone <repo-url>
cd linears

# Run the setup script
./init.sh

# Set your Linear API key
export LINEAR_API_KEY='lin_api_...'

# Explore available resources
cargo run -- resources

# List your issues
cargo run -- list issue

# Get a specific issue
cargo run -- get issue ENG-123
```

## Installation

### Prerequisites

- Rust toolchain (stable, 1.75+)
- LINEAR_API_KEY environment variable

### Build from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Install globally
cargo install --path .
```

## Usage

### Discovery Commands

```bash
# List all query resources
linears resources

# List all mutation operations
linears ops
```

### Query Commands

```bash
# List entities
linears list issue
linears list issue --first 50
linears list issue --all  # Auto-paginate (max 1000)
linears list issue --filter '{"team":{"id":{"eq":"..."}}}'

# Get single entity (auto-detects UUID vs identifier)
linears get issue abc123-uuid-here
linears get issue ENG-123

# Search entities
linears search issue "bug in login"

# Raw GraphQL query
linears raw --query 'query { viewer { id name } }'
linears raw --query ./query.graphql
```

### Mutation Commands

```bash
# Create entity
linears create issue --input '{"title":"New issue","teamId":"..."}'
linears create issue --input-file issue.yaml

# Update entity
linears update issue ENG-123 --set '{"title":"Updated title"}'

# Delete entity
linears delete issue ENG-123

# Archive/Unarchive
linears archive issue ENG-123
linears unarchive issue ENG-123

# Universal mutation
linears mutate issueCreate --vars '{"input":{...}}'
```

### Query Options

| Flag | Description |
|------|-------------|
| `--first N` | Limit results (default: 20) |
| `--after CURSOR` | Forward pagination cursor |
| `--last N` | Backward pagination limit |
| `--before CURSOR` | Backward pagination cursor |
| `--all` | Auto-paginate all results (max 1000) |
| `--include-archived` | Include archived entities |
| `--order-by ENUM` | Sort order |
| `--filter JSON/YAML` | Filter expression |
| `--filter-file PATH` | Filter from file |
| `--preset minimal/default/wide` | Field selection preset |
| `--select FIELDS` | Comma-separated scalar fields |
| `--expand RELATION[:fields]` | Include relation data |

### Global Flags

| Flag | Description |
|------|-------------|
| `--out FORMAT` | Output: json, yaml, table, text, ndjson |
| `--pretty` | Pretty-print JSON/YAML |
| `--no-color` | Disable colored output |
| `--verbose` / `-v` | Show GraphQL query |
| `--endpoint URL` | Override API endpoint |
| `--timeout SECS` | Request timeout |

## Output Formats

### Table (default)

Human-friendly table with relative dates and semantic colors.

### JSON

```json
{
  "resource": "issue",
  "operation": "list",
  "pageInfo": {"hasNextPage": true, "endCursor": "..."},
  "nodes": [...]
}
```

### YAML

Same structure as JSON, formatted as YAML.

### NDJSON

One JSON object per line (for streaming/pipes/LLMs).

### Text

Human-readable single-entity view.

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `LINEAR_API_KEY` | Yes | Personal API key |
| `LINEAR_ENDPOINT` | No | Override API endpoint |
| `LINEARS_OUTPUT` | No | Default output format |
| `LINEARS_WORKSPACE` | No | Workspace for multi-workspace keys |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error |
| 3 | Network error |
| 4 | GraphQL error |

## Schema Management

```bash
# View schema info
linears schema info

# Show diff with upstream
linears schema diff

# Sync schema (maintainers)
cargo xtask schema sync
cargo xtask codegen
```

## Development

```bash
# Run setup
./init.sh

# Build
cargo build

# Test
cargo test

# Lint
cargo fmt
cargo clippy

# Sync schema and regenerate code
cargo xtask schema sync
cargo xtask codegen
```

## Project Structure

```
linears/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli/                  # Clap command definitions
│   ├── generated/            # Codegen output (never hand-edited)
│   ├── query_builder/        # Query construction
│   ├── mutation_builder/     # Mutation construction
│   ├── client/               # HTTP client, retries
│   ├── render/               # Output formatters
│   └── validate/             # Input validation
├── xtask/                    # Schema sync and codegen
├── schemas/linear/           # Checked-in schema
└── tests/                    # Unit and integration tests
```

## License

MIT

# linears

A Rust CLI providing complete coverage of Linear's GraphQL API.

> **Work in Progress**: This project is under active development and may have breaking changes. Feedback welcome - feel free to try it out and open an issue if you run into problems or have suggestions.

## Features

- **Complete API coverage**: 140 query resources, 330+ mutations
- **Schema-driven**: Auto-generated from Linear's GraphQL schema
- **Multiple outputs**: table, JSON, YAML, NDJSON, text
- **Smart pagination**: `--all` flag auto-paginates (max 1000)
- **LLM-friendly**: NDJSON streaming, structured errors, `--no-color`

## Quick Start

```bash
# Install from source
cargo install --path .

# Set your API key
export LINEARS_API_KEY='lin_api_...'

# Explore available resources
linears resources

# List your issues
linears list issue

# Get a specific issue
linears get issue ENG-123
```

## Authentication

1. Get your API key: **Linear Settings > API > Personal API keys**
2. Export it:
   ```bash
   export LINEARS_API_KEY='lin_api_...'
   ```
3. For multi-workspace keys, specify workspace:
   ```bash
   linears list issue --workspace my-workspace
   ```

## Installation

> **Note**: Not yet published to crates.io. Install from source.

```bash
git clone <repo-url>
cd linears
cargo install --path .
```

**Prerequisites**: Rust 1.75+

## Usage

### Discovery

```bash
linears resources          # List all 140 query resources
linears ops                # List all 330+ mutation operations
```

### Queries

```bash
# List entities
linears list issue
linears list issue --first 50
linears list issue --all                              # Auto-paginate
linears list issue --filter '{"team":{"id":{"eq":"..."}}}'

# Get single entity (auto-detects UUID vs identifier)
linears get issue abc123-uuid
linears get issue ENG-123

# Search
linears search issue "bug in login"

# Raw GraphQL
linears raw --query 'query { viewer { id name } }'
linears raw --query ./query.graphql --var id=abc-123
```

### Mutations

```bash
# Create
linears create issue --input '{"title":"Bug fix","teamId":"..."}'
linears create issue --input-file issue.yaml

# Update
linears update issue ENG-123 --set '{"title":"Updated"}'

# Delete / Archive
linears delete issue ENG-123
linears archive issue ENG-123
linears unarchive issue ENG-123

# Any mutation
linears mutate issueCreate --vars '{"input":{...}}'
```

## Query Options

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

## Global Flags

| Flag | Description |
|------|-------------|
| `--out FORMAT` | Output: json, yaml, table, text, ndjson |
| `--pretty` | Pretty-print JSON/YAML |
| `--no-color` | Disable colored output |
| `-v, --verbose` | Show GraphQL query |
| `--endpoint URL` | Override API endpoint |
| `--timeout SECS` | Request timeout (default: 30) |
| `--workspace SLUG` | Workspace for multi-workspace keys |

## Output Formats

| Format | Use case |
|--------|----------|
| `table` | Human-readable (default) |
| `json` | Structured data, scripting |
| `yaml` | Config files, readability |
| `ndjson` | Streaming, LLM pipelines |
| `text` | Single-entity human view |

```bash
linears list issue --out json --pretty
linears list issue --out ndjson          # One object per line
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `LINEARS_API_KEY` | Yes | Personal API key |
| `LINEARS_ENDPOINT` | No | Override API endpoint |
| `LINEARS_OUTPUT` | No | Default output format |
| `LINEARS_WORKSPACE` | No | Default workspace |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Authentication error |
| 3 | Network error |
| 4 | GraphQL error |

## Schema Info

```bash
linears schema info    # View schema version and stats
linears schema diff    # Compare local vs upstream
```

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for build, test, and contribution guide.

## License

MIT

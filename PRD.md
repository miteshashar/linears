PRD — linears (Rust CLI for Linear GraphQL with full entity coverage)

1) Product intent

Build a single CLI that provides complete coverage of Linear’s GraphQL surface area (as expressed by the checked-in schema), including:
- Queries: list/get/smart-search across all entities/resources
- Mutations (v1): create/update/archive/unarchive/delete and any other mutation operations exposed by the schema
- LLM-friendly I/O: JSON + YAML, plus human-friendly table/text (table default)

Design is compile-time schema-driven: schema is synced at dev time, checked in, and used for codegen. No runtime schema reflection.

⸻

2) Scope (v1)

Must-have
- Dev-time schema sync from Linear SDK repo schema file (packages/sdk/src/schema.graphql), committed to this repo.
- Dev-time codegen to generate a fixed command registry:
- Resource enum (Query root resources)
- MutationOp enum (Mutation root operations)
- Per-resource capabilities: filter/orderBy/pagination/includeArchived (when present)
- Per-resource selection presets and smart-search plan
- Per-mutation variable schemas (best-effort metadata for validation/help)
- Fixed verbs (no runtime-generated subcommands). Resources/ops are compile-time enums generated from schema.
- Auth: personal API key only via environment variable:
- LINEAR_API_KEY is required for any network call
- No CLI option for API key
- No OAuth support
- Input formats for filters/variables/inputs: JSON + YAML
- Output formats: table (default), JSON, YAML, text, NDJSON
- Output format detection: LINEARS_OUTPUT env var, then --out flag, then default (table)
- TDD-first: snapshot tests fail on schema drift; mutation/query builders covered.

Non-goals (v1)
- A full TUI
- Implementing a full GraphQL typechecker/validator
- Offline mode beyond schema/index artifacts (response caching deferred)

⸻

3) Primary UX

Command summary
- Discovery
- linears resources
- linears ops (lists mutation ops)
- Query
- linears list <resource> [options]
- linears get <resource> <id|key> [options]  (auto-detects UUID vs readable key like ENG-123)
- linears search <resource> <text> [options]
- linears raw --query <FILE|STRING> [--vars <FILE>] [--var k=v...]
- Mutation
- Convenience CRUD-ish helpers (generated only when schema supports the underlying operation)
- linears create <resource> --input <JSON|YAML> [--input-file <path>]
- linears update <resource> <id|key> --set <JSON|YAML> [--set-file <path>]
- linears delete <resource> <id|key> (only if delete op exists)
- linears archive <resource> <id|key> (only if archive op exists)
- linears unarchive <resource> <id|key> (only if unarchive op exists)
- Universal mutation escape hatch (full coverage)
- linears mutate <op> [--vars <JSON|YAML>] [--vars-file <path>] [--var k=v...]
- Schema tooling (dev workflow)
- linears schema info
- linears schema diff
- linears schema sync (intended for maintainers; implemented via xtask, see §6)

Global flags
- --out json|yaml|table|text|ndjson (default table; also reads LINEARS_OUTPUT env var)
- --pretty (pretty JSON/YAML)
- --no-color
- --verbose / -v (show GraphQL query being sent; useful for debugging)
- --endpoint <URL> (optional override; default Linear endpoint)
- --timeout <secs> (optional)

Version info (linears --version)
- CLI version
- Schema version: commit hash, date, permalink (from schema.meta.json)

Query options (resource-aware; only enabled if schema indicates support)
- Pagination:
- --first <n> (default 20)
- --after <cursor>
- --last <n>
- --before <cursor>
- --all (auto-paginate; polite throttling; max 1000 records)
- Archived:
- --include-archived
- Sorting:
- --order-by <ENUM> (generated per resource if orderBy exists)
- Filtering:
- --filter <JSON|YAML> (inline)
- --filter-file <path.{json,yaml,yml}>
- Field selection (bounded and safe):
- --preset minimal|default|wide (resource-specific, generated)
- --select <comma-separated> (top-level scalar fields; conservative)
- --expand <relation[:fields]> (one-level relation expansion, bounded)

Mutation options
- --out …, --pretty, --endpoint …
- For helpers:
- create: --input/--input-file maps to underlying createX(input: …)
- update: --set/--set-file maps to underlying updateX(input: { id, ... }) (or schema-equivalent)
- archive/unarchive/delete: map to schema-equivalent mutation signatures
- For mutate:
- --vars/--vars-file/--var provide GraphQL variables (JSON/YAML), passed as variables (never interpolated)

⸻

4) Output conventions

Default json response envelope

Always return a stable envelope:
- Query list: { "resource", "operation", "pageInfo"?, "nodes": [...] }
- Query get: { "resource", "operation", "entity": {...} }
- Search: { "resource", "operation", "strategy", "nodes": [...] }
- Mutation: { "op", "operation", "result": {...} }
- Errors: { "error": { "kind", "message", "details"?, "hint"? }, "graphqlErrors"?: [...] }

ndjson

One JSON object per line (stream/pipes/LLMs).

table / text

Human-friendly views driven by --preset (default preset for table/text if not specified).

Display conventions:
- Dates: relative + locale-aware (e.g., "2 hours ago", "yesterday", "Jan 11")
- Colors: minimal, semantic (status/priority), compatible with light/dark terminals
- ID auto-detection: UUID pattern vs readable key (e.g., ENG-123)

⸻

5) Smart search (compile-time plan generation)

For search <resource> <text>:
	1.	Prefer a native search query field if schema provides one for that resource category.
	2.	Else generate a filter OR-heuristic using the resource’s filter input type:
- Exact-first: id, identifier, key, slug (if present)
- Contains-insensitive: name, title, description (if present)
	3.	Else return a structured error recommending raw, and include detected candidate fields.

Search results must be deterministic (query + variables) and snapshot-testable.

⸻

6) Schema & codegen (dev-time, checked-in)

Source of truth
- schemas/linear/schema.graphql (checked in)

Dev tooling (xtask pattern)

Preferred workflow:
- cargo xtask schema sync
- Fetch from https://raw.githubusercontent.com/linear/linear/master/packages/sdk/src/schema.graphql
- Record commit hash (via GitHub API) for version tracking
- Normalize formatting for stable diffs
- Write schemas/linear/schema.graphql
- Write schemas/linear/schema.meta.json (commit hash, date, permalink)
- cargo xtask codegen
- Parse SDL
- Generate:
- src/generated/resources.rs (Resource enum; clap ValueEnum)
- src/generated/registry.rs (query capabilities, presets, expansions)
- src/generated/mutation_ops.rs (MutationOp enum; clap ValueEnum)
- src/generated/mutation_registry.rs (op arg metadata, helper mappings)
- src/generated/search_plan.rs

CI enforcement:
- Running schema sync + codegen must produce no git diff.
- Snapshot tests fail on drift; maintainers update snapshots and fix builders accordingly.

⸻

7) Mutation coverage (v1)

Principle

If it exists under GraphQL Mutation root in the schema, it must be invocable via:
- linears mutate <op> ... (universal)
And optionally via a convenience helper when it matches a known pattern.

Helper mapping rules (compile-time heuristics)

Generate helper support when schema contains recognizable operations:
- create <resource> when a mutation exists with name create<Resource> or createX returning that type (or the schema’s canonical create op for that entity).
- update <resource> when a mutation exists that takes an input containing id and updates that resource.
- archive/unarchive/delete similarly, when an op exists and input contains id.

If ambiguous or multiple candidates exist:
- Do not generate helper for that resource; rely on mutate.
- ops output should show the canonical op(s) and expected variables.

Mutation result selection

Mutations must request a minimal useful selection set:
- Prefer fields in the returned payload that include:
- success, lastSyncId, entity/<resource> node, id, updatedAt
- Generated per op based on payload type fields (bounded; no deep nesting by default).
- Allow --select/--expand for mutation results only if safely supported; otherwise raw is the escape hatch.

⸻

8) Auth & configuration

Auth (strict)
- Required: LINEAR_API_KEY
- No CLI option to provide the token
- No OAuth flows

Configuration env vars
- LINEAR_API_KEY (required for any network call)
- LINEAR_ENDPOINT (optional; overrides default endpoint)
- LINEARS_OUTPUT (optional; sets default output format: json|yaml|table|text|ndjson)
- LINEARS_WORKSPACE (required when API key has multi-workspace access; workspace slug or ID)

Endpoint
- Default: https://api.linear.app/graphql
- Allow override by --endpoint or env var LINEAR_ENDPOINT

⸻

9) Implementation architecture (Rust)

Suggested crates
- CLI: clap
- HTTP/async: reqwest, tokio
- Serialization: serde, serde_json, serde_yaml
- Errors: thiserror (core), anyhow (CLI boundary)
- SDL parser: graphql-parser (or equivalent)
- Table rendering: tabled (or equivalent)

Modules
- generated/* (codegen output; never hand-edited)
- query_builder (list/get/search; presets/expand; variables)
- mutation_builder (mutate + helpers; result selection)
- client (HTTP; retries; backoff; error mapping)
  - 429: retry after Retry-After header (if ≤60s), else show reset time and exit
  - 5xx: max 10 retries with exponential backoff + jitter
- render (json/yaml/ndjson/table/text)
- validate (minimal schema-guided key validation for filter/vars; strict mode optional)
- xtask/ (schema sync + codegen)

Design constraints:
- Keep dynamic JSON/YAML at boundaries; internal registry is typed and generated.
- Never interpolate user values into GraphQL documents; always use variables.

Exit codes:
- 0: success
- 1: general/unknown error
- 2: auth error (missing/invalid LINEAR_API_KEY)
- 3: network error (connection failed, timeout)
- 4: GraphQL error (valid request, Linear returned errors)

Progress indicators:
- Show spinner/progress to stderr only when attached to TTY
- Silent when piped or redirected

Stdin support:
- --filter -, --input -, --set -, --vars - all accept stdin
- Enables piping: echo '{"title":"New issue"}' | linears create issue --input -

⸻

10) TDD plan (mandatory)

Unit + snapshot tests
- registry_snapshot: resources and capabilities derived from schema
- mutation_ops_snapshot: list of mutation ops + arg metadata (as recorded by codegen)
- query_snapshot_all_resources:
- For each listable resource: generated query text + variables skeleton
- search_plan_snapshot: per-resource plan and produced filter structure
- mutation_snapshot_selected_ops:
- For representative ops: generated mutation document + variables skeleton + selection set

Integration tests (offline)
- Mock GraphQL server validates:
- Authorization header uses LINEAR_API_KEY value
- request body matches expected document + variables
- retry/backoff behavior on 429/5xx
- output formatting stable for all formats

Optional live smoke tests (off by default)
- Enabled by env var; requires LINEAR_API_KEY.

⸻

11) Acceptance criteria (v1)
- linears resources lists all generated query resources.
- linears ops lists all generated mutation ops.
- For every query resource:
- list/get/search is available when supported by schema shape; returns structured error when not.
- Filtering/sorting/pagination flags appear only when supported.
- For every mutation op:
- linears mutate <op> can execute with provided variables (JSON/YAML).
- For common CRUD-ish entities where mapping is unambiguous:
- create/update/archive/unarchive/delete helpers work and are tested.
- Default output is table; JSON/YAML/text/NDJSON work consistently.
- LINEARS_OUTPUT env var correctly overrides default output format.
- Schema updates produce deterministic diffs and failing tests guide required updates.

⸻

12) Future plan — saved queries (expanded one level)

Goal

Make repeated workflows reproducible and shareable without dropping into raw.

Proposed v2 commands
- linears query save <name> --query <FILE|STRING> [--vars <FILE>] [--var k=v...] [--note "..."]
- linears query run <name> [--vars <FILE>] [--var k=v...] [--out …]
- linears query list
- linears query show <name>
- linears query rm <name>

Storage
- Save under ~/.config/linears/queries/:
- <name>.graphql
- <name>.vars.json|yaml (optional defaults)
- <name>.meta.json (notes, createdAt, lastRunAt)

Behavior
- run merges defaults with overrides (--vars-file and --var).
- Outputs same envelope rules as raw, enabling pipelines and LLM usage.

⸻

13) Future plan — shell completions

- linears completions <shell> (bash|zsh|fish|powershell)
- Output completion script to stdout for user to source/install
- Generated via clap_complete

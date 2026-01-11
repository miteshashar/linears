#!/usr/bin/env bash
# linears - Development Environment Setup Script
# A Rust CLI for Linear's GraphQL API

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check for required tools
check_requirements() {
    log_info "Checking requirements..."

    # Check Rust toolchain
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo not found. Install from https://rustup.rs/"
        exit 1
    fi

    RUST_VERSION=$(rustc --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    log_success "Rust found: $(rustc --version)"

    # Check minimum version (1.75+)
    MAJOR=$(echo "$RUST_VERSION" | cut -d. -f1)
    MINOR=$(echo "$RUST_VERSION" | cut -d. -f2)
    if [[ "$MAJOR" -lt 1 ]] || [[ "$MAJOR" -eq 1 && "$MINOR" -lt 75 ]]; then
        log_warn "Rust 1.75+ recommended, found $RUST_VERSION"
    fi

    # Check for optional tools
    if command -v rustfmt &> /dev/null; then
        log_success "rustfmt found"
    else
        log_warn "rustfmt not found - install with: rustup component add rustfmt"
    fi

    if command -v clippy-driver &> /dev/null; then
        log_success "clippy found"
    else
        log_warn "clippy not found - install with: rustup component add clippy"
    fi
}

# Check environment variables
check_env() {
    log_info "Checking environment variables..."

    if [[ -z "${LINEARS_API_KEY:-}" ]]; then
        log_warn "LINEARS_API_KEY not set - required for API calls"
        log_info "  Get your key from: https://linear.app/settings/api"
        log_info "  Set with: export LINEARS_API_KEY='lin_api_...'"
    else
        log_success "LINEARS_API_KEY is set"
    fi

    if [[ -n "${LINEARS_ENDPOINT:-}" ]]; then
        log_info "LINEARS_ENDPOINT override: $LINEARS_ENDPOINT"
    fi

    if [[ -n "${LINEARS_OUTPUT:-}" ]]; then
        log_info "LINEARS_OUTPUT default: $LINEARS_OUTPUT"
    fi

    if [[ -n "${LINEARS_WORKSPACE:-}" ]]; then
        log_info "LINEARS_WORKSPACE: $LINEARS_WORKSPACE"
    fi
}

# Build the project
build_project() {
    log_info "Building project..."

    # Install dependencies and build
    if cargo build; then
        log_success "Build successful"
    else
        log_error "Build failed"
        exit 1
    fi
}

# Build in release mode
build_release() {
    log_info "Building release version..."

    if cargo build --release; then
        log_success "Release build successful"
        log_info "Binary at: ./target/release/linears"
    else
        log_error "Release build failed"
        exit 1
    fi
}

# Run tests
run_tests() {
    log_info "Running tests..."

    # INSTA_UPDATE=always auto-accepts new/changed snapshots
    if INSTA_UPDATE=always cargo test; then
        log_success "All tests passed"
    else
        log_error "Tests failed"
        exit 1
    fi
}

# Run linters
run_lint() {
    log_info "Running linters..."

    if cargo fmt -- --check; then
        log_success "Code formatting OK"
    else
        log_warn "Formatting issues found. Run: cargo fmt"
    fi

    if cargo clippy -- -D warnings; then
        log_success "Clippy checks passed"
    else
        log_warn "Clippy warnings found"
    fi
}

# Sync schema from Linear SDK
sync_schema() {
    log_info "Syncing GraphQL schema from Linear SDK..."

    if cargo run -p xtask -- schema sync; then
        log_success "Schema synced"
    else
        log_error "Schema sync failed"
        exit 1
    fi
}

# Run code generation
run_codegen() {
    log_info "Running code generation..."

    if cargo run -p xtask -- codegen; then
        log_success "Code generation complete"
    else
        log_error "Code generation failed"
        exit 1
    fi
}

# Install the CLI globally
install_cli() {
    log_info "Installing linears CLI..."

    if cargo install --path .; then
        log_success "Installed! Run 'linears --help' to get started"
    else
        log_error "Installation failed"
        exit 1
    fi
}

# Show help
show_help() {
    echo "linears Development Environment Setup"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  (no args)     Full setup: check requirements, build, test"
    echo "  check         Check requirements and environment only"
    echo "  build         Build debug version"
    echo "  release       Build release version"
    echo "  test          Run tests"
    echo "  lint          Run formatters and linters"
    echo "  schema        Sync GraphQL schema from Linear"
    echo "  codegen       Run code generation"
    echo "  install       Install CLI globally"
    echo "  help          Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  LINEARS_API_KEY     Required for API calls (get from https://linear.app/settings/api)"
    echo "  LINEARS_ENDPOINT    Override API endpoint (default: https://api.linear.app/graphql)"
    echo "  LINEARS_OUTPUT     Default output format: json|yaml|table|text|ndjson"
    echo "  LINEARS_WORKSPACE  Workspace slug/ID for multi-workspace API keys"
}

# Main entry point
main() {
    cd "$(dirname "$0")"

    case "${1:-}" in
        check)
            check_requirements
            check_env
            ;;
        build)
            build_project
            ;;
        release)
            build_release
            ;;
        test)
            run_tests
            ;;
        lint)
            run_lint
            ;;
        schema)
            sync_schema
            ;;
        codegen)
            run_codegen
            ;;
        install)
            install_cli
            ;;
        help|--help|-h)
            show_help
            ;;
        "")
            # Default: full setup
            echo "========================================"
            echo "linears - Development Environment Setup"
            echo "========================================"
            echo ""
            check_requirements
            echo ""
            check_env
            echo ""
            build_project
            echo ""
            run_tests
            echo ""
            log_success "Setup complete!"
            echo ""
            echo "Next steps:"
            echo "  1. Set LINEARS_API_KEY if not already set"
            echo "  2. Run 'cargo run -- --help' to see available commands"
            echo "  3. Run 'cargo run -- resources' to see available resources"
            echo "  4. Run 'cargo run -- list issue' to list issues"
            echo ""
            ;;
        *)
            log_error "Unknown command: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"

# Mina Web Wallet - Build Automation
# ===================================

# Configuration
WASM_PACK_VERSION := 0.13.1
SHELL := /bin/bash

.PHONY: all install build test format lint clean serve help

# Default target
all: build

# =============================================================================
# INSTALLATION
# =============================================================================

## Install all dependencies
install: install-wasm-pack install-npm
	@echo "All dependencies installed successfully"

## Install wasm-pack
install-wasm-pack:
	@if ! command -v wasm-pack &> /dev/null || [ "$$(wasm-pack --version | cut -d' ' -f2)" != "$(WASM_PACK_VERSION)" ]; then \
		echo "Installing wasm-pack $(WASM_PACK_VERSION)..."; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -y; \
	else \
		echo "wasm-pack $(WASM_PACK_VERSION) already installed"; \
	fi

## Install npm dependencies (for development tools)
install-npm:
	@if [ -f package.json ]; then \
		npm install; \
	else \
		echo "No package.json found, skipping npm install"; \
	fi

# =============================================================================
# BUILDING
# =============================================================================

## Build everything
build: build-wasm build-cli
	@echo "Build complete!"

## Build WebAssembly module
build-wasm:
	@echo "Building WASM module..."
	cd wasm-module && wasm-pack build --target web --out-dir ../frontend/pkg
	@echo "WASM module built successfully"

## Build CLI tool
build-cli:
	@echo "Building CLI..."
	cargo build --release -p mina-wallet-cli
	@echo "CLI built at target/release/mina-wallet"

## Build for production (optimized)
build-prod: build-wasm-prod build-cli
	@echo "Production build complete!"

## Build optimized WASM module
build-wasm-prod:
	@echo "Building optimized WASM module..."
	cd wasm-module && wasm-pack build --target web --release --out-dir ../frontend/pkg
	@echo "Optimized WASM module built successfully"

# =============================================================================
# DEVELOPMENT
# =============================================================================

## Start development server
serve: build-wasm
	@echo "Starting development server at http://localhost:3000"
	@cd frontend && python3 -m http.server 3000 || python -m SimpleHTTPServer 3000

## Watch for changes and rebuild (requires cargo-watch)
watch:
	@echo "Watching for changes..."
	cargo watch -x "build -p mina-web-wallet-core" -s "make build-wasm"

# =============================================================================
# FORMATTING
# =============================================================================

## Format all code
format: format-rust format-toml
	@echo "All code formatted"

## Format Rust code
format-rust:
	@echo "Formatting Rust code..."
	cargo fmt --all

## Format TOML files
format-toml:
	@echo "Formatting TOML files..."
	@if command -v taplo &> /dev/null; then \
		taplo format; \
	else \
		echo "taplo not installed, skipping TOML formatting"; \
	fi

# =============================================================================
# FORMAT CHECKING (for CI)
# =============================================================================

## Check formatting without modifying files
check-format: check-format-rust check-format-toml
	@echo "Format check passed"

## Check Rust formatting
check-format-rust:
	@echo "Checking Rust formatting..."
	cargo fmt --all -- --check

## Check TOML formatting
check-format-toml:
	@echo "Checking TOML formatting..."
	@if command -v taplo &> /dev/null; then \
		taplo format --check; \
	else \
		echo "taplo not installed, skipping TOML format check"; \
	fi

# =============================================================================
# LINTING
# =============================================================================

## Run all linters
lint: lint-rust lint-clippy
	@echo "Linting complete"

## Run Rust linter (clippy)
lint-rust lint-clippy:
	@echo "Running clippy..."
	cargo clippy --all-targets --all-features -- -D warnings

## Run shellcheck on shell scripts
lint-shell:
	@echo "Checking shell scripts..."
	@if command -v shellcheck &> /dev/null; then \
		find . -name "*.sh" -exec shellcheck {} \; ; \
	else \
		echo "shellcheck not installed, skipping"; \
	fi

# =============================================================================
# TESTING
# =============================================================================

## Run all tests
test: test-rust test-wasm
	@echo "All tests passed!"

## Run Rust unit tests
test-rust:
	@echo "Running Rust tests..."
	cargo test --all

## Run WASM tests in Node.js
test-wasm:
	@echo "Running WASM tests..."
	cd wasm-module && wasm-pack test --node

## Run WASM tests in headless browser
test-wasm-browser:
	@echo "Running WASM tests in browser..."
	cd wasm-module && wasm-pack test --headless --chrome

## Run CLI end-to-end tests
test-cli: build-cli
	@echo "Running CLI tests..."
	./target/release/mina-wallet generate --format json > /dev/null
	@echo "CLI tests passed"

# =============================================================================
# CLEANING
# =============================================================================

## Clean all build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf frontend/pkg
	rm -rf wasm-module/pkg
	rm -rf node_modules
	@echo "Clean complete"

## Clean only WASM artifacts
clean-wasm:
	rm -rf frontend/pkg
	rm -rf wasm-module/pkg

# =============================================================================
# DOCUMENTATION
# =============================================================================

## Generate Rust documentation
docs:
	cargo doc --no-deps --open

# =============================================================================
# HELP
# =============================================================================

## Display this help message
help:
	@echo "Mina Web Wallet - Available Commands"
	@echo "====================================="
	@echo ""
	@echo "Installation:"
	@echo "  make install          Install all dependencies"
	@echo "  make install-wasm-pack Install wasm-pack"
	@echo ""
	@echo "Building:"
	@echo "  make build            Build everything (WASM + CLI)"
	@echo "  make build-wasm       Build WebAssembly module"
	@echo "  make build-cli        Build CLI tool"
	@echo "  make build-prod       Build for production"
	@echo ""
	@echo "Development:"
	@echo "  make serve            Start dev server at localhost:3000"
	@echo "  make watch            Watch and rebuild on changes"
	@echo ""
	@echo "Code Quality:"
	@echo "  make format           Format all code"
	@echo "  make check-format     Check formatting (for CI)"
	@echo "  make lint             Run all linters"
	@echo ""
	@echo "Testing:"
	@echo "  make test             Run all tests"
	@echo "  make test-rust        Run Rust unit tests"
	@echo "  make test-wasm        Run WASM tests"
	@echo ""
	@echo "Other:"
	@echo "  make clean            Clean build artifacts"
	@echo "  make docs             Generate documentation"
	@echo "  make help             Show this help message"

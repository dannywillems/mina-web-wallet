# CLAUDE.md

This file provides guidance to Claude Code (or any AI assistant) when working with this repository.

## Project Overview

This is **Mina Web Wallet** - a privacy-preserving Mina wallet toolkit that runs entirely in your browser. All cryptographic operations (key generation, signing) happen locally using WebAssembly compiled from Rust.

**Critical Security Property**: Private keys never leave the browser. All wallet operations are performed client-side.

## Essential Commands

All commands are run through Make. Do not use `npm run` or `cargo` commands directly for standard workflows.

```bash
# First-time setup
make install

# Build everything (WASM + CLI)
make build

# Run tests
make test

# Format code
make format

# Check formatting (CI)
make check-format

# Lint code
make lint

# Start development server
make serve

# Clean build artifacts
make clean
```

## Architecture

### Directory Structure

```
mina-web-wallet/
├── core/           # Shared Rust library for wallet operations
│   └── src/
│       ├── lib.rs      # Re-exports from mina-signer
│       └── wallet.rs   # High-level wallet abstraction
├── wasm-module/    # WebAssembly module for browser
│   └── src/
│       └── lib.rs      # WASM bindings via wasm-bindgen
├── cli/            # Command-line wallet tool
│   └── src/
│       └── main.rs     # CLI using clap
├── frontend/       # Web interface
│   ├── index.html      # Main HTML page
│   ├── app.js          # JavaScript application
│   └── favicon.svg     # Site icon
├── .github/
│   └── workflows/      # CI/CD pipelines
├── Cargo.toml          # Workspace configuration
├── Makefile            # Build automation
└── rust-toolchain.toml # Rust nightly configuration
```

### Key Dependencies

This project uses crates from [o1-labs/proof-systems](https://github.com/o1-labs/proof-systems):

- `mina-signer` - Key generation, signing, address encoding
- `mina-curves` - Pallas/Vesta curve implementations
- `mina-hasher` - Poseidon hash function

### Technology Stack

- **Rust** - Core cryptographic operations
- **wasm-pack** - Compiles Rust to WebAssembly
- **Bootstrap 5** - Frontend styling (CDN, no build step)
- **Vanilla JavaScript** - No frontend framework

## Coding Guidelines

### General

1. **Use Makefile exclusively** - All build/test/format commands go through Make
2. **No emojis in code** - Keep source files professional
3. **Rust nightly toolchain** - Required for some dependencies
4. **Edition 2024** - Use latest Rust edition features

### Rust Code

1. **Follow rustfmt** - Run `make format` before committing
2. **Use clippy** - Run `make lint` to catch common issues
3. **Error handling** - Use `thiserror` for custom error types
4. **Documentation** - Add `///` doc comments to public APIs

### JavaScript Code

1. **ES modules** - Use `import`/`export` syntax
2. **Async/await** - Prefer over raw Promises
3. **No external dependencies** - Keep frontend minimal

### Frontend

1. **Bootstrap via CDN** - No local CSS framework files
2. **Single HTML file** - Keep structure simple
3. **Responsive design** - Mobile-first approach

## Pre-Commit Checklist

Before committing changes, always run:

```bash
make test       # Ensure all tests pass
make format     # Format Rust, TOML, JS code
make lint       # Check for common issues
```

## Commit Message Format

Use conventional commit prefixes:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `refactor:` - Code change that neither fixes a bug nor adds a feature
- `test:` - Adding or modifying tests
- `chore:` - Build process or auxiliary tool changes

Rules:
- No emojis in commit messages
- Wrap title at 72 characters
- Wrap body at 80 characters
- Do not add Claude as co-author

Example: `feat: add transaction signing support`

## Testing

### Unit Tests

```bash
make test-rust    # Run Rust unit tests
make test-wasm    # Run WASM tests in headless browser
```

### Manual Testing

```bash
make serve        # Start local dev server at localhost:3000
```

## Security Considerations

1. **Never log private keys** - Even in debug builds
2. **Clear sensitive data** - Zero out memory after use when possible
3. **Validate all inputs** - Especially from user or network
4. **Use constant-time comparisons** - For cryptographic values

## Troubleshooting

### WASM Build Fails

Ensure you have:
- Rust nightly toolchain installed
- `wasm32-unknown-unknown` target added
- `wasm-pack` installed (`make install-wasm-pack`)

### Tests Fail in Browser

Make sure you're using a modern browser with WebAssembly support.

### Cargo Dependency Issues

Try cleaning and rebuilding:
```bash
make clean
make install
make build
```

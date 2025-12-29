# Mina Web Wallet

A privacy-preserving Mina wallet toolkit that runs entirely in your browser. All cryptographic operations happen locally using WebAssembly - your private keys never leave your device.

## Features

- **Generate Wallets**: Create new Mina wallets with secure random key generation
- **Import Wallets**: Restore wallets from secret keys (hex or Base58 format)
- **Address Validation**: Verify Mina addresses are valid
- **Local-First Security**: All operations run in your browser via WebAssembly
- **CLI Tool**: Command-line interface for wallet management

## Quick Start

### Prerequisites

- Rust nightly toolchain
- wasm-pack
- Make

### Installation

```bash
# Clone the repository
git clone https://github.com/dannywillems/mina-web-wallet
cd mina-web-wallet

# Install dependencies
make install

# Build everything
make build

# Start development server
make serve
```

Then open http://localhost:3000 in your browser.

### CLI Usage

```bash
# Generate a new wallet
./target/release/mina-wallet generate

# Generate wallet for testnet
./target/release/mina-wallet generate --network testnet

# Import from secret key
./target/release/mina-wallet import <secret-key>

# Validate an address
./target/release/mina-wallet validate B62q...
```

## Project Structure

```
mina-web-wallet/
├── core/           # Shared Rust library
├── wasm-module/    # WebAssembly module for browser
├── cli/            # Command-line tool
├── frontend/       # Web interface
└── .github/        # CI/CD workflows
```

## Development

```bash
# Format code
make format

# Run linter
make lint

# Run tests
make test

# Clean build artifacts
make clean
```

## Security

This wallet prioritizes security:

- **Private keys never leave the browser** - All cryptographic operations are performed locally
- **No server-side storage** - Your wallet data is only stored in your browser
- **Open source** - Review the code yourself

**Warning**: Always backup your secret keys securely. If lost, they cannot be recovered.

## Built With

- [o1-labs/proof-systems](https://github.com/o1-labs/proof-systems) - Mina cryptographic primitives
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - Rust to WebAssembly compiler
- [Bootstrap 5](https://getbootstrap.com/) - Frontend styling

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Inspired by the [Zcash Web Wallet](https://github.com/dannywillems/zcash-web-wallet) project.

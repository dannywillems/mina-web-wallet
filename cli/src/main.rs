//! Mina Wallet CLI
//!
//! Command-line interface for Mina wallet operations including:
//! - Generating new wallets
//! - Importing existing wallets
//! - Displaying wallet information

use clap::{Parser, Subcommand};
use mina_signer::NetworkId;
use mina_web_wallet_core::Wallet;

#[derive(Parser)]
#[command(name = "mina-wallet")]
#[command(author, version, about = "Mina wallet CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new random wallet
    Generate {
        /// Network: mainnet or testnet
        #[arg(short, long, default_value = "mainnet")]
        network: String,

        /// Output format: text or json
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Import a wallet from a secret key
    Import {
        /// Secret key in hex or base58 format
        secret_key: String,

        /// Network: mainnet or testnet
        #[arg(short, long, default_value = "mainnet")]
        network: String,

        /// Output format: text or json
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Validate a Mina address
    Validate {
        /// The Mina address to validate
        address: String,
    },

    /// Get address from a secret key (without showing the secret)
    Address {
        /// Secret key in hex or base58 format
        secret_key: String,
    },
}

fn parse_network(network: &str) -> Result<NetworkId, String> {
    match network.to_lowercase().as_str() {
        "mainnet" => Ok(NetworkId::MAINNET),
        "testnet" => Ok(NetworkId::TESTNET),
        _ => Err(format!(
            "Invalid network '{}'. Use 'mainnet' or 'testnet'.",
            network
        )),
    }
}

fn import_wallet(secret_key: &str, network: NetworkId) -> Result<Wallet, String> {
    // Try hex format first
    if let Ok(wallet) = Wallet::from_secret_key_hex(secret_key, network.clone()) {
        return Ok(wallet);
    }

    // Try base58 format
    if let Ok(wallet) = Wallet::from_secret_key_base58(secret_key, network) {
        return Ok(wallet);
    }

    Err("Invalid secret key format. Expected hex (64 chars) or base58 (52 chars).".to_string())
}

fn print_wallet_text(wallet: &Wallet) {
    println!("Wallet Generated Successfully!");
    println!("==============================");
    println!("Address:          {}", wallet.address());
    println!("Secret Key (Hex): {}", wallet.secret_key_hex());
    println!("Secret Key (B58): {}", wallet.secret_key_base58());
    println!("Network:          {:?}", wallet.network());
    println!();
    println!(
        "WARNING: Store your secret key securely! Anyone with access to it can control your funds."
    );
}

fn print_wallet_json(wallet: &Wallet) {
    let json = serde_json::json!({
        "address": wallet.address(),
        "secret_key_hex": wallet.secret_key_hex(),
        "secret_key_base58": wallet.secret_key_base58(),
        "network": format!("{:?}", wallet.network()).to_lowercase(),
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { network, format } => {
            let network_id = match parse_network(&network) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            let wallet = match Wallet::new(network_id) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error generating wallet: {}", e);
                    std::process::exit(1);
                }
            };

            match format.as_str() {
                "json" => print_wallet_json(&wallet),
                _ => print_wallet_text(&wallet),
            }
        }

        Commands::Import {
            secret_key,
            network,
            format,
        } => {
            let network_id = match parse_network(&network) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            match import_wallet(&secret_key, network_id) {
                Ok(wallet) => match format.as_str() {
                    "json" => print_wallet_json(&wallet),
                    _ => print_wallet_text(&wallet),
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Validate { address } => match mina_web_wallet_core::address_to_pubkey(&address) {
            Ok(_) => {
                println!("Address is valid: {}", address);
            }
            Err(e) => {
                eprintln!("Invalid address: {:?}", e);
                std::process::exit(1);
            }
        },

        Commands::Address { secret_key } => {
            // Default to mainnet for address derivation
            match import_wallet(&secret_key, NetworkId::MAINNET) {
                Ok(wallet) => {
                    println!("{}", wallet.address());
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

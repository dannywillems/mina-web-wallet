//! Wallet management for Mina
//!
//! This module provides high-level wallet operations including:
//! - Creating new wallets
//! - Importing existing wallets from secret keys
//! - Signing messages and transactions

use mina_signer::{Keypair, NetworkId, PubKey, SecKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors that can occur during wallet operations
#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Invalid secret key: {0}")]
    InvalidSecretKey(String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Keypair generation failed: {0}")]
    KeypairGenerationFailed(String),
}

pub type Result<T> = std::result::Result<T, WalletError>;

/// A Mina wallet containing a keypair and associated metadata
#[derive(Clone)]
pub struct Wallet {
    /// The keypair (secret + public key)
    keypair: Keypair,
    /// The network (mainnet or testnet)
    network: NetworkId,
}

impl Wallet {
    /// Create a new random wallet
    pub fn new(network: NetworkId) -> Result<Self> {
        let keypair = Keypair::rand(&mut rand::rngs::OsRng)
            .map_err(|e| WalletError::KeypairGenerationFailed(format!("{:?}", e)))?;
        Ok(Self { keypair, network })
    }

    /// Create a wallet from an existing secret key (hex format)
    pub fn from_secret_key_hex(secret_hex: &str, network: NetworkId) -> Result<Self> {
        let secret = SecKey::from_hex(secret_hex)
            .map_err(|e| WalletError::InvalidSecretKey(format!("{:?}", e)))?;
        let keypair = Keypair::from_secret_key(secret)
            .map_err(|e| WalletError::InvalidSecretKey(format!("{:?}", e)))?;
        Ok(Self { keypair, network })
    }

    /// Create a wallet from an existing secret key (Base58 format)
    pub fn from_secret_key_base58(secret_b58: &str, network: NetworkId) -> Result<Self> {
        let secret = SecKey::from_base58(secret_b58)
            .map_err(|e| WalletError::InvalidSecretKey(format!("{:?}", e)))?;
        let keypair = Keypair::from_secret_key(secret)
            .map_err(|e| WalletError::InvalidSecretKey(format!("{:?}", e)))?;
        Ok(Self { keypair, network })
    }

    /// Get the public key
    pub fn public_key(&self) -> &PubKey {
        &self.keypair.public
    }

    /// Get the Mina address
    pub fn address(&self) -> String {
        self.keypair.public.into_address()
    }

    /// Get the secret key in hex format
    pub fn secret_key_hex(&self) -> String {
        self.keypair.secret.to_hex()
    }

    /// Get the secret key in Base58 format
    pub fn secret_key_base58(&self) -> String {
        self.keypair.secret.to_base58()
    }

    /// Get the network
    pub fn network(&self) -> &NetworkId {
        &self.network
    }

    /// Get the underlying keypair
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }
}

/// Wallet information that can be safely serialized (no secret key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub network: String,
}

impl From<&Wallet> for WalletInfo {
    fn from(wallet: &Wallet) -> Self {
        Self {
            address: wallet.address(),
            network: match wallet.network() {
                NetworkId::MAINNET => "mainnet".to_string(),
                NetworkId::TESTNET => "testnet".to_string(),
            },
        }
    }
}

impl std::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't expose secret key in debug output
        f.debug_struct("Wallet")
            .field("address", &self.address())
            .field("network", self.network())
            .finish()
    }
}

impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new(NetworkId::MAINNET).expect("Failed to create wallet");
        let address = wallet.address();
        assert!(address.starts_with("B62q"));
    }

    #[test]
    fn test_wallet_from_secret_key() {
        // Create a wallet and export the secret key
        let wallet1 = Wallet::new(NetworkId::MAINNET).expect("Failed to create wallet");
        let secret_hex = wallet1.secret_key_hex();
        let secret_b58 = wallet1.secret_key_base58();

        // Import the wallet from hex
        let wallet2 = Wallet::from_secret_key_hex(&secret_hex, NetworkId::MAINNET).unwrap();
        assert_eq!(wallet1.address(), wallet2.address());

        // Import the wallet from base58
        let wallet3 = Wallet::from_secret_key_base58(&secret_b58, NetworkId::MAINNET).unwrap();
        assert_eq!(wallet1.address(), wallet3.address());
    }

    #[test]
    fn test_wallet_info() {
        let wallet = Wallet::new(NetworkId::TESTNET).expect("Failed to create wallet");
        let info: WalletInfo = (&wallet).into();
        assert_eq!(info.address, wallet.address());
        assert_eq!(info.network, "testnet");
    }
}

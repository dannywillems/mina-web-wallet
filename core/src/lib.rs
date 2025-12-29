//! Mina Web Wallet Core Library
//!
//! This library provides the core cryptographic functionality for the Mina web wallet,
//! built on top of o1-labs/proof-systems crates:
//! - Key generation and management
//! - Schnorr signatures
//! - Address encoding/decoding
//! - Transaction signing

pub mod wallet;

// Re-export types from mina-signer for convenience
pub use mina_signer::keypair::KeypairError;
pub use mina_signer::pubkey::PubKeyError;
pub use mina_signer::{CompressedPubKey, Keypair, NetworkId, PubKey, SecKey, Signature};

// Re-export our wallet functionality
pub use wallet::{Wallet, WalletError, WalletInfo};

/// Field types from mina-curves
pub mod fields {
    pub use mina_curves::pasta::{Fp, Fq};
}

/// Create a new random keypair
pub fn generate_keypair() -> Result<Keypair, KeypairError> {
    Keypair::rand(&mut rand::rngs::OsRng)
}

/// Get the Mina address for a keypair
pub fn get_address(keypair: &Keypair) -> String {
    keypair.public.into_address()
}

/// Get the Mina address for a public key
pub fn pubkey_to_address(pubkey: &PubKey) -> String {
    pubkey.into_address()
}

/// Parse a Mina address and return the compressed public key
pub fn address_to_pubkey(address: &str) -> Result<CompressedPubKey, PubKeyError> {
    CompressedPubKey::from_address(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = generate_keypair().expect("Failed to generate keypair");
        let address = get_address(&keypair);

        // Mina addresses start with B62q
        assert!(address.starts_with("B62q"));
    }

    #[test]
    fn test_address_roundtrip() {
        let keypair = generate_keypair().expect("Failed to generate keypair");
        let address = get_address(&keypair);

        // Parse the address back to compressed pubkey
        let pubkey = address_to_pubkey(&address).unwrap();

        // Convert back to address - should match
        let address2 = pubkey.into_address();
        assert_eq!(address, address2);
    }
}

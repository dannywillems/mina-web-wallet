//! Mina Web Wallet - WebAssembly Module
//!
//! This module exposes the Mina wallet functionality to JavaScript/TypeScript
//! through WebAssembly bindings.

use mina_signer::NetworkId;
use mina_web_wallet_core::Wallet;
use o1_utils::field_helpers::FieldHelpers;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Result type for WASM operations
#[derive(Serialize, Deserialize)]
pub struct WasmResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> WasmResult<T> {
    fn ok(data: T) -> JsValue {
        let result = Self {
            success: true,
            data: Some(data),
            error: None,
        };
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }

    fn err(error: String) -> JsValue {
        let result: WasmResult<()> = WasmResult {
            success: false,
            data: None,
            error: Some(error),
        };
        serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
    }
}

/// Wallet data that can be exported to JavaScript
#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub address: String,
    pub secret_key_hex: String,
    pub secret_key_base58: String,
    pub network: String,
}

/// Generate a new random wallet
///
/// # Arguments
/// * `network` - Either "mainnet" or "testnet"
///
/// # Returns
/// JSON object with wallet data including address and secret keys
#[wasm_bindgen]
pub fn generate_wallet(network: &str) -> JsValue {
    let network_id = match network.to_lowercase().as_str() {
        "mainnet" => NetworkId::MAINNET,
        "testnet" => NetworkId::TESTNET,
        _ => {
            return WasmResult::<WalletData>::err(
                "Invalid network. Use 'mainnet' or 'testnet'.".to_string(),
            );
        }
    };

    match Wallet::new(network_id) {
        Ok(wallet) => {
            let data = WalletData {
                address: wallet.address(),
                secret_key_hex: wallet.secret_key_hex(),
                secret_key_base58: wallet.secret_key_base58(),
                network: network.to_lowercase(),
            };
            WasmResult::ok(data)
        }
        Err(e) => WasmResult::<WalletData>::err(format!("Failed to generate wallet: {}", e)),
    }
}

/// Import a wallet from a secret key in hex format
///
/// # Arguments
/// * `secret_hex` - The secret key in hexadecimal format (64 characters)
/// * `network` - Either "mainnet" or "testnet"
///
/// # Returns
/// JSON object with wallet data
#[wasm_bindgen]
pub fn import_wallet_from_hex(secret_hex: &str, network: &str) -> JsValue {
    let network_id = match network.to_lowercase().as_str() {
        "mainnet" => NetworkId::MAINNET,
        "testnet" => NetworkId::TESTNET,
        _ => {
            return WasmResult::<WalletData>::err(
                "Invalid network. Use 'mainnet' or 'testnet'.".to_string(),
            );
        }
    };

    match Wallet::from_secret_key_hex(secret_hex, network_id) {
        Ok(wallet) => {
            let data = WalletData {
                address: wallet.address(),
                secret_key_hex: wallet.secret_key_hex(),
                secret_key_base58: wallet.secret_key_base58(),
                network: network.to_lowercase(),
            };
            WasmResult::ok(data)
        }
        Err(e) => WasmResult::<WalletData>::err(format!("Failed to import wallet: {}", e)),
    }
}

/// Import a wallet from a secret key in Base58 format
///
/// # Arguments
/// * `secret_base58` - The secret key in Base58 format (52 characters)
/// * `network` - Either "mainnet" or "testnet"
///
/// # Returns
/// JSON object with wallet data
#[wasm_bindgen]
pub fn import_wallet_from_base58(secret_base58: &str, network: &str) -> JsValue {
    let network_id = match network.to_lowercase().as_str() {
        "mainnet" => NetworkId::MAINNET,
        "testnet" => NetworkId::TESTNET,
        _ => {
            return WasmResult::<WalletData>::err(
                "Invalid network. Use 'mainnet' or 'testnet'.".to_string(),
            );
        }
    };

    match Wallet::from_secret_key_base58(secret_base58, network_id) {
        Ok(wallet) => {
            let data = WalletData {
                address: wallet.address(),
                secret_key_hex: wallet.secret_key_hex(),
                secret_key_base58: wallet.secret_key_base58(),
                network: network.to_lowercase(),
            };
            WasmResult::ok(data)
        }
        Err(e) => WasmResult::<WalletData>::err(format!("Failed to import wallet: {}", e)),
    }
}

/// Validate a Mina address
///
/// # Arguments
/// * `address` - The Mina address to validate
///
/// # Returns
/// JSON object indicating whether the address is valid
#[wasm_bindgen]
pub fn validate_address(address: &str) -> JsValue {
    #[derive(Serialize)]
    struct ValidationResult {
        valid: bool,
        error: Option<String>,
    }

    match mina_web_wallet_core::address_to_pubkey(address) {
        Ok(_) => WasmResult::ok(ValidationResult {
            valid: true,
            error: None,
        }),
        Err(e) => WasmResult::ok(ValidationResult {
            valid: false,
            error: Some(format!("{:?}", e)),
        }),
    }
}

/// Get the public key components from a Mina address
///
/// # Arguments
/// * `address` - The Mina address
///
/// # Returns
/// JSON object with public key x-coordinate and parity
#[wasm_bindgen]
pub fn address_to_pubkey(address: &str) -> JsValue {
    #[derive(Serialize)]
    struct PubKeyComponents {
        x: String,
        is_odd: bool,
    }

    match mina_web_wallet_core::address_to_pubkey(address) {
        Ok(pubkey) => {
            let x_hex = hex::encode(pubkey.x.to_bytes());
            WasmResult::ok(PubKeyComponents {
                x: x_hex,
                is_odd: pubkey.is_odd,
            })
        }
        Err(e) => WasmResult::<PubKeyComponents>::err(format!("Invalid address: {:?}", e)),
    }
}

/// Get the library version
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_generate_wallet() {
        let result = generate_wallet("mainnet");
        // Result should be a valid JsValue
        assert!(!result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_validate_address() {
        let result = validate_address("B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg");
        assert!(!result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }
}

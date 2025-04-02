use crate::{consts::*, prelude::*, Error};
use chrono::prelude::Utc;
use ethers::core::utils::keccak256;
use lazy_static::lazy_static;
use log::info;
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicU64, Ordering};

fn now_timestamp_ms() -> u64 {
    let now = Utc::now();
    now.timestamp_millis() as u64
}

pub(crate) fn next_nonce() -> u64 {
    let nonce = CUR_NONCE.fetch_add(1, Ordering::Relaxed);
    let now_ms = now_timestamp_ms();
    if nonce > now_ms + 1000 {
        info!("nonce progressed too far ahead {nonce} {now_ms}");
    }
    // more than 300 seconds behind
    if nonce + 300000 < now_ms {
        CUR_NONCE.fetch_max(now_ms, Ordering::Relaxed);
    }
    nonce
}

pub(crate) const WIRE_DECIMALS: u8 = 8;

pub(crate) fn float_to_string_for_hashing(x: f64) -> String {
    let mut x = format!("{:.*}", WIRE_DECIMALS.into(), x);
    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
    }
    if x == "-0" {
        "0".to_string()
    } else {
        x
    }
}

/// Converts a string to a deterministic 128-bit (16 bytes) hex string prefixed with "0x".
/// Uses Keccak256 hashing.
pub(crate) fn string_to_hex_string(input: &str) -> String {
    let hash = keccak256(input.as_bytes());
    // Take the first 16 bytes (128 bits) of the hash
    let truncated_hash = &hash[0..16];
    let hex_string = truncated_hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    format!("0x{}", hex_string)
}

pub(crate) fn generate_random_key() -> Result<[u8; 32]> {
    let mut arr = [0u8; 32];
    thread_rng()
        .try_fill(&mut arr[..])
        .map_err(|e| Error::RandGen(e.to_string()))?;
    Ok(arr)
}

pub fn truncate_float(float: f64, decimals: u32, round_up: bool) -> f64 {
    let pow10 = 10i64.pow(decimals) as f64;
    let mut float = (float * pow10) as u64;
    if round_up {
        float += 1;
    }
    float as f64 / pow10
}

pub fn bps_diff(x: f64, y: f64) -> u16 {
    if x.abs() < EPSILON {
        INF_BPS
    } else {
        (((y - x).abs() / (x)) * 10_000.0) as u16
    }
}

#[derive(Copy, Clone)]
pub enum BaseUrl {
    Localhost,
    Testnet,
    Mainnet,
}

impl BaseUrl {
    pub(crate) fn get_url(&self) -> String {
        match self {
            BaseUrl::Localhost => LOCAL_API_URL.to_string(),
            BaseUrl::Mainnet => MAINNET_API_URL.to_string(),
            BaseUrl::Testnet => TESTNET_API_URL.to_string(),
        }
    }
}

lazy_static! {
    static ref CUR_NONCE: AtomicU64 = AtomicU64::new(now_timestamp_ms());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_to_string_for_hashing_test() {
        assert_eq!(float_to_string_for_hashing(0.), "0".to_string());
        assert_eq!(float_to_string_for_hashing(-0.), "0".to_string());
        assert_eq!(float_to_string_for_hashing(-0.0000), "0".to_string());
        assert_eq!(
            float_to_string_for_hashing(0.00076000),
            "0.00076".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(0.00000001),
            "0.00000001".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(0.12345678),
            "0.12345678".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(87654321.12345678),
            "87654321.12345678".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(987654321.00000000),
            "987654321".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(87654321.1234),
            "87654321.1234".to_string()
        );
        assert_eq!(float_to_string_for_hashing(0.000760), "0.00076".to_string());
        assert_eq!(float_to_string_for_hashing(0.00076), "0.00076".to_string());
        assert_eq!(
            float_to_string_for_hashing(987654321.0),
            "987654321".to_string()
        );
        assert_eq!(
            float_to_string_for_hashing(987654321.),
            "987654321".to_string()
        );
    }

    #[test]
    fn string_to_hex_string_test() {
        // Basic test case
        assert_eq!(
            string_to_hex_string("test"),
            "0x9c22ff5f21f0b81b113e63f7db6da94fedef11b2119b4088b89664fb9a3cb658"[0..34] // 0x + 16*2 hex chars
        );
        // Test with empty string
        assert_eq!(
            string_to_hex_string(""),
            "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"[0..34]
        );
        // Test with a longer string
        assert_eq!(
            string_to_hex_string("a longer test string for hashing"),
            "0xb2ae80682ae56e5e8a930a907beeb1460f768176176b60f677105e9e661e3aaa"[0..34]
        );
        // Test with unicode
        assert_eq!(
            string_to_hex_string("你好世界"), // "Hello World" in Chinese
            "0xd3f05f6be06f8b0a91c1ab0ea49f4554921f35d01eb600778ca36273f6b6d2a8"[0..34]
        );
    }
}

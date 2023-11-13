use dotenv::dotenv;
use hmac::{Hmac, HmacCore};
use sha2::{
    digest::{core_api::CoreWrapper, KeyInit},
    Sha256,
};
use std::env;

use crypto::{sha3::Sha3, digest::Digest};

use jwt::{Header, RegisteredClaims, Token, VerifyWithKey};

pub type HmacSha256 = Hmac<Sha256>;

pub fn get_key() -> Vec<u8> {
    dotenv().ok();
    env::var("SECRETE_KEY")
        .expect("Secret key must be set")
        .chars()
        .map(|ch| ch as u8)
        .collect()
}

pub fn signe_key() -> CoreWrapper<HmacCore<Sha256>> {
    let env_secret_key = get_key();
    HmacSha256::new_from_slice(&env_secret_key[..]).unwrap()
}

pub fn read_token(key: &str) -> Result<String, String> {
    let token = Token::<Header, RegisteredClaims, _>::parse_unverified(key)
        .map_err(|_| "Unable to parse key".to_string())?;

    let secret_key = signe_key();

    match token.verify_with_key(&secret_key) {
        Ok(t) => match t.claims().subject.clone() {
            Some(claim_sub) => Ok(claim_sub),
            None => Err("Claims is not valide".to_string()),
        },
        Err(_) => Err("Token not valide".to_string()),
    }
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}
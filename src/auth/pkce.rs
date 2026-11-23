// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use base64::{
    Engine,
    engine::general_purpose::URL_SAFE_NO_PAD,
    prelude::BASE64_URL_SAFE_NO_PAD,
};
use sha2::{
    Digest,
    Sha256,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Random(#[from] getrandom::Error),
}

pub struct Pkce {
    pub verifier: String,
    #[allow(dead_code)]
    pub challenge: String,
}

impl Pkce {
    pub fn generate() -> Result<Self, Error> {
        let verifier = generate_code_verifier()?;
        let challenge = compute_code_challenge(&verifier);

        Ok(Pkce {
            verifier,
            challenge,
        })
    }
}

fn generate_code_verifier() -> Result<String, Error> {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes)?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(bytes))
}

pub fn compute_code_challenge(code_verifier: &str) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier))
}

pub fn generate_nonce() -> Result<String, Error> {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes)?;

    Ok(URL_SAFE_NO_PAD.encode(bytes))
}

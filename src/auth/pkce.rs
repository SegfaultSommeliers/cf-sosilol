// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use anyhow::Result;
use base64::{
    Engine,
    prelude::BASE64_URL_SAFE_NO_PAD,
};
use sha2::{
    Digest,
    Sha256,
};

pub struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

impl Pkce {
    pub fn generate() -> Result<Self> {
        let verifier = generate_code_verifier()?;
        let challenge = compute_code_challenge(&verifier);

        Ok(Pkce {
            verifier,
            challenge,
        })
    }
}

fn generate_code_verifier() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes)?;
    Ok(BASE64_URL_SAFE_NO_PAD.encode(&bytes))
}

fn compute_code_challenge(code_verifier: &str) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(code_verifier))
}

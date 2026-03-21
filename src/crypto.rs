use age::secrecy::ExposeSecret;
use anyhow::{Context, Result};
use std::io::{Read, Write};

pub fn generate_identity() -> Result<(String, String)> {
    let identity = age::x25519::Identity::generate();
    let public_key = identity.to_public().to_string();

    let mut output = String::new();
    output.push_str(&format!("# public key: {}\n", public_key));
    output.push_str(identity.to_string().expose_secret());
    output.push('\n');

    Ok((public_key, output))
}

pub fn encrypt(plaintext: &[u8], recipients: &[age::x25519::Recipient]) -> Result<Vec<u8>> {
    let encryptor = age::Encryptor::with_recipients(
        recipients.iter().map(|r| r as &dyn age::Recipient),
    )
    .context("No recipients")?;

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(ciphertext: &[u8], identity: &age::x25519::Identity) -> Result<Vec<u8>> {
    let decryptor =
        age::Decryptor::new(ciphertext).map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    let mut reader = decryptor
        .decrypt(std::iter::once(identity as &dyn age::Identity))
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    let mut decrypted = vec![];
    reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}

pub fn parse_recipient(key: &str) -> Result<age::x25519::Recipient> {
    key.parse::<age::x25519::Recipient>()
        .map_err(|_| anyhow::anyhow!("Invalid age public key: {}", key))
}

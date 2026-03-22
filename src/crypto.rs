use age::secrecy::ExposeSecret;
use anyhow::{Context, Result};
use std::io::{Read, Write};

/// Generate a new age X25519 identity (key pair).
///
/// Returns a tuple of `(public_key, identity_file_contents)` where:
/// - `public_key` is the age public key string (starts with `age1...`)
/// - `identity_file_contents` is the full identity file including the public key comment
pub fn generate_identity() -> Result<(String, String)> {
    let identity = age::x25519::Identity::generate();
    let public_key = identity.to_public().to_string();

    let mut output = String::new();
    output.push_str(&format!("# public key: {}\n", public_key));
    output.push_str(identity.to_string().expose_secret());
    output.push('\n');

    Ok((public_key, output))
}

/// Encrypt plaintext for a set of age recipients.
///
/// The ciphertext can be decrypted by any of the recipients using their private key.
/// Uses X25519 key agreement and ChaCha20-Poly1305 AEAD encryption.
pub fn encrypt(plaintext: &[u8], recipients: &[age::x25519::Recipient]) -> Result<Vec<u8>> {
    let encryptor =
        age::Encryptor::with_recipients(recipients.iter().map(|r| r as &dyn age::Recipient))
            .context("No recipients")?;

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext)?;
    writer.finish()?;

    Ok(encrypted)
}

/// Decrypt age-encrypted ciphertext using an identity (private key).
///
/// Returns the decrypted plaintext bytes. Fails if the identity is not
/// one of the recipients the data was encrypted for.
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

/// Parse an age public key string into a [`age::x25519::Recipient`].
///
/// The key must start with `age1` and be a valid bech32-encoded X25519 public key.
pub fn parse_recipient(key: &str) -> Result<age::x25519::Recipient> {
    key.parse::<age::x25519::Recipient>()
        .map_err(|_| anyhow::anyhow!("Invalid age public key: {}", key))
}

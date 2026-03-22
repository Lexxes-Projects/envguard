use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::crypto;

/// Returns the current working directory as the project root.
pub fn project_root() -> Result<PathBuf> {
    Ok(std::env::current_dir()?)
}

/// Returns the path to the `.envguard/` directory in the project root.
pub fn envguard_dir() -> Result<PathBuf> {
    Ok(project_root()?.join(".envguard"))
}

fn secrets_path(env: &str) -> Result<PathBuf> {
    Ok(envguard_dir()?.join("secrets").join(format!("{}.age", env)))
}

/// Load the user's age identity (private key).
///
/// First checks the `ENVGUARD_IDENTITY` environment variable (for CI/CD use),
/// then falls back to reading `.envguard/keys/identity.age` from disk.
pub fn load_identity() -> Result<age::x25519::Identity> {
    // Check ENVGUARD_IDENTITY env var first (for CI/CD)
    if let Ok(identity_str) = std::env::var("ENVGUARD_IDENTITY") {
        for line in identity_str.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            return line
                .parse::<age::x25519::Identity>()
                .map_err(|_| anyhow::anyhow!("Invalid identity in ENVGUARD_IDENTITY"));
        }
        anyhow::bail!("No identity found in ENVGUARD_IDENTITY");
    }

    let path = envguard_dir()?.join("keys").join("identity.age");
    if !path.exists() {
        anyhow::bail!("Not initialized. Run: envguard init");
    }

    let content = std::fs::read_to_string(&path)?;
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        return line
            .parse::<age::x25519::Identity>()
            .map_err(|_| anyhow::anyhow!("Invalid identity file"));
    }

    anyhow::bail!("No identity found in key file");
}

/// Load all trusted recipients (public keys) from `.envguard/recipients.txt`.
///
/// Returns a list of parsed age recipients that secrets can be encrypted for.
pub fn load_recipients() -> Result<Vec<age::x25519::Recipient>> {
    let path = envguard_dir()?.join("recipients.txt");
    if !path.exists() {
        anyhow::bail!("No recipients.txt found. Run: envguard init");
    }

    let content = std::fs::read_to_string(&path)?;
    let mut recipients = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        recipients.push(
            crypto::parse_recipient(line)
                .with_context(|| format!("Invalid key in recipients.txt: {}", line))?,
        );
    }

    if recipients.is_empty() {
        anyhow::bail!("No recipients found in recipients.txt");
    }

    Ok(recipients)
}

/// Load and decrypt secrets for a given environment.
///
/// Reads the encrypted file at `.envguard/secrets/{env}.age`, decrypts it
/// using the provided identity, and parses the contents as key-value pairs.
/// Returns an empty map if the environment file doesn't exist yet.
pub fn load_secrets(
    env: &str,
    identity: &age::x25519::Identity,
) -> Result<BTreeMap<String, String>> {
    let path = secrets_path(env)?;

    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let ciphertext = std::fs::read(&path)?;
    let plaintext = crypto::decrypt(&ciphertext, identity)?;
    let content = String::from_utf8(plaintext)?;

    Ok(parse_env(&content))
}

/// Encrypt and save secrets for a given environment.
///
/// Serializes the secrets as `KEY=VALUE` pairs, encrypts them for all
/// provided recipients, and writes to `.envguard/secrets/{env}.age`.
pub fn save_secrets(
    env: &str,
    secrets: &BTreeMap<String, String>,
    recipients: &[age::x25519::Recipient],
) -> Result<()> {
    let plaintext = serialize_env(secrets);
    let ciphertext = crypto::encrypt(plaintext.as_bytes(), recipients)?;

    let path = secrets_path(env)?;
    std::fs::write(&path, &ciphertext)?;

    Ok(())
}

/// Parse a `.env` formatted string into key-value pairs.
///
/// Handles comments (lines starting with `#`), empty lines,
/// and values with single or double quotes.
pub fn parse_env(content: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let mut value = value.trim().to_string();

            // Strip surrounding quotes
            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value = value[1..value.len() - 1].to_string();
            }

            map.insert(key, value);
        }
    }

    map
}

fn serialize_env(secrets: &BTreeMap<String, String>) -> String {
    let mut output = String::new();
    for (key, value) in secrets {
        output.push_str(key);
        output.push('=');
        output.push_str(value);
        output.push('\n');
    }
    output
}

//! # envguard
//!
//! Encrypted `.env` files that just work. Zero config, single binary, built-in team sharing.
//!
//! ## Overview
//!
//! envguard encrypts your `.env` secrets using [age](https://github.com/FiloSottile/age)
//! so you can commit them directly to git. Your team shares public keys, not secrets.
//!
//! ## CLI Usage
//!
//! envguard is primarily a command-line tool:
//!
//! ```sh
//! cargo install envguard
//!
//! envguard init                          # Generate encryption keys
//! envguard set API_KEY "sk-123"          # Add an encrypted secret
//! envguard run -- npm start              # Run with secrets injected
//! envguard trust age1abc...              # Add a teammate
//! ```
//!
//! See the [README](https://github.com/Lexxes-Projects/envguard) for full CLI documentation.
//!
//! ## Library Usage
//!
//! The [`crypto`] and [`store`] modules can also be used as a library for
//! programmatic access to envguard vaults:
//!
//! - [`crypto::generate_identity`] — Generate a new age key pair
//! - [`crypto::encrypt`] — Encrypt data for a set of recipients
//! - [`crypto::decrypt`] — Decrypt data using an identity
//! - [`crypto::parse_recipient`] — Parse an age public key string
//! - [`store::load_identity`] — Load the current user's identity (or from `ENVGUARD_IDENTITY` env var)
//! - [`store::load_recipients`] — Load all trusted public keys
//! - [`store::load_secrets`] — Decrypt and load secrets for an environment
//! - [`store::save_secrets`] — Encrypt and save secrets for an environment
//! - [`store::parse_env`] — Parse a `.env` formatted string into key-value pairs

/// Encryption and decryption using age (X25519 + ChaCha20-Poly1305).
///
/// This module wraps the [`age`](https://docs.rs/age) crate to provide
/// simple encrypt/decrypt operations and key management for envguard.
pub mod crypto;

/// Secret storage and file management.
///
/// Handles reading and writing encrypted secret files, loading identities
/// and recipients, and parsing `.env` formatted content.
pub mod store;

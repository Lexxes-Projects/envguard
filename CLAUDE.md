# envguard

Encrypted `.env` files that just work. Replace plaintext secrets with git-safe encrypted vaults. Zero config, single binary, built-in team sharing.

## Pitch

"Stop sharing .env files over Slack. Encrypt them, commit them, share keys — not secrets."

## Stack

- **Language:** Rust
- **Encryption:** age (via `rage` crate) — X25519 + ChaCha20-Poly1305
- **CLI:** clap (derive macros)
- **Distribution:** cargo-dist, GitHub Releases, Homebrew

## Architecture

```
.envguard/
├── keys/
│   └── identity.age      # User's private key (gitignored)
├── recipients.txt         # All trusted public keys (committed)
└── secrets/
    ├── default.age        # Encrypted key-value store (committed)
    ├── development.age    # Per-environment (committed)
    └── production.age     # Per-environment (committed)
```

- Each environment is a separate age-encrypted file
- File contains newline-separated KEY=VALUE pairs (plaintext inside the encrypted blob)
- Encrypted to ALL recipients in recipients.txt → any team member can decrypt
- Adding a new team member = add their pubkey + re-encrypt all files

## MVP Commands

- `envguard init` — Generate keypair, create .envguard/ structure, add to .gitignore
- `envguard set KEY VALUE [-e env]` — Decrypt, add/update key, re-encrypt
- `envguard get KEY [-e env]` — Decrypt and print single value
- `envguard list [-e env]` — List all keys (names only, not values)
- `envguard run [-e env] -- <command>` — Decrypt all, inject as env vars, run subprocess
- `envguard trust <public-key>` — Add recipient, re-encrypt all files for new recipient set
- `envguard export [-e env]` — Decrypt all, print as .env format to stdout
- `envguard keys` — Show your public key (for sharing with teammates)

## Differentiation vs Competitors

- **vs dotenvx:** No Node.js dependency, built-in team key sharing, single binary
- **vs SOPS:** No config files needed, .env-native, built-in `run` command
- **vs direnv:** Actually encrypts secrets, works in CI/Docker, not shell-only
- **vs Doppler/Infisical:** No server, no SaaS, no subscription, fully offline

## Status

- [ ] Project scaffolding
- [ ] Core encryption/decryption with age
- [ ] init command
- [ ] set/get commands
- [ ] run command
- [ ] trust command
- [ ] export/list/keys commands
- [ ] Error handling and polish
- [ ] README with GIF demo
- [ ] GitHub release setup

<p align="center">
  <h1 align="center">envguard</h1>
  <p align="center">Encrypted .env files that just work.</p>
  <p align="center">
    <a href="https://github.com/Lexxes-Projects/envguard/actions"><img src="https://img.shields.io/github/actions/workflow/status/Lexxes-Projects/envguard/ci.yml?style=flat-square" alt="CI"></a>
    <a href="https://crates.io/crates/envguard"><img src="https://img.shields.io/crates/v/envguard?style=flat-square" alt="crates.io"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
    <a href="https://ko-fi.com/lexxes"><img src="https://img.shields.io/badge/Ko--fi-Support%20envguard-ff5e5b?style=flat-square&logo=ko-fi&logoColor=white" alt="Ko-fi"></a>
  </p>
</p>

---

Stop sharing `.env` files over Slack. **envguard** encrypts your secrets, commits them to git, and lets your whole team decrypt — without a server, SaaS, or complex setup.

```bash
# Initialize in your project
envguard init

# Add secrets
envguard set DATABASE_URL "postgres://user:pass@localhost/mydb"
envguard set API_KEY "sk-live-abc123"

# Run your app with secrets injected
envguard run -- npm start

# Share with a teammate — just their public key
envguard trust age1abc...their_public_key
```

That's it. Secrets are encrypted with [age](https://github.com/FiloSottile/age), committed to git, and decryptable by anyone you trust.

## Why envguard?

| | envguard | dotenvx | SOPS | Doppler/Infisical |
|---|---|---|---|---|
| Zero config | **Yes** | No | No (.sops.yaml) | No (server/SaaS) |
| Single binary | **Yes** | No (Node.js) | Yes | No |
| Built-in `run` command | **Yes** | Yes | No | Yes |
| Team key sharing | **Built-in** | Manual | Manual | Built-in |
| Free & offline | **Yes** | Yes | Yes | No |
| `.env` native | **Yes** | Yes | No | No |

## Install

```bash
# From source
cargo install envguard

# Or download a binary from GitHub Releases
curl -sSL https://github.com/Lexxes-Projects/envguard/releases/latest/download/envguard-linux-x86_64 -o envguard
chmod +x envguard
```

## How it works

```
your-project/
├── .envguard/
│   ├── keys/
│   │   └── identity.age    # Your private key (gitignored)
│   ├── recipients.txt      # All trusted public keys (committed)
│   └── secrets/
│       ├── default.age     # Encrypted secrets (committed)
│       ├── development.age # Per-environment (committed)
│       └── production.age  # Per-environment (committed)
└── ...
```

- Secrets are encrypted with **age** (X25519 + ChaCha20-Poly1305)
- Each environment is a separate encrypted file
- All trusted team members can decrypt (multi-recipient encryption)
- Private keys never leave your machine

## Usage

### Initialize

```bash
envguard init
# ✓ Initialized envguard
#   Your public key (share with teammates):
#   age1abc123...
```

### Manage secrets

```bash
# Set a secret (default environment)
envguard set DATABASE_URL "postgres://localhost/mydb"

# Set for a specific environment
envguard set API_KEY "sk-live-abc123" -e production

# Get a single secret
envguard get DATABASE_URL

# List all secret names
envguard list
envguard list -e production
```

### Run with secrets

```bash
# Inject secrets as environment variables
envguard run -- npm start
envguard run -e production -- ./deploy.sh

# Export as .env format (pipe to a file, use with Docker, etc.)
envguard export > .env
envguard export -e production | docker run --env-file /dev/stdin myapp
```

### Team sharing

```bash
# Show your public key
envguard keys
# age1abc123...

# Trust a teammate (re-encrypts all secrets for the new recipient)
envguard trust age1xyz789...

# That's it — they can now decrypt with their private key
```

### CI/CD

Set the `ENVGUARD_IDENTITY` environment variable to the contents of your identity file:

```yaml
# GitHub Actions
- name: Decrypt secrets
  env:
    ENVGUARD_IDENTITY: ${{ secrets.ENVGUARD_IDENTITY }}
  run: envguard run -- ./deploy.sh
```

## Security

- **Encryption:** age (X25519 key agreement, ChaCha20-Poly1305 AEAD)
- **Private keys** are stored locally and gitignored — they never leave your machine
- **No network calls** — everything is local, offline, and auditable
- **No dependencies at runtime** — single static binary

## License

MIT

---

<p align="center">
  <sub>Built with care by <a href="https://github.com/Lexxes-Projects">Lexxes Projects</a></sub><br>
  <sub>If envguard saves you time, <a href="https://ko-fi.com/lexxes">buy us a coffee</a> ☕</sub>
</p>

<p align="center">
  <h1 align="center">envguard</h1>
  <p align="center"><strong>Encrypted .env files that just work.</strong></p>
  <p align="center">
    <a href="https://github.com/Lexxes-Projects/envguard/actions"><img src="https://img.shields.io/github/actions/workflow/status/Lexxes-Projects/envguard/ci.yml?style=flat-square" alt="CI"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
    <a href="https://ko-fi.com/lexxes"><img src="https://img.shields.io/badge/Ko--fi-Support%20envguard-ff5e5b?style=flat-square&logo=ko-fi&logoColor=white" alt="Ko-fi"></a>
  </p>
</p>

<p align="center">
  <img src="demo.gif" alt="envguard demo" width="600">
</p>

## The problem

Every team has secrets: database passwords, API keys, tokens. And every team handles them badly:

- **Plaintext `.env` files** get copy-pasted over Slack, email, or sticky notes
- **`.gitignore`** means secrets aren't version-controlled — someone deletes theirs and the project breaks
- **New team members** wait hours for someone to send them the right `.env`
- **CI/CD** needs secrets too, so they end up hardcoded in dashboards or passed around in yet another channel
- **Paid tools** like Doppler or Infisical cost $8-21/user/month and require hosted infrastructure

You shouldn't need a SaaS subscription to keep your API keys safe.

## The solution

**envguard** encrypts your `.env` secrets so you can commit them directly to git. Your team shares *public keys*, not secrets. Anyone you trust can decrypt. No server, no cloud, no account.

```bash
envguard init                    # Generate your encryption keys
envguard set API_KEY "sk-123"    # Add a secret (encrypted instantly)
envguard run -- npm start        # Run your app with secrets injected
envguard trust age1abc...        # Add a teammate — that's it
```

Secrets are encrypted with [age](https://github.com/FiloSottile/age), the same modern encryption used by Mozilla SOPS, Google engineers, and the Go security team. Your private key never leaves your machine.

## Migrate in 10 seconds

Already have a `.env` file? Import it and you're done:

```bash
envguard init
envguard import .env
rm .env                          # No more plaintext secrets
git add .envguard && git commit  # Safe to commit
```

Your entire team can now clone the repo and decrypt — no Slack messages, no "can you send me the .env?"

## How it compares

| | envguard | dotenvx | SOPS | Doppler | Infisical |
|---|---|---|---|---|---|
| Zero config | **Yes** | No | No (.sops.yaml) | No | No |
| Single binary | **Yes** | No (Node.js) | Yes | No | No |
| Built-in `run` | **Yes** | Yes | No | Yes | Yes |
| Team key sharing | **Built-in** | Manual | Manual | Built-in | Built-in |
| Works offline | **Yes** | Yes | Yes | No | No |
| `.env` native | **Yes** | Yes | No | No | No |
| Self-hosted | **N/A** (no server) | N/A | N/A | $21/user/mo | $8/user/mo |
| **Cost** | **Free forever** | Free | Free | Paid | Paid |

## Install

```bash
# Download a pre-built binary (Linux, macOS, Windows)
curl -sSL https://github.com/Lexxes-Projects/envguard/releases/latest/download/envguard-linux-x86_64 -o envguard
chmod +x envguard && sudo mv envguard /usr/local/bin/

# Or install from crates.io (requires Rust)
cargo install envguard
```

## How it works

envguard uses **public-key cryptography** — the same concept behind SSH keys and HTTPS:

1. **Each person generates a key pair** — a public key (safe to share) and a private key (stays on your machine)
2. **Secrets are encrypted for all trusted public keys** — anyone in `recipients.txt` can decrypt
3. **Adding a teammate** means adding their public key and re-encrypting — they instantly get access
4. **Removing access** means removing their key and re-encrypting — they can no longer decrypt new versions

```
your-project/
├── .envguard/
│   ├── keys/
│   │   └── identity.age      # Your private key (gitignored, never committed)
│   ├── recipients.txt         # Public keys of everyone who can decrypt (committed)
│   └── secrets/
│       ├── default.age        # Encrypted secrets for default environment (committed)
│       ├── development.age    # Encrypted secrets for development (committed)
│       └── production.age     # Encrypted secrets for production (committed)
└── ...
```

**What gets committed to git:** `recipients.txt` (public keys) and `secrets/*.age` (encrypted blobs).
**What stays local:** `keys/identity.age` (your private key). This is automatically gitignored.

## Usage

### Getting started

```bash
# Initialize envguard in your project
envguard init
# => Your public key (share with teammates):
# => age1abc123...

# Add secrets
envguard set DATABASE_URL "postgres://user:pass@localhost/mydb"
envguard set API_KEY "sk-live-abc123"
envguard set STRIPE_KEY "sk_live_xyz" -e production
```

### Managing secrets

```bash
envguard get DATABASE_URL             # Print a single secret
envguard list                         # List all secret names (values stay hidden)
envguard list -e production           # List secrets for a specific environment
envguard remove API_KEY               # Delete a secret
envguard set API_KEY "new-value"      # Overwrite an existing secret
```

### Importing existing .env files

```bash
envguard import .env                          # Import all secrets from .env
envguard import .env.production -e production # Import into a specific environment
```

### Running your app

```bash
# Secrets are injected as environment variables — your app doesn't need to change
envguard run -- npm start
envguard run -- python manage.py runserver
envguard run -- go run .
envguard run -e production -- ./deploy.sh

# Or export as .env format for tools that need it
envguard export > .env
envguard export -e production | docker run --env-file /dev/stdin myapp
```

### Team sharing

```bash
# 1. Your teammate installs envguard and runs:
envguard init
envguard keys
# => age1xyz789...    (they send you this)

# 2. You trust their key:
envguard trust age1xyz789...
# => All secrets are re-encrypted for the new recipient

# 3. They pull from git and can immediately decrypt:
git pull
envguard get DATABASE_URL   # Works instantly
```

No Slack messages. No shared passwords. No "can someone send me the .env?"

### CI/CD

Your CI pipeline needs secrets too. Set the `ENVGUARD_IDENTITY` environment variable to the contents of your identity file:

```yaml
# GitHub Actions
- name: Deploy with secrets
  env:
    ENVGUARD_IDENTITY: ${{ secrets.ENVGUARD_IDENTITY }}
  run: envguard run -e production -- ./deploy.sh
```

```bash
# Any CI system — just set the env var
export ENVGUARD_IDENTITY="$(cat path/to/identity.age)"
envguard run -- ./deploy.sh
```

**Setup:** Copy the contents of `.envguard/keys/identity.age` into your CI's secret storage (e.g., GitHub repository secrets). envguard will use it automatically.

## Security

envguard is built on [age](https://github.com/FiloSottile/age), a modern file encryption tool designed by Filippo Valsorda (Go security lead at Google, previously at Cloudflare). age is used in production by Mozilla SOPS and is widely trusted in the security community.

### What's under the hood

| Layer | Algorithm | Purpose |
|---|---|---|
| Key agreement | X25519 (Curve25519) | Derives a shared secret between sender and each recipient |
| Encryption | ChaCha20-Poly1305 | Encrypts the actual secrets (AEAD — authenticated encryption) |
| Key derivation | HKDF-SHA-256 | Derives encryption keys from the shared secret |

This is the same class of cryptography used by Signal, WireGuard, and TLS 1.3.

### What envguard protects against

- **Secrets leaking via git** — encrypted files are opaque binary blobs, even if your repo goes public
- **Secrets leaking via Slack/email** — you share *public keys*, not secrets
- **Unauthorized access** — only people whose public key is in `recipients.txt` can decrypt
- **Tampering** — age uses authenticated encryption (AEAD), so modified ciphertext is rejected

### What envguard does NOT protect against

- **A compromised machine** — if an attacker has access to your machine, they can read your private key. This is true for any local encryption tool.
- **Secrets already in git history** — if you previously committed plaintext secrets, they're still in your git log. Use [git-filter-repo](https://github.com/newren/git-filter-repo) or [BFG Repo Cleaner](https://rtyley.github.io/bfg-repo-cleaner/) to remove them.
- **Secrets leaked via runtime** — env vars can appear in crash dumps, logs, or process listings. This is a general risk of environment variables, not specific to envguard.

### Trust model

- **Zero trust in third parties** — no server, no cloud, no SaaS, no network calls. Everything is local.
- **You control access** — `recipients.txt` is the source of truth. Add or remove keys as needed.
- **Private keys are never transmitted** — your key stays in `.envguard/keys/`, which is gitignored by default.
- **Open source** — the entire codebase is auditable. The encryption is handled by the well-reviewed `age` library, not custom crypto.

## Frequently asked questions

<details>
<summary><strong>What if I lose my private key?</strong></summary>

Another team member who still has access can re-initialize your key:

1. You run `envguard init` in a fresh directory to generate a new key pair
2. You send your new public key to a teammate
3. They run `envguard trust <your-new-key>`
4. You pull the updated repo and can decrypt again

If you're the only person with access and you lose your key, the secrets are unrecoverable. This is by design — there is no backdoor.
</details>

<details>
<summary><strong>How do I revoke someone's access?</strong></summary>

1. Remove their public key from `.envguard/recipients.txt`
2. Re-encrypt all secrets: `envguard rekey`
3. Rotate any secrets the revoked person had access to — they may have saved decrypted copies

Note: They can still decrypt older versions of the encrypted files from git history. Rotate your secrets after revoking access.
</details>

<details>
<summary><strong>Can I use this in a monorepo?</strong></summary>

Yes. Run `envguard init` in the root or in any subdirectory — envguard looks for `.envguard/` in the current directory.
</details>

<details>
<summary><strong>Is this production-ready?</strong></summary>

The encryption layer (age) is battle-tested and used in production by many organizations. envguard itself is a thin wrapper around age that manages key-value pairs and file structure. The attack surface is minimal. That said, always review any security tool before adopting it for sensitive workloads.
</details>

<details>
<summary><strong>How is this different from just using age directly?</strong></summary>

age encrypts files. envguard encrypts *secrets* — it understands key-value pairs, environments, team key management, and subprocess injection. Think of it as a developer-friendly layer on top of age, like how dotenv is a developer-friendly layer on top of environment variables.
</details>

<details>
<summary><strong>What if my repo becomes public?</strong></summary>

Your secrets are safe. The encrypted `.age` files are opaque binary blobs that cannot be decrypted without a private key. Only people listed in `recipients.txt` (whose private keys you have no access to) can decrypt them. This is the whole point.
</details>

## License

MIT

---

<p align="center">
  <sub>Built with care by <a href="https://github.com/Lexxes-Projects">Lexxes Projects</a></sub><br>
  <sub>If envguard saves you time, consider <a href="https://ko-fi.com/lexxes">supporting the project</a></sub>
</p>

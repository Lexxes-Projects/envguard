use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "envguard",
    about = "Encrypted .env files that just work.\n\nReplace plaintext secrets with git-safe encrypted vaults.\nZero config, single binary, built-in team sharing.",
    version,
    after_help = "If envguard saves you time, consider sponsoring: https://ko-fi.com/lexxes"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize envguard in the current project
    Init,

    /// Set a secret value
    Set {
        /// Secret name (e.g. DATABASE_URL)
        key: String,
        /// Secret value
        value: String,
        /// Environment (default, development, production, etc.)
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Get a secret value
    Get {
        /// Secret name
        key: String,
        /// Environment
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Remove a secret
    #[command(alias = "rm")]
    Remove {
        /// Secret name to remove
        key: String,
        /// Environment
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Import secrets from a .env file
    Import {
        /// Path to .env file (default: .env)
        #[arg(default_value = ".env")]
        file: String,
        /// Environment to import into
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// List all secret names (without values)
    List {
        /// Environment
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Run a command with decrypted secrets as env vars
    Run {
        /// Environment
        #[arg(short, long, default_value = "default")]
        env: String,
        /// Command and arguments (after --)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,
    },

    /// Trust a teammate's public key (re-encrypts all secrets)
    Trust {
        /// age public key (starts with age1...)
        public_key: String,
    },

    /// Export secrets as .env format to stdout
    Export {
        /// Environment
        #[arg(short, long, default_value = "default")]
        env: String,
    },

    /// Show your public key (share with teammates)
    Keys,
}

impl Cli {
    pub fn parse_args() -> Self {
        <Self as Parser>::parse()
    }
}

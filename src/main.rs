mod cli;
mod crypto;
mod store;

use anyhow::Result;
use cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse_args();

    match cli.command {
        Command::Init => cmd_init()?,
        Command::Set { key, value, env } => cmd_set(&key, &value, &env)?,
        Command::Get { key, env } => cmd_get(&key, &env)?,
        Command::Remove { key, env } => cmd_remove(&key, &env)?,
        Command::Import { file, env } => cmd_import(&file, &env)?,
        Command::List { env } => cmd_list(&env)?,
        Command::Run { env, command } => cmd_run(&env, &command)?,
        Command::Trust { public_key } => cmd_trust(&public_key)?,
        Command::Export { env } => cmd_export(&env)?,
        Command::Rekey => cmd_rekey()?,
        Command::Keys => cmd_keys()?,
    }

    Ok(())
}

fn cmd_init() -> Result<()> {
    let dir = store::envguard_dir()?;

    if dir.exists() {
        anyhow::bail!("Already initialized (.envguard/ exists)");
    }

    std::fs::create_dir_all(dir.join("keys"))?;
    std::fs::create_dir_all(dir.join("secrets"))?;

    let (public_key, identity) = crypto::generate_identity()?;

    std::fs::write(dir.join("keys").join("identity.age"), &identity)?;
    std::fs::write(dir.join("recipients.txt"), format!("{}\n", public_key))?;
    std::fs::write(dir.join(".gitignore"), "keys/\n")?;

    // Append to project .gitignore
    let project_gitignore = store::project_root()?.join(".gitignore");
    if project_gitignore.exists() {
        let content = std::fs::read_to_string(&project_gitignore)?;
        if !content.contains(".envguard/keys/") {
            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .open(&project_gitignore)?;
            use std::io::Write;
            writeln!(f, "\n# envguard private keys")?;
            writeln!(f, ".envguard/keys/")?;
        }
    }

    eprintln!("Initialized envguard");
    eprintln!("  Your public key (share with teammates):");
    eprintln!("  {}", public_key);
    eprintln!();
    eprintln!("  Run: envguard set MY_SECRET \"value\" to add your first secret");

    Ok(())
}

fn cmd_set(key: &str, value: &str, env: &str) -> Result<()> {
    let identity = store::load_identity()?;
    let recipients = store::load_recipients()?;
    let mut secrets = store::load_secrets(env, &identity)?;
    secrets.insert(key.to_string(), value.to_string());
    store::save_secrets(env, &secrets, &recipients)?;
    eprintln!("Set {} ({})", key, env);
    Ok(())
}

fn cmd_get(key: &str, env: &str) -> Result<()> {
    let identity = store::load_identity()?;
    let secrets = store::load_secrets(env, &identity)?;
    match secrets.get(key) {
        Some(value) => println!("{}", value),
        None => anyhow::bail!("Key '{}' not found in {}", key, env),
    }
    Ok(())
}

fn cmd_remove(key: &str, env: &str) -> Result<()> {
    let identity = store::load_identity()?;
    let recipients = store::load_recipients()?;
    let mut secrets = store::load_secrets(env, &identity)?;

    if secrets.remove(key).is_none() {
        anyhow::bail!("Key '{}' not found in {}", key, env);
    }

    store::save_secrets(env, &secrets, &recipients)?;
    eprintln!("Removed {} ({})", key, env);
    Ok(())
}

fn cmd_import(file: &str, env: &str) -> Result<()> {
    let path = std::path::Path::new(file);
    if !path.exists() {
        anyhow::bail!("File not found: {}", file);
    }

    let content = std::fs::read_to_string(path)?;
    let imported = store::parse_env(&content);

    if imported.is_empty() {
        anyhow::bail!("No secrets found in {}", file);
    }

    let identity = store::load_identity()?;
    let recipients = store::load_recipients()?;
    let mut secrets = store::load_secrets(env, &identity)?;

    let count = imported.len();
    for (key, value) in imported {
        secrets.insert(key, value);
    }

    store::save_secrets(env, &secrets, &recipients)?;
    eprintln!("Imported {} secrets from {} ({})", count, file, env);
    Ok(())
}

fn cmd_list(env: &str) -> Result<()> {
    let identity = store::load_identity()?;
    let secrets = store::load_secrets(env, &identity)?;
    let mut keys: Vec<_> = secrets.keys().collect();
    keys.sort();
    for key in keys {
        println!("{}", key);
    }
    Ok(())
}

fn cmd_run(env: &str, command: &[String]) -> Result<()> {
    if command.is_empty() {
        anyhow::bail!("No command specified. Usage: envguard run -- <command>");
    }
    let identity = store::load_identity()?;
    let secrets = store::load_secrets(env, &identity)?;
    let status = std::process::Command::new(&command[0])
        .args(&command[1..])
        .envs(&secrets)
        .status()?;
    std::process::exit(status.code().unwrap_or(1));
}

fn cmd_trust(public_key: &str) -> Result<()> {
    crypto::parse_recipient(public_key)?;

    let identity = store::load_identity()?;
    let recipients_path = store::envguard_dir()?.join("recipients.txt");
    let existing = std::fs::read_to_string(&recipients_path)?;
    if existing.lines().any(|l| l.trim() == public_key) {
        anyhow::bail!("Key already trusted");
    }

    {
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&recipients_path)?;
        use std::io::Write;
        writeln!(f, "{}", public_key)?;
    }

    let recipients = store::load_recipients()?;

    // Re-encrypt all existing secret files for new recipient set
    let secrets_dir = store::envguard_dir()?.join("secrets");
    if secrets_dir.exists() {
        for entry in std::fs::read_dir(&secrets_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("age") {
                let env_name = path.file_stem().unwrap().to_string_lossy().to_string();
                let secrets = store::load_secrets(&env_name, &identity)?;
                store::save_secrets(&env_name, &secrets, &recipients)?;
            }
        }
    }

    eprintln!("Trusted new key and re-encrypted all secrets");
    eprintln!("  Total recipients: {}", recipients.len());
    Ok(())
}

fn cmd_export(env: &str) -> Result<()> {
    let identity = store::load_identity()?;
    let secrets = store::load_secrets(env, &identity)?;
    let mut keys: Vec<_> = secrets.keys().collect();
    keys.sort();
    for key in keys {
        let value = &secrets[key];
        if value.contains(' ') || value.contains('"') || value.contains('#') {
            println!(
                "{}=\"{}\"",
                key,
                value.replace('\\', "\\\\").replace('"', "\\\"")
            );
        } else {
            println!("{}={}", key, value);
        }
    }
    Ok(())
}

fn cmd_rekey() -> Result<()> {
    let identity = store::load_identity()?;
    let recipients = store::load_recipients()?;

    let secrets_dir = store::envguard_dir()?.join("secrets");
    if !secrets_dir.exists() {
        anyhow::bail!("No secrets to re-encrypt");
    }

    let mut count = 0;
    for entry in std::fs::read_dir(&secrets_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("age") {
            let env_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let secrets = store::load_secrets(&env_name, &identity)?;
            store::save_secrets(&env_name, &secrets, &recipients)?;
            count += 1;
        }
    }

    eprintln!(
        "Re-encrypted {} environment(s) for {} recipient(s)",
        count,
        recipients.len()
    );
    Ok(())
}

fn cmd_keys() -> Result<()> {
    let identity_path = store::envguard_dir()?.join("keys").join("identity.age");
    if !identity_path.exists() {
        anyhow::bail!("Not initialized. Run: envguard init");
    }
    let content = std::fs::read_to_string(&identity_path)?;
    for line in content.lines() {
        if let Some(pubkey) = line.strip_prefix("# public key: ") {
            println!("{}", pubkey);
            return Ok(());
        }
    }
    anyhow::bail!("Could not find public key in identity file");
}

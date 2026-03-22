Output demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 900
Set Height 500
Set Theme "Catppuccin Mocha"
Set Padding 20

Type "# Initialize envguard in your project"
Enter
Sleep 500ms

Type "envguard init"
Enter
Sleep 1.5s

Type ""
Enter
Sleep 300ms

Type "# Add some secrets"
Enter
Sleep 500ms

Type 'envguard set DATABASE_URL "postgres://user:pass@localhost/mydb"'
Enter
Sleep 1s

Type 'envguard set API_KEY "sk-live-abc123secret"'
Enter
Sleep 1s

Type 'envguard set STRIPE_KEY "sk_live_xyz789" -e production'
Enter
Sleep 1s

Type ""
Enter
Sleep 300ms

Type "# List your secrets"
Enter
Sleep 500ms

Type "envguard list"
Enter
Sleep 1s

Type ""
Enter
Sleep 300ms

Type "# Run your app with secrets injected"
Enter
Sleep 500ms

Type 'envguard run -- sh -c "echo DB=\$DATABASE_URL"'
Enter
Sleep 1.5s

Type ""
Enter
Sleep 300ms

Type "# Export as .env format"
Enter
Sleep 500ms

Type "envguard export"
Enter
Sleep 1.5s

Type ""
Enter
Sleep 300ms

Type "# Share your key with teammates"
Enter
Sleep 500ms

Type "envguard keys"
Enter
Sleep 1.5s

Type ""
Enter
Sleep 300ms

Type "# Secrets are encrypted and safe to commit!"
Enter
Sleep 500ms

Type "file .envguard/secrets/default.age"
Enter
Sleep 2s

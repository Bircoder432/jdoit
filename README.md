# jdoit – Just Do It in the terminal

**jdoit** is a smart CLI tool that automates your terminal routine using patterns and hooks.

Stop typing long commands. Just say what you want, and jdoit will suggest what to do.

---

## ✨ Features

- 🔍 **Smart command matching** – by exact text or regex patterns
- ⚙️ **Customizable hooks** – define your own commands in simple TOML configs
- 🎯 **Context‑aware** – understands files, flags, and positional arguments
- 📦 **Modular configs** – organize hooks by topic (git, files, cargo, etc.)
- 🧠 **Intelligent argument parsing** – positional arguments, flags, and everything after `--` goes to `{@}`
- 🖥️ **Interactive menu** – powered by `skim` (fuzzy finder)
- ✅ **Safe execution** – always asks for confirmation before running commands
- 🚀 **Runs anything** – any command your terminal can handle

---

## 🚀 Quick start

```bash
# Clone a repo
jdoit clone https://github.com/user/repo

# Install a package (detects your distro)
jdoit install htop

# Create and edit a file
jdoit src/main.rs

# Run cargo with extra args
jdoit run -- --release

# Use custom hooks
jdoit commit "Fix bug" -a
```

---

## 📦 Installation

```bash
cargo install jdoit
```

Or build from source:

```bash
git clone https://github.com/vstor08/jdoit
cd jdoit
cargo build --release
sudo cp target/release/jdoit /usr/local/bin/
```

---

## ⚙️ Configuration

Hooks are stored in `~/.config/jdoit/hooks/*.toml`.

Example `git.toml`:

```toml
name = "Git hooks"
description = "Useful git commands"

[hooks.commit]
pattern = { text = "commit" }
command = "git commit -m {1}"
help = "Commit changes"
args = ["message"]
flags = ["-a"]

[hooks.clone]
pattern = { text = "clone" }
command = "git clone {1}"
help = "Clone repository"
args = ["url"]
```

---

## 📖 How it works

1. You type `jdoit <command> [args...]`
2. jdoit parses your input into a `UserContext` (flags + positional args)
3. It finds all hooks matching:
   - the command pattern (text or regex)
   - argument count and flags
4. You select the desired hook in an interactive `skim` menu
5. jdoit builds the final command, substitutes `{0}`, `{1}`, `{flags}`, `{@}`, and executes it

---

## 🧩 Built‑in placeholders

- `{0}`, `{1}`, … – positional arguments (0 = the command itself)
- `{flags}` – all flags (e.g. `-a -f value`)
- `{@}` – any extra positional arguments after the expected ones

---

## 🤝 Contributing

Issues and pull requests are welcome!

---

## 📄 License

MIT

---

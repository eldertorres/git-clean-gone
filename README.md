# git-clean-gone

A small Rust CLI that removes local Git branches whose upstream remote branch is marked as `[gone]`.

## What it does

When run inside a Git repository, `git-clean-gone`:

1. runs `git fetch --prune`
2. finds local branches shown by Git as `[gone]`
3. shows the branches marked as `[gone]`
4. skips the currently checked-out branch
5. lets you interactively choose which stale branches to delete
6. asks for confirmation
7. deletes selected stale branches with `git branch -D`
8. prints a summary

## Requirements

- Git must be installed and available in `PATH`
- Run the command inside a Git repository

## Installation

### Windows

Run in PowerShell:

```powershell
irm https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-windows.ps1 | iex
```

This downloads the latest Windows release and installs:

```text
%LOCALAPPDATA%\Programs\git-clean-gone\bin\git-clean-gone.exe
```

The installer adds that directory to the user `PATH`. If the command is not found immediately, open a new terminal.

### macOS

Run in Terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-unix.sh | sh
```

Supported release binaries:

- Apple Silicon: `aarch64-apple-darwin`
- Intel: `x86_64-apple-darwin`

The installer puts the binary in `~/.local/bin` by default.

### Linux

Run in Terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-unix.sh | sh
```

The published Linux binary currently targets `x86_64-unknown-linux-gnu`, which is the normal glibc-based x86_64 Linux target. This covers Arch Linux and most common desktop/server distributions.

The installer puts the binary in `~/.local/bin` by default.

### Arch Linux package build

Arch users can build and install with the included PKGBUILD:

```bash
git clone https://github.com/eldertorres/git-clean-gone.git
cd git-clean-gone/packaging/arch
makepkg -si
```

This installs `git-clean-gone` through `pacman` as a local package.

### Rust users

The project is not published to crates.io yet, so use the Git repository directly:

```bash
cargo install --git https://github.com/eldertorres/git-clean-gone.git
```

## Custom install directory

Unix/macOS:

```bash
GIT_CLEAN_GONE_INSTALL_DIR="$HOME/bin" \
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-unix.sh)"
```

Windows PowerShell:

```powershell
$env:GIT_CLEAN_GONE_INSTALL_DIR = "$HOME\bin"
irm https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-windows.ps1 | iex
```

## Install a specific version

Unix/macOS:

```bash
GIT_CLEAN_GONE_VERSION=v0.1.2 \
  sh -c "$(curl -fsSL https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-unix.sh)"
```

Windows PowerShell:

```powershell
$env:GIT_CLEAN_GONE_VERSION = "v0.1.2"
irm https://raw.githubusercontent.com/eldertorres/git-clean-gone/master/scripts/install-windows.ps1 | iex
```

## Usage

Inside a Git repository:

```bash
git-clean-gone
```

Example flow:

```text
🔍 Searching for branches tagged as [gone]...
🔄️ Branches refreshed successfully.
📍 Current branch: master will not be deleted
🔍 Found 3 branches tagged as [gone]

📋 Branches marked as [gone]:
  1. feature/foo
  2. feature/bar
  3. master (current branch, will be skipped)

✅ All eligible branches are selected for deletion by default.
Use Space to toggle a branch, Enter to confirm, and arrows to move.

? Select branches to delete:
❯ [x] feature/foo
  [x] feature/bar

⚠️ Do you want to delete these branches? (Y/N)
```

## Release process

This repository uses `cargo-dist` through `.github/workflows/release.yml` to build release assets for Windows, macOS, and Linux.

To publish a new version:

1. update `version` in `Cargo.toml`
2. commit the change
3. tag the commit, for example:

   ```bash
   git tag v0.1.2
   git push origin v0.1.2
   ```

4. wait for the GitHub Actions release workflow to finish

The stable install commands above download from the latest GitHub release assets, so users do not need to know the current version number.

## Notes

- There is no Homebrew tap yet.
- There is no crates.io package yet.
- The Windows installer currently supports x86_64 Windows.
- The Linux release binary currently supports x86_64 glibc Linux.

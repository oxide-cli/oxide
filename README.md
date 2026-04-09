# Oxide

Oxide is a Rust CLI for scaffolding JavaScript and TypeScript projects from Oxide templates.

It can:

- create a new project from a template
- download and cache templates locally
- reuse cached templates between runs
- authenticate against the Oxide service
- publish a GitHub repository as an Oxide template

## Installation

### Quick install

Linux and macOS:

```bash
curl -sSL https://raw.githubusercontent.com/oxide-cli/oxide/main/install.sh | bash
```

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/oxide-cli/oxide/main/install.ps1 | iex
```

Unix installers place the `oxide` binary into `~/.local/bin`, so make sure that directory is in your `PATH`.

### Install with npm

Install the CLI globally from npm:

```bash
npm install -g @maksym-zhuk/oxide-cli
```

This package downloads the matching Oxide binary from the latest GitHub release during `postinstall`.

### Install with cargo

Install the crate from crates.io:

```bash
cargo install oxide-cli
```

Cargo installs the `oxide` binary into Cargo's bin directory, so make sure `~/.cargo/bin` is in your `PATH`.

### Manual install from GitHub Releases

Download the latest release artifact for your platform:

- [Linux x86_64](https://github.com/oxide-cli/oxide/releases/latest/download/oxide-linux-x86_64.tar.gz)
- [macOS Apple Silicon](https://github.com/oxide-cli/oxide/releases/latest/download/oxide-macos-aarch64.tar.gz)
- [Windows x86_64](https://github.com/oxide-cli/oxide/releases/latest/download/oxide-windows-x86_64.zip)

## Getting started

Log in first:

```bash
oxide login
```

Create a new project from a template:

```bash
oxide new my-app react-vite-ts
```

If the template is not cached yet, Oxide downloads the latest version automatically before generating the project.

## Commands

```text
oxide new <NAME> <TEMPLATE_NAME>
oxide install-template <TEMPLATE_NAME>
oxide delete <TEMPLATE_NAME>
oxide installed
oxide login
oxide logout
oxide account
oxide publish-template <TEMPLATE_URL>
```

### Common workflows

Install or refresh a template in the local cache:

```bash
oxide install-template react-vite-ts
```

List cached templates:

```bash
oxide installed
```

Remove a cached template:

```bash
oxide delete react-vite-ts
```

Show the currently authenticated account:

```bash
oxide account
```

Publish a GitHub repository as a template:

```bash
oxide publish-template https://github.com/owner/repo
```

## How templates work

- Oxide stores local data under `~/.oxide/`.
- Cached templates live in `~/.oxide/cache/templates`.
- Authentication data is stored in `~/.oxide/auth.json`.
- Template downloads are version-aware: Oxide tracks the remote template commit SHA and skips re-downloading when the cached version is already current.
- Template files ending with `.tera` are rendered during project generation and written without the `.tera` suffix.

Available template variables:

- `project_name`
- `project_name_kebab`
- `project_name_snake`
- `tauri_user_name`

## Template publishing notes

Published templates are expected to include an `oxide.template.json` manifest in the template root. Oxide uses that manifest to track template metadata such as the template name, version, source repository, and whether the template is official.

## Development

Run the CLI locally:

```bash
cargo run -- --help
```

Run tests:

```bash
cargo test
```

## License

Licensed under either of these:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-Apache](LICENSE-Apache))

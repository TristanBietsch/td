# td

A minimal date/time CLI tool. Primarily designed for AI agents to quickly retrieve current date/time information.

## Install

### Prerequisites

- [Rust](https://rustup.rs/) (includes `cargo`)

### From source

```bash
# Clone the repository
git clone https://github.com/tristanbietsch/td.git
cd td

# Build and install to ~/.cargo/bin
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`.

### Manual install

```bash
# Build optimized binary
cargo build --release

# Copy to a directory in your PATH
sudo cp target/release/td /usr/local/bin/
```

## Usage

```bash
td              # 2026-01-29T14:32:05
td -d           # 2026-01-29
td -t           # 14:32:05
td -u           # UTC time
td -f "%A"      # Thursday
```

## Flags

| Flag | Description |
|------|-------------|
| `-d, --date` | Date only |
| `-t, --time` | Time only |
| `-u, --utc` | Use UTC |
| `-f, --format` | Custom strftime format |
| `-h, --help` | Help |
| `-V, --version` | Version |

## License

MIT

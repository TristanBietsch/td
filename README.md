# td

A minimal date/time CLI tool. Does one thing well.

## Install

```bash
cargo build --release
cp target/release/td /usr/local/bin/
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

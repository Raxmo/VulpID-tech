# VulpID-tech

A Rust game engine inspired by the IDtech engines (Quake, Doom).

## Building

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Cargo (included with Rust)

### Build Instructions

```bash
cd repo
cargo build
```

For release (optimized) build:

```bash
cargo build --release
```

## Running

```bash
cd repo
cargo run
```

With release optimizations:

```bash
cargo run --release
```

## Testing

Run the test suite:

```bash
cd repo
cargo test
```

## Development

- All source code is in `/repo`
- Development notes in `/notes`
- Test data and external tests in `/tests`
- Detailed documentation in `DOCUMENTATION.md`
- Project guidance in `AGENTS.md`

### Code Quality

Format code:
```bash
cd repo && cargo fmt
```

Check for issues:
```bash
cd repo && cargo clippy
```

## Project Status

See `/notes` for development progress and planning.

## Documentation

For architecture, design decisions, and technical details, see `DOCUMENTATION.md`.

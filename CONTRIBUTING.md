# Contributing to tmuxship

Thank you for your interest in contributing to tmuxship!

## Development Setup

### Prerequisites

- Rust (latest stable)
- tmux
- Starship

### Building from source

```bash
# Clone the repository
git clone https://github.com/Yukaii/tmuxship.git
cd tmuxship

# Build
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Project Structure

```
tmuxship/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── lib.rs       # Library interface
│   ├── config.rs    # Config resolution logic
│   └── render.rs    # ANSI to tmux format conversion
├── tests/           # Integration tests
├── examples/        # Example Starship configs
└── Cargo.toml
```

## Testing

### Running tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'
```

### Test organization

- **Unit tests**: Located alongside the code in `src/` modules
- **Integration tests**: Located in `tests/` directory
  - `cli.rs` - CLI interface tests
  - `config_resolution.rs` - Config resolution tests
  - `rendering.rs` - ANSI to tmux format conversion tests

### Manual testing

Test with tmux:

```bash
# Build
cargo build --release

# Test left status
./target/release/tmuxship left

# Test with specific config
./target/release/tmuxship left --config examples/starship.toml

# Test in tmux
tmux set -g status-left '#(./target/release/tmuxship left)'
```

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Keep functions small and focused
- Add tests for new functionality

## Architecture

### Config Resolution

The config resolution logic in `src/config.rs` follows this priority:

1. CLI flag (`--config`)
2. Environment variables (`TMUX_SHIP_<SIDE>_CONFIG`, `STARSHIP_CONFIG`)
3. XDG config directories
4. Home directory config files
5. Fallback to Starship defaults

### Rendering Pipeline

1. **Variable Fetching** (`src/lib.rs`)
   - Queries tmux for format variables
   - Sets environment variables with `TMUX_` prefix

2. **Starship Execution** (`src/lib.rs`)
   - Invokes `starship prompt` with resolved config
   - Captures JSON output

3. **Format Conversion** (`src/render.rs`)
   - Parses ANSI escape sequences
   - Converts to tmux format strings (`#[fg=...,bg=...,bold]`)
   - Handles RGB colors and basic styles

4. **Output** (`src/main.rs`)
   - Writes tmux-formatted string to stdout

### Adding New Features

When adding new features:

1. Add tests first (TDD approach)
2. Implement the feature
3. Update documentation
4. Add examples if relevant

Example workflow for adding a new variable:

```rust
// 1. Add to default variable list (src/lib.rs)
const DEFAULT_TMUX_VARS: &[&str] = &[
    "session_name",
    "your_new_var",  // Add here
    // ...
];

// 2. Add test (tests/*)
#[test]
fn test_new_variable() {
    // Test that the variable is fetched and set correctly
}

// 3. Update documentation (README.md, examples/)
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatting (`cargo fmt`)
6. Run linter (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, Remove, etc.)
- Reference issues when relevant

Examples:
- `Add support for custom color schemes`
- `Fix config resolution on macOS`
- `Update README with new examples`

## Performance Considerations

tmuxship is invoked frequently by tmux, so performance is important:

- Minimize tmux variable fetches (use `TMUX_SHIP_TMUX_VARS` if needed)
- Avoid unnecessary allocations in hot paths
- Keep Starship config simple (fewer modules = faster rendering)
- Use `cargo build --release` for benchmarking

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about the codebase
- General discussion

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT).

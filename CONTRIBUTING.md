# Contributing to hyprland-per-window-layout

Thank you for your interest in contributing! ❤️

## Development Setup

### Prerequisites

- Rust (stable channel)
- Hyprland compositor running
- At least 2 keyboard layouts configured in hyprland.conf

### Building

```bash
git clone https://github.com/coffebar/hyprland-per-window-layout.git
cd hyprland-per-window-layout
cargo build --release
```

### Running Locally

```bash
# Stop any existing instance
pkill hyprland-per-window-layout

# Run your development build
RUST_LOG=debug ./target/release/hyprland-per-window-layout
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Follow existing code patterns and conventions

## Testing Your Changes

1. Build the project with your changes
2. Run with `RUST_LOG=debug` to see debug output
3. Test switching between windows and verify layouts change correctly
4. Test with different applications (terminal, browser, chat apps)
5. Verify the daemon handles Hyprland restarts gracefully

## Submitting Changes

### Commit Messages

Use clear, descriptive commit messages:
- `fix: <description>` for bug fixes
- `feat: <description>` for new features
- `refactor: <description>` for code improvements
- `docs: <description>` for documentation changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and commit
4. Push to your fork
5. Open a Pull Request with:
   - Clear description of changes
   - How you tested the changes
   - Any relevant issue numbers

## Types of Contributions

### Bug Reports

- Check if the issue already exists
- Include Hyprland version (`hyprctl version`)
- Include debug logs (`RUST_LOG=debug`)
- Describe steps to reproduce

### Feature Requests

- Explain the use case
- Describe expected behavior
- Consider if it fits the project's "zero configuration" philosophy

### Code Contributions

- Bug fixes are always welcome
- New features should be discussed first (open an issue)
- Performance improvements are appreciated
- Code cleanup/refactoring is welcome if it improves maintainability

## Questions?

Open an issue for any questions about contributing.
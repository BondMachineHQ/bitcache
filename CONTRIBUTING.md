# Contributing to bitcache

Thank you for your interest in contributing to bitcache! This document provides guidelines and instructions for contributing.

## Development Setup

1. **Install Rust**: Get the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/)

2. **Clone the repository**:
   ```bash
   git clone https://github.com/BondMachineHQ/bitcache.git
   cd bitcache
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Code Style

- Follow standard Rust formatting using `cargo fmt`
- Ensure code passes `cargo clippy` checks
- Add documentation for public APIs
- Include examples in documentation where appropriate

## Making Changes

1. **Fork the repository** on GitHub

2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes** and commit them:
   ```bash
   git commit -m "Description of your changes"
   ```

4. **Run tests and checks**:
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

## Testing

Before submitting a pull request, ensure:

- All existing tests pass
- New functionality includes appropriate tests
- Code is properly formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)

## Documentation

- Update README.md if adding new features
- Add inline documentation for new functions and types
- Include usage examples for new functionality

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about the codebase
- Suggestions for improvements

Thank you for contributing!

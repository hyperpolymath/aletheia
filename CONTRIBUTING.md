# Contributing to Aletheia

Thank you for your interest in contributing to Aletheia! This document provides guidelines for contributing to the project.

## 🌟 Welcome

We welcome contributions of all kinds:

- 🐛 Bug reports
- 💡 Feature requests
- 📝 Documentation improvements
- 🔧 Code contributions
- 🧪 Test improvements
- 🎨 UX/UI suggestions

## 🚀 Quick Start

1. **Fork the repository** on GitLab
2. **Clone your fork**:
   ```bash
   git clone https://gitlab.com/YOUR_USERNAME/aletheia.git
   cd aletheia
   ```
3. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes**
5. **Test your changes**:
   ```bash
   cargo test
   cargo run
   ```
6. **Commit your changes**:
   ```bash
   git commit -m "feat: add your feature description"
   ```
7. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Create a Merge Request** on GitLab

## 📋 Contribution Guidelines

### Code Quality

All contributions must maintain RSR Bronze-level compliance:

- ✅ **Zero Dependencies**: Do not add external crates
- ✅ **No Unsafe Code**: Avoid `unsafe` blocks
- ✅ **Type Safety**: Leverage Rust's type system
- ✅ **Tests Required**: Add tests for new functionality
- ✅ **Documentation**: Update docs for user-facing changes

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples**:
```
feat(checker): add Silver-level compliance checks
fix(report): correct percentage calculation
docs(readme): update installation instructions
test(main): add integration test for path validation
```

### Code Style

- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Use `cargo clippy` and fix all warnings
- **Naming**: Follow [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html)
- **Comments**: Document public APIs and complex logic
- **Error Handling**: Use `Result` and `Option`, avoid panics

### Testing

All code must include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = your_function(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

Run tests before submitting:
```bash
cargo test
cargo test -- --nocapture  # See println! output
cargo test --release      # Test optimized build
```

## 🐛 Bug Reports

Good bug reports include:

1. **Clear title**: Summarize the issue
2. **Environment**: OS, Rust version, Aletheia version
3. **Steps to reproduce**: Minimal example
4. **Expected behavior**: What should happen
5. **Actual behavior**: What actually happens
6. **Additional context**: Logs, screenshots, etc.

**Template**:
```markdown
## Bug Description
[Clear description of the bug]

## Environment
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Aletheia version: [e.g., 0.1.0]

## Steps to Reproduce
1. Run `aletheia /path/to/repo`
2. See error

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happens]

## Additional Context
[Logs, screenshots, etc.]
```

## 💡 Feature Requests

Good feature requests include:

1. **Problem statement**: What problem does this solve?
2. **Proposed solution**: How should it work?
3. **Alternatives considered**: Other approaches?
4. **RSR impact**: Does this affect compliance levels?

## 🔍 Code Review Process

1. **Automated checks**: CI must pass (tests, formatting, clippy)
2. **Maintainer review**: At least one maintainer approval required
3. **RSR compliance**: Changes must not break Bronze-level compliance
4. **Documentation**: User-facing changes must update docs
5. **Changelog**: Notable changes added to CHANGELOG.md

## 🏗️ Development Setup

### Prerequisites

- Rust 1.75 or later
- Git
- (Optional) `just` for build automation
- (Optional) Nix for reproducible builds

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run locally
cargo run

# Run with arguments
cargo run -- /path/to/repo
```

### Using Just

```bash
# See all available commands
just --list

# Build and test
just build
just test

# Run checks
just check
just lint
just fmt

# Install locally
just install
```

## 🧪 Testing Strategy

### Unit Tests

Test individual functions in isolation:

```rust
#[test]
fn test_file_exists() {
    let temp_dir = create_temp_dir();
    assert!(file_exists(&temp_dir, "existing_file.txt"));
    assert!(!file_exists(&temp_dir, "missing_file.txt"));
}
```

### Integration Tests

Test complete workflows in `tests/` directory:

```rust
#[test]
fn test_rsr_verification_complete() {
    let test_repo = setup_rsr_compliant_repo();
    let report = verify_repository(&test_repo);
    assert!(report.bronze_compliance());
}
```

### Manual Testing

1. Test on RSR-compliant repositories
2. Test on non-compliant repositories
3. Test edge cases (empty dirs, symlinks, etc.)
4. Test error handling (invalid paths, permissions, etc.)

## 📚 Documentation

Update documentation for:

- **User-facing changes**: Update README.md
- **API changes**: Update code comments and examples
- **Security changes**: Update SECURITY.md
- **Build changes**: Update justfile, flake.nix

## 🌐 TPCF Compliance

Aletheia operates under **Perimeter 3 (Community Sandbox)** of the Tri-Perimeter Contribution Framework:

- **Open Contribution**: Anyone can submit merge requests
- **Review Required**: All contributions reviewed by maintainers
- **Quality Standards**: Must maintain RSR Bronze-level compliance
- **Code of Conduct**: Must follow [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

## 🎓 Learning Resources

New to Rust? These resources can help:

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [RSR Framework Documentation](https://gitlab.com/maa-framework)

## 🤝 Community

- **Issues**: [GitLab Issues](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues)
- **Merge Requests**: [GitLab MRs](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/merge_requests)
- **Discussions**: [GitLab Discussions](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues)

## 📜 License

By contributing, you agree that your contributions will be dual-licensed under:

- MIT License
- Palimpsest License v0.8

See [LICENSE.txt](LICENSE.txt) for details.

## 🙏 Thank You

Every contribution makes Aletheia better. Thank you for helping us pursue *alētheia* - the unconcealment of truth through rigorous standards.

---

**Questions?** Open an issue or ask in your merge request!

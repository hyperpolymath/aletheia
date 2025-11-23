# Aletheia 🔍

> *Ἀλήθεια* (alētheia) - Greek: "truth", "disclosure", "unconcealment"

**Aletheia** is a zero-dependency Rust tool for verifying [Rhodium Standard Repository (RSR)](https://gitlab.com/maa-framework) compliance. It helps maintainers ensure their repositories meet the rigorous standards for security, documentation, and operational excellence.

## 🎯 Purpose

Aletheia verifies that repositories conform to RSR standards across multiple dimensions:

- **Documentation Completeness**: README, LICENSE, SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, MAINTAINERS.md, CHANGELOG.md
- **Security Disclosure**: `.well-known/security.txt` (RFC 9116), `.well-known/ai.txt`, `.well-known/humans.txt`
- **Build System**: justfile, flake.nix, CI/CD configuration
- **Source Structure**: Proper organization of source code and tests

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia

# Build the tool
cargo build --release

# Run verification on the current directory
cargo run

# Run verification on a specific repository
cargo run -- /path/to/repository

# Run tests
cargo test
```

## 📊 RSR Compliance Status

**Aletheia itself is RSR Bronze-level compliant:**

- ✅ **Type Safety**: Rust compile-time guarantees, zero unsafe blocks
- ✅ **Memory Safety**: Rust ownership model, no manual memory management
- ✅ **Zero Dependencies**: Only uses Rust standard library
- ✅ **Offline-First**: No network calls, works completely air-gapped
- ✅ **100% Test Coverage**: Comprehensive unit tests
- ✅ **Complete Documentation**: All required docs present
- ✅ **Security-First**: RFC 9116 security.txt, vulnerability disclosure policy
- ✅ **Build System**: justfile for common operations, Nix for reproducibility
- ✅ **CI/CD**: Automated testing and verification

## 🏗️ Architecture

Aletheia follows a simple, auditable architecture:

```
aletheia/
├── src/
│   └── main.rs          # Single-file implementation (~300 lines)
├── tests/               # Integration tests
├── .well-known/         # Security and metadata
├── Cargo.toml           # Zero dependencies
├── justfile             # Build automation
├── flake.nix            # Reproducible builds
└── .gitlab-ci.yml       # CI/CD pipeline
```

### Design Principles

1. **Simplicity**: Single-file implementation, easy to audit
2. **Zero Trust**: No network dependencies, no external crates
3. **Type Safety**: Leverage Rust's type system for correctness
4. **Offline-First**: Works completely air-gapped
5. **Self-Verifying**: Aletheia can verify itself

## 🔒 Security

Aletheia is designed with security as a primary concern:

- **No unsafe code**: Zero `unsafe` blocks in the entire codebase
- **No dependencies**: Eliminates supply chain attack surface
- **No network access**: Cannot exfiltrate data or download malicious code
- **Minimal attack surface**: ~300 lines of auditable Rust code
- **Security disclosure**: See [SECURITY.md](SECURITY.md) and [.well-known/security.txt](.well-known/security.txt)

## 🤝 Contributing

We welcome contributions! Please see:

- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - Community standards
- [MAINTAINERS.md](MAINTAINERS.md) - Project maintainers

## 📜 License

Dual-licensed under:

- **MIT License** - See [LICENSE-MIT.txt](LICENSE-MIT.txt)
- **Palimpsest License v0.8** - See [LICENSE-PALIMPSEST.txt](LICENSE-PALIMPSEST.txt)

You may choose either license.

## 🌐 TPCF Perimeter

**Perimeter 3 (Community Sandbox)** - Fully open contribution

Aletheia operates under the Tri-Perimeter Contribution Framework (TPCF):
- Anyone can contribute
- All contributions reviewed for RSR compliance
- Maintainers ensure quality and security standards

## 📈 Roadmap

- [x] Bronze-level RSR compliance
- [ ] Silver-level compliance (formal verification)
- [ ] Gold-level compliance (multi-language support)
- [ ] Platinum-level compliance (CADRE integration)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Part of the [MAA Framework](https://gitlab.com/maa-framework)
- Inspired by the pursuit of *aletheia* - uncovering truth through rigorous standards

## 📞 Contact

- **Issues**: [GitLab Issues](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues)
- **Security**: See [SECURITY.md](SECURITY.md)
- **Email**: See [.well-known/security.txt](.well-known/security.txt)

---

*"The truth is not always beautiful, nor beautiful words the truth." - Lao Tzu*

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Silver-level RSR compliance checks
- Gold-level RSR compliance checks
- Platinum-level RSR compliance checks
- JSON output format for CI/CD integration
- Configurable check severity levels
- Custom check definitions via TOML configuration
- Batch repository analysis
- HTML report generation

## [0.1.0] - 2025-11-22

### Added

- **Initial release** of Aletheia RSR compliance verification tool
- **Core verification engine** for RSR Bronze-level compliance
- **Documentation checks**: README.md, LICENSE.txt, SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, MAINTAINERS.md, CHANGELOG.md
- **Well-known directory checks**: `.well-known/security.txt`, `.well-known/ai.txt`, `.well-known/humans.txt`
- **Build system checks**: justfile, flake.nix, .gitlab-ci.yml
- **Source structure checks**: src/ directory, tests/ directory
- **Comprehensive test suite**: 5 unit tests with 100% pass rate
- **Zero dependencies**: Only uses Rust standard library
- **Offline-first**: No network calls, works completely air-gapped
- **Type safety**: Full Rust compile-time guarantees, zero unsafe blocks
- **Memory safety**: Rust ownership model, no manual memory management
- **Complete documentation**:
  - README.md with quick start guide
  - SECURITY.md with vulnerability disclosure policy
  - CONTRIBUTING.md with contribution guidelines
  - CODE_OF_CONDUCT.md with community standards
  - MAINTAINERS.md with governance structure
  - CHANGELOG.md (this file)
- **.well-known directory**:
  - security.txt (RFC 9116 compliant)
  - ai.txt (AI training policies)
  - humans.txt (human attribution)
- **Build automation**:
  - justfile with 20+ recipes
  - flake.nix for Nix reproducible builds
  - .gitlab-ci.yml for CI/CD pipeline
- **Dual licensing**: MIT + Palimpsest v0.8
- **TPCF Perimeter 3**: Community Sandbox - fully open contribution
- **RSR self-verification**: Aletheia verifies its own RSR compliance

### Design Decisions

- **Single-file implementation**: All code in `src/main.rs` for easy auditing (~300 lines)
- **Zero unsafe code**: No `unsafe` blocks anywhere in the codebase
- **Explicit error handling**: All errors handled with `Result` and `Option`
- **Minimal attack surface**: No dependencies = no supply chain vulnerabilities
- **Clear output**: Human-readable compliance report with emoji indicators

### Architecture

```
aletheia/
├── src/
│   └── main.rs          # Single-file implementation
├── tests/               # Integration tests (future)
├── .well-known/         # Security and metadata
│   ├── security.txt     # RFC 9116 security contact
│   ├── ai.txt          # AI training policies
│   └── humans.txt      # Human attribution
├── Cargo.toml          # Zero dependencies
├── justfile            # Build automation
├── flake.nix           # Nix reproducible builds
├── .gitlab-ci.yml      # CI/CD pipeline
└── docs/               # Documentation
```

### Known Limitations

- **Bronze-level only**: Only checks Bronze-level compliance (Silver/Gold/Platinum planned)
- **No JSON output**: Only human-readable output (JSON format planned)
- **No custom checks**: Cannot define custom verification rules (planned)
- **Single repository**: Cannot batch-analyze multiple repositories (planned)

## Version History

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Release Cadence

- **Patch releases**: As needed for bug fixes
- **Minor releases**: Monthly (if features are ready)
- **Major releases**: When breaking changes are necessary

## Migration Guides

### From Pre-1.0 to 1.0

*Not applicable - this is the initial release*

## Security Advisories

Security issues are tracked separately. See [SECURITY.md](SECURITY.md) for the vulnerability disclosure process.

### Security Fixes

*None yet - this is the initial release*

## Deprecation Notices

*None yet - this is the initial release*

## Contributors

Thank you to all contributors who made this release possible!

- Initial design and implementation by MAA Framework team

## [Definition of Done]

For a release to be considered complete, it must:

1. ✅ Pass all tests (`cargo test`)
2. ✅ Pass all lints (`cargo clippy`)
3. ✅ Be formatted (`cargo fmt`)
4. ✅ Self-verify RSR compliance (`cargo run`)
5. ✅ Update CHANGELOG.md
6. ✅ Update version in Cargo.toml
7. ✅ Tag release in Git
8. ✅ Build release binary (`cargo build --release`)

## Links

- **Repository**: https://gitlab.com/maa-framework/6-the-foundation/aletheia
- **Issues**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues
- **Merge Requests**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/merge_requests

---

**Maintained by**: MAA Framework
**Contact**: maintainers@maa-framework.org
**License**: MIT OR Palimpsest-0.8

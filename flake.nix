{
  description = "Aletheia - RSR Compliance Verification Tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Use stable Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };

        # Build inputs
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        buildInputs = with pkgs; [
          # Add any system libraries here if needed
          # Currently zero dependencies, so none needed
        ];

        # Development tools
        devTools = with pkgs; [
          # Rust development tools
          rust-analyzer
          cargo-watch
          cargo-audit
          cargo-tarpaulin

          # Build automation
          just

          # Git
          git

          # Optional: mutation testing
          # cargo-mutants
        ];

      in
      {
        # Package definition
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "aletheia";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          meta = with pkgs.lib; {
            description = "RSR compliance verification tool - truth in repository standards";
            homepage = "https://gitlab.com/maa-framework/6-the-foundation/aletheia";
            license = with licenses; [ mit /* TODO: Add Palimpsest-0.8 when available */ ];
            maintainers = [ "MAA Framework" ];
            platforms = platforms.all;
          };

          # Run tests during build
          doCheck = true;

          # Additional check phase
          checkPhase = ''
            cargo test --release
          '';

          # Install phase
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/aletheia $out/bin/

            # Install documentation
            mkdir -p $out/share/doc/aletheia
            cp README.md $out/share/doc/aletheia/
            cp SECURITY.md $out/share/doc/aletheia/
            cp CONTRIBUTING.md $out/share/doc/aletheia/
            cp CODE_OF_CONDUCT.md $out/share/doc/aletheia/
            cp CHANGELOG.md $out/share/doc/aletheia/

            # Install well-known files
            mkdir -p $out/share/aletheia/.well-known
            cp .well-known/* $out/share/aletheia/.well-known/
          '';
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ devTools;

          shellHook = ''
            echo "🔍 Aletheia Development Environment"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  just build       - Build the project"
            echo "  just test        - Run tests"
            echo "  just check       - Run all checks"
            echo "  just run         - Verify current directory"
            echo "  just validate    - Self-verification"
            echo "  just --list      - Show all commands"
            echo ""
            echo "📚 Documentation:"
            echo "  README.md        - Project overview"
            echo "  CONTRIBUTING.md  - Contribution guidelines"
            echo "  SECURITY.md      - Security policy"
            echo ""
            echo "🎯 RSR Bronze Compliance:"
            echo "  ✅ Zero dependencies"
            echo "  ✅ No unsafe code"
            echo "  ✅ Type safety (Rust)"
            echo "  ✅ Memory safety (Ownership)"
            echo "  ✅ Offline-first"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
        };

        # CI/CD shell - minimal dependencies for faster CI builds
        devShells.ci = pkgs.mkShell {
          inherit buildInputs;

          nativeBuildInputs = nativeBuildInputs ++ [ pkgs.just ];

          shellHook = ''
            echo "CI Environment"
            rustc --version
            cargo --version
          '';
        };

        # Apps
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };

        # Formatter
        formatter = pkgs.nixpkgs-fmt;

        # Checks - run with `nix flake check`
        checks = {
          # Build check
          build = self.packages.${system}.default;

          # Format check
          format = pkgs.runCommand "check-format" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo fmt -- --check
            touch $out
          '';

          # Clippy check
          clippy = pkgs.runCommand "check-clippy" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo clippy -- -D warnings
            touch $out
          '';

          # Test check
          test = pkgs.runCommand "check-test" {
            buildInputs = [ rustToolchain ];
          } ''
            cd ${./.}
            cargo test
            touch $out
          '';

          # RSR self-verification
          rsr-compliance = pkgs.runCommand "check-rsr" {
            buildInputs = [ self.packages.${system}.default ];
          } ''
            cd ${./.}
            aletheia
            touch $out
          '';
        };
      }
    );
}

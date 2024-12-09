{
  description = "A flake for building a Rust workspace using buildRustPackage.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    flake-utils,
    rust-overlay,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        lib = pkgs.lib;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.stable.latest.default;
          rustc = pkgs.rust-bin.stable.latest.default;
        };

        app = rustPlatform.buildRustPackage rec {
          pname = "snake";
          version = "0.0.1";
          src = ./.;
          # cargoBuildFlags = "-p app";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [pkg-config cmake makeWrapper];
          buildInputs = with pkgs; [alsa-lib fontconfig libudev-zero];

          LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [wayland wayland-protocols libxkbcommon]);

          postFixup = ''
            wrapProgram $out/bin/snake \
              --set LD_LIBRARY_PATH ${LD_LIBRARY_PATH}
          '';
        };
      in {
        packages = rec {
          snake = app;
          default = snake;
        };
      }
    );
}

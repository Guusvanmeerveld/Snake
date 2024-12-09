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

        libraries = with pkgs; [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];

        app = rustPlatform.buildRustPackage {
          pname = "snake";
          version = "0.0.1";
          src = ./.;
          # cargoBuildFlags = "-p app";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [pkg-config cmake makeWrapper];
          buildInputs = with pkgs; [alsa-lib fontconfig libudev-zero];

          postFixup = ''
            wrapProgram $out/bin/snake \
              --set LD_LIBRARY_PATH ${lib.makeLibraryPath libraries}
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

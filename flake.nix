{
  description = "Native modding for the Bevy game engine.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs =
              [
                # Rust dependencies
                (
                  rust-bin.stable.latest.default.override {
                    extensions = [
                      "rust-analyzer"
                      "rust-src"
                    ];
                    targets = [ "wasm32-unknown-unknown" ];
                  }
                )
                cargo-edit
                openssl
                pkg-config
              ]
              ++ lib.optionals (lib.strings.hasInfix "linux" system) [
                # for Linux
                # Audio (Linux only)
                alsa-lib
                # Cross Platform 3D Graphics API
                vulkan-loader
                # For debugging around vulkan
                vulkan-tools
                # Wayland dependencies
                wayland
                # Other dependencies
                libudev-zero
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                libxkbcommon
                wasm-bindgen-cli
                binaryen
              ];
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              xorg.libX11
              xorg.libXi
              xorg.libXcursor
              libxkbcommon
            ];
            shellHook = ''
              export PATH="$HOME/.cargo/bin:$PATH"
            '';
          };
      }
    );
}


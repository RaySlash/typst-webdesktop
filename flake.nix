{
  description = "demoapp";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems = {
      url = "github:nix-systems/default";
      flake = false;
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
      ];

      systems = import inputs.systems;

      perSystem = { config, self', inputs', pkgs, system, ... }:
      let
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        msrv = cargoToml.package.rust-version;
        rust-toolchain = (pkgs.rust-bin.stable.latest.default).override {
          extensions = [ "rust-src" "rust-analyzer" "rustfmt" "clippy" ];
          targets = [  "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        };

        buildDeps = with pkgs; [
          pkg-config
          openssl
          glib
          cairo
          pango
          atk
          gdk-pixbuf
          libsoup
          gtk3
          libappindicator
          dioxus-cli
          webkitgtk
        ];


        mkDevShell = rust-toolchain: pkgs.mkShell {
          LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath buildDeps}:$LD_LIBRARY_PATH";
          packages = buildDeps ++ [ rust-toolchain ];
        };
    in
    {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ (import inputs.rust-overlay) ];
      };

      formatter = pkgs.alejandra;

      devShells.default = self'.devShells.msrv;
      devShells.msrv = (mkDevShell rust-toolchain);
      devShells.stable = (mkDevShell pkgs.rust-bin.stable.latest.default);
      devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)));
    };
  };
}

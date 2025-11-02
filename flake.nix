{
  inputs = {
    self.submodules = true;
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    let
      mkBits =
        pkgs:
        pkgs.rustPlatform.buildRustPackage {
          name = "bits";
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          src = builtins.path {
            path = ./.;
            name = "bits-src";
          };
        };
      overlay = final: prev: {
        bits = mkBits final;
      };
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ overlay ];
        };
      in
      {
        packages = {
          bits = pkgs.bits;
          default = self.packages.${system}.bits;
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ pkgs.bits ];
          packages = with pkgs; [
            rust-analyzer
            rustfmt
            clippy
            cargo-deny
            yaml-language-server
          ];
        };
      }
    );
}

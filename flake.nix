{
  description = "merge rubric and postgres membership databases";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        package = import ./nix/package.nix;
        docker-image = import ./nix/docker-image.nix;
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        craneLib = crane.mkLib pkgs;
      in
      {
        packages = rec {
          bin = package {
            inherit pkgs;
            craneLib = craneLib;
          };
          docker = docker-image {
            inherit pkgs;
            bin = bin;
          };
          default = docker;
        };

        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              rust-bin.stable.latest.default
              dive
            ];

            shellHook = '''';
          };
      }
    );
}

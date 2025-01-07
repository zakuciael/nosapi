{
  description = "A collection of Rust creates for all things NosTale.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      debug = true;

      perSystem =
        {
          pkgs,
          inputs',
          toolchain,
          rustPlatform,
          ...
        }:
        {
          _module.args = {
            toolchain = inputs'.fenix.packages.fromToolchainFile {
              file = ./rust-toolchain.toml;
              sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
            };
            rustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          };

          checks = {
            knope = pkgs.callPackage ./nix/tools/knope.nix { inherit rustPlatform; };
          };

          devShells = {
            default = pkgs.callPackage ./nix/shell.nix {
              inherit toolchain;
              cargo-wrapper = pkgs.callPackage ./nix/tools/cargo-wrapper.nix { cargo = toolchain; };
              knope = pkgs.callPackage ./nix/tools/knope.nix { inherit rustPlatform; };
              napi-cli = pkgs.callPackage ./nix/tools/napi-cli { };
            };
          };
        };
    };
}

{
  description = "A collection of Rust creates for all things NosTale.";

  nixConfig = {
    extra-substituters = [
      "https://nix-community.cachix.org"
      "https://crane.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
    ];
  };

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

      perSystem =
        {
          pkgs,
          inputs',
          crane,
          toolchain,
          ...
        }:
        {
          _module.args = {
            toolchain = inputs'.fenix.packages.stable.withComponents [
              "cargo"
              "rustc"
              "rust-src"
              "clippy"
              "rustfmt"
            ];
            crane = (inputs.crane.mkLib pkgs).overrideToolchain (_: toolchain);
          };

          formatter = pkgs.nixfmt-rfc-style;
          checks = { };
          packages = { };
          devShells = {
            default = pkgs.callPackage ./nix/shell.nix {
              inherit toolchain crane;
            };
          };
          apps = { };
        };
    };
}

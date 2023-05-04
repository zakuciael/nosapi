{
  description = "A collection of Rust creates for all things NosTale.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "i686-linux"
      ];
      debug = true;

      perSystem =
        {
          pkgs,
          system,
          crane,
          ...
        }:
        {
          _module.args = {
            pkgs = import inputs.nixpkgs {
              inherit system;
              config = { };
              overlays = [ inputs.fenix.overlays.default ];
            };
            crane = (inputs.crane.mkLib pkgs).overrideToolchain (
              { fenix, ... }:
              fenix.stable.withComponents [
                "cargo"
                "rustc"
                "rust-src"
                "clippy"
                "rustfmt"
              ]
            );
          };

          checks = { };
          packages = { };
          devShells = {
            default = pkgs.callPackage ./nix/shell.nix { inherit crane; };
          };
          apps = { };
        };
    };
}

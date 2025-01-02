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
    treefmt-nix.url = "github:numtide/treefmt-nix";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs =
    { flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [ inputs.treefmt-nix.flakeModule ];
      debug = true;

      perSystem =
        {
          config,
          pkgs,
          inputs',
          crane,
          toolchain,
          build,
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
            build = pkgs.callPackage ./nix/build.nix { inherit crane; };
          };

          treefmt = {
            flakeFormatter = true;
            flakeCheck = true;
            enableDefaultExcludes = true;
            projectRootFile = "flake.nix";
            programs = {
              # Common
              prettier = {
                enable = true;
                settings = {
                  printWidth = 100;
                  singleQuote = false;
                  tabWidth = 2;
                  semi = true;
                };
              };

              # YAML
              actionlint.enable = true;

              # TOML
              taplo.enable = true;

              # Nix
              deadnix.enable = true;
              statix.enable = true;
              nixfmt.enable = true;

              # Rust
              rustfmt.enable = true;
            };
          };

          checks = {
            inherit (build.packages) nosapi_blackbox;
            inherit (build.checks) clippy unit-tests;
          };

          devShells = {
            default = pkgs.callPackage ./nix/shell.nix {
              inherit toolchain crane;
              treefmt = config.treefmt.build.devShell;
            };
          };
        };
    };
}

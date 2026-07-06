{
  config,
  lib,
  pkgs,
  inputs,
  ...
}:
let
  rustBin = inputs.rust-overlay.lib.mkRustBin { } pkgs.buildPackages;
  toolchainFromFile = rustBin.fromRustupToolchainFile ./rust-toolchain.toml;
  # Workaround for RustRover being unable to find `clippy` without `rustup`
  # This wrapper forwards calls to `cargo check` to the `cargo clippy` command.
  toolchainPackage = toolchainFromFile.overrideAttrs (prev: {
    buildCommand = prev.buildCommand + "cp -f ${lib.getExe cargo-clippy-wrapper} $out/bin/cargo";
  });

  cargo = lib.getExe' config.languages.rust.toolchainPackage "cargo";

  mdformat-rustfmt =
    let
      inherit (pkgs) fetchFromGitHub;
      inherit (pkgs.python3Packages)
        buildPythonPackage
        poetry-core
        mdformat
        ;
    in
    buildPythonPackage (finalAttrs: {
      pname = "mdformat-rustfmt";
      version = "0.0.3";
      pyproject = true;

      src = fetchFromGitHub {
        owner = "hukkin";
        repo = "mdformat-rustfmt";
        tag = finalAttrs.version;
        hash = "sha256-qKHhNFQe0GsncwtqHnVVQ/17DX2/gQ7WCayEY+nlu/w=";
      };

      build-system = [
        poetry-core
      ];

      dependencies = [
        mdformat
        pkgs.rustfmt
      ];

      meta = {
        description = "Mdformat plugin to rustfmt Rust code blocks";
        homepage = "https://github.com/hukkin/mdformat-rustfmt";
        license = lib.licenses.mit;
        maintainers = [ ];
      };
    });

  cargo-clippy-wrapper =
    let
      cargo-unwrapped = lib.getExe' toolchainFromFile "cargo";
    in
    pkgs.writeShellApplication {
      name = "cargo-clippy-wrapper";
      text = ''
        if [ $# -gt 0 ] && [ "$1" == "check" ]; then
          ${cargo-unwrapped} clippy "''${@:2}"
        else
          ${cargo-unwrapped} "$@"
        fi
      '';
    };
in
{
  packages = with pkgs; [
    cargo-insta
    cargo-nextest
    cargo-msrv
    release-plz
    config.languages.rust.toolchainPackage
    pkg-config
    openssl
  ];

  languages = {
    rust = {
      enable = true;
      inherit toolchainPackage;
      lsp.package = toolchainPackage;
    };
    nix = {
      enable = true;
      lsp.enable = true;
    };
  };

  tasks = {
    "nosapi:build:dev".exec = "${cargo} build --workspace --all-features";
    "nosapi:build:release".exec = "${cargo} build --release --workspace --all-features";
    "nosapi:test".exec = "${cargo} nextest run --workspace --all-features";
  };

  processes =
    let
      browser-sync = pkgs.browser-sync + /bin/browser-sync;
      cargo-watch = lib.getExe pkgs.cargo-watch;
    in
    {
      "cargo-doc" = {
        exec = ''
          run-cargo-doc() {
            ${cargo} doc --no-deps --all-features --workspace

            # Trigger reload in browser
            ${browser-sync} reload \
              --port ${toString config.processes."browser-sync".ports.http.value}
          }; export -f run-cargo-doc

          ${cargo-watch} watch -s run-cargo-doc
        '';
        ready = {
          exec = "ls target/doc/*/index.html";
          period = 1;
          success_threshold = 1;
          failure_threshold = 100000;
        };
      };
      browser-sync = {
        exec = ''
          ${browser-sync} start \
            --port ${toString config.processes."browser-sync".ports.http.value} \
            --ss target/doc \
            -s target/doc \
            --directory
        '';
        ports.http.allocate = 8008;
        after = [ "devenv:processes:cargo-doc@ready" ];
      };
    };

  git-hooks.hooks = {
    treefmt.enable = true;
    clippy = {
      enable = true;
      settings = {
        allFeatures = true;
        denyWarnings = true;
        extraArgs = "--workspace";
      };
    };
  };

  treefmt = {
    enable = true;
    config = {
      settings = {
        excludes = [
          "*.lock"
          "*.patch"
          ".gitignore"
          ".gitmodules"
          ".hgignore"
          ".svnignore"
          "devenv.yaml"
          ".git/*"
          ".idea/*"
          ".zed/*"
          ".vscode/*"
          ".run/*"
          ".direnv/*"
          ".devenv/*"
          "assets/*"
          ".github/*.json"
          "*.snap"
          "*.snap.new"
        ];
        on-unmatched = "warn";
      };

      programs = {
        # GitHub Actions
        actionlint.enable = true;

        # Nix files
        nixfmt.enable = true;
        deadnix.enable = true;
        statix.enable = true;

        # Rust
        rustfmt.enable = true;

        # TOML
        taplo = {
          enable = true;
          settings = {
            array_auto_collapse = false;
            array_auto_expand = false;
          };
        };

        # Markdown
        mdformat = {
          enable = true;
          settings = {
            end-of-line = "lf";
            number = true;
          };
          plugins =
            ps: with ps; [
              mdformat-simple-breaks
              mdformat-gfm
              mdformat-gfm-alerts
              mdformat-beautysh
              mdformat-rustfmt
            ];
        };
      };
    };
  };
}

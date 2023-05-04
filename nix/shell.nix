{
  crane,
  writeShellApplication,
  fetchFromGitHub,
}:
let
  toolchain =
    let
      cargo = crane.cargo;
      cargo-wrapper = writeShellApplication {
        name = "cargo-wrapper";
        text = /* bash */
          ''
            if [ $# -gt 0 ] && [ "$1" == "check" ]; then
              ${cargo}/bin/cargo clippy "''${@:2}"
            else
              ${cargo}/bin/cargo "$@"
            fi
          '';
      };
    in
    crane.rustc.overrideAttrs (prev: {
      buildCommand =
        prev.buildCommand
        + ''
          cp -f ${cargo-wrapper}/bin/cargo-wrapper $out/bin/cargo
        '';
    });
  knope =
    let
      src = crane.cleanCargoSource (fetchFromGitHub {
        owner = "knope-dev";
        repo = "knope";
        rev = "knope/v0.18.1";
        hash = "sha256-KA5ePuN9MWbhsrz3UVr8brbs77P0AHXK/3f6RccfWac=";
      });
      commonArgs = {
        inherit src;
        inherit (crane.crateNameFromCargoToml { cargoToml = "${src}/crates/knope/Cargo.toml"; })
          pname
          version
          ;
        strictDeps = true;
      };
      cargoArtifacts = crane.buildDepsOnly commonArgs;
    in
    crane.buildPackage (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoExtraArgs = "-p knope";
        doCheck = false;
      }
    );
in
crane.devShell {
  name = "nosapi";

  packages = [ knope ];

  shellHook = /* bash */
    ''
      if [ -d .direnv/ ]; then
        rm -rf .direnv/links
        mkdir -p .direnv/links/
        ln -sf "${toolchain}" .direnv/links/rust
      fi
    '';
}

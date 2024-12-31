{ crane, fetchFromGitHub }:
let
  src = crane.cleanCargoSource (fetchFromGitHub {
    owner = "knope-dev";
    repo = "knope";
    rev = "knope/v0.18.1";
    hash = "sha256-KA5ePuN9MWbhsrz3UVr8brbs77P0AHXK/3f6RccfWac=";
  });
  commonArgs = {
    inherit src;
    inherit
      (crane.crateNameFromCargoToml {
        cargoToml = "${src}/crates/knope/Cargo.toml";
      })
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
)

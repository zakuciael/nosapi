{ lib, crane }:
let
  commonArgs = {
    src = crane.cleanCargoSource ./..;
    strictDeps = true;
    version = "0.0.0";
    pname = "workspace";
  };
  cargoArtifacts = crane.buildDepsOnly commonArgs;

  mkArgsForCrate =
    crate:
    let
      src = lib.fileset.toSource {
        root = ./..;
        fileset = lib.fileset.unions [
          ./../Cargo.toml
          ./../Cargo.lock
          (crane.fileset.commonCargoSources crate)
        ];
      };
    in
    commonArgs
    // rec {
      inherit (crane.crateNameFromCargoToml { cargoToml = "${crate}/Cargo.toml"; }) pname version;
      inherit cargoArtifacts;
      inherit src;

      cargoExtraArgs = "-p ${pname}";
      doCheck = false;
    };
in
{
  checks = {
    clippy = crane.cargoClippy (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--workspace -- --deny warnings";
      }
    );
    unit-tests = crane.cargoTest (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoTestExtraArgs = "--workspace -- --nocapture";
      }
    );
  };
  packages = {
    nosapi_blackbox = crane.buildPackage (mkArgsForCrate ./../crates/nosapi_blackbox);
  };
}

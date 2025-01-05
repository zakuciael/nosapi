{ crane, fetchFromGitHub }:
let
  src = crane.cleanCargoSource (fetchFromGitHub {
    owner = "crate-ci";
    repo = "cargo-release";
    rev = "be2f62752cd1aa7fa5a9aeb33b8c048588ff2d7c";
    hash = "sha256-8WmmMyocWivWdVRqhG2VkIdvUe5vuFmxJDAqr5SNv8w=";
  });
  commonArgs = {
    inherit src;
    strictDeps = true;
  };
  cargoArtifacts = crane.buildDepsOnly commonArgs;
in
crane.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    doCheck = false;
  }
)

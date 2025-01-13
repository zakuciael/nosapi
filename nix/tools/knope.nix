{
  git,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
}:
rustPlatform.buildRustPackage rec {
  pname = "knope";
  version = "0.18.1";

  src = fetchFromGitHub {
    owner = "knope-dev";
    repo = "knope";
    rev = "knope/v${version}";
    hash = "sha256-KA5ePuN9MWbhsrz3UVr8brbs77P0AHXK/3f6RccfWac=";
  };

  cargoHash = "sha256-QOpw1/ZpDl9UsQS+j4Km+iFXpChWajNp69kQFyFfRW4=";

  nativeBuildInputs = [ pkg-config ];

  nativeCheckInputs = [ git ];

  cargoBuildFlags = [
    "-p"
    "knope"
  ];
}

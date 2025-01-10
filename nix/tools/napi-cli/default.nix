{
  pkgs,
  nodejs,
  stdenv,
}:
let
  nodePackages = import ./composition.nix {
    inherit pkgs nodejs;
    inherit (stdenv.hostPlatform) system;
  };
in
nodePackages."@napi-rs/cli-3.0.0-alpha.65"

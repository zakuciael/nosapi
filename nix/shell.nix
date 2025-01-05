{
  toolchain,
  crane,
  callPackage,
  nixd,
  nixfmt-rfc-style,
  treefmt,
  knope ? (callPackage ./tools/knope.nix { inherit crane; }),
  cargo-wrapper ? (callPackage ./tools/cargo-wrapper.nix { inherit (crane) cargo; }),
  cargo-release ? (callPackage ./tools/cargo-release.nix { inherit crane; }),
}:
let
  wrappedToolchain = toolchain.overrideAttrs (prev: {
    buildCommand = prev.buildCommand + "cp -f ${cargo-wrapper}/bin/cargo-wrapper $out/bin/cargo";
  });
in
crane.devShell {
  name = "nosapi";

  inputsFrom = [ treefmt ];

  packages = [
    knope
    cargo-release
    nixd
    nixfmt-rfc-style
    wrappedToolchain
  ];

  shellHook = ''
    if [ -d .direnv/ ]; then
      rm -rf .direnv/links
      mkdir -p .direnv/links/
      ln -sf "${wrappedToolchain}" .direnv/links/rust
    fi
  '';
}

{
  toolchain,
  crane,
  callPackage,
  nixd,
  nixfmt,
  knope ? (callPackage ./tools/knope.nix { inherit crane; }),
  cargo-wrapper ? (callPackage ./tools/cargo-wrapper.nix { inherit (crane) cargo; }),
}:
let
  wrappedToolchain = toolchain.overrideAttrs (prev: {
    buildCommand = prev.buildCommand + "cp -f ${cargo-wrapper}/bin/cargo-wrapper $out/bin/cargo";
  });
in
crane.devShell {
  name = "nosapi";

  packages = [
    knope
    nixd
    nixfmt
    wrappedToolchain
  ];

  shellHook = # bash
    ''
      if [ -d .direnv/ ]; then
        rm -rf .direnv/links
        mkdir -p .direnv/links/
        ln -sf "${wrappedToolchain}" .direnv/links/rust
      fi
    '';
}

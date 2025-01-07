{
  mkShell,
  toolchain,
  cargo-wrapper,
  knope,
  napi-cli,
  nodePackages,
  treefmt,
  actionlint,
  deadnix,
  nixfmt-rfc-style,
  prettier ? nodePackages.prettier,
  statix,
  taplo,
  nixd,
  just,
}:
let
  # Workaround for RustRover being unable to find `clippy` without `rustup`
  # This wrapper forwards calls to `cargo check` to the `cargo clippy` command.
  wrappedToolchain = toolchain.overrideAttrs (prev: {
    buildCommand = prev.buildCommand + "cp -f ${cargo-wrapper}/bin/cargo-wrapper $out/bin/cargo";
  });
in
mkShell {
  name = "nosapi";

  packages = [
    toolchain
    knope
    napi-cli
    treefmt
    actionlint
    deadnix
    nixfmt-rfc-style
    prettier
    statix
    taplo
    nixd
    just
  ];

  shellHook = ''
    if [ -d .direnv/ ]; then
      rm -rf .direnv/links
      mkdir -p .direnv/links/
      ln -sf "${wrappedToolchain}" .direnv/links/rust
    fi

  '';
}

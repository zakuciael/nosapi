{
  mkShell,
  toolchain,
  cargo-wrapper,
  cargo-nextest,
  knope,
  napi-cli,
  nixd,
  just,
  treefmtDevShell,
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

  inputsFrom = [ treefmtDevShell ];

  packages = [
    toolchain
    cargo-nextest
    knope
    napi-cli
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

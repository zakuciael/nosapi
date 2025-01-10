{
  mkShell,
  treefmt,
  actionlint,
  deadnix,
  nixfmt-rfc-style,
  nodePackages,
  prettier ? nodePackages.prettier,
  statix,
  taplo,
  just,
  rustfmt,
}:
mkShell {
  name = "nosapi-treefmt";
  packages = [
    treefmt
    actionlint
    deadnix
    nixfmt-rfc-style
    prettier
    statix
    taplo
    just
    rustfmt
  ];
}

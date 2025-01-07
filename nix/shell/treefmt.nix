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
  ];
}

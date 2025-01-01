{ cargo, writeShellApplication }:
writeShellApplication {
  name = "cargo-wrapper";
  text = # bash
    ''
      if [ $# -gt 0 ] && [ "$1" == "check" ]; then
        ${cargo}/bin/cargo clippy "''${@:2}"
      else
        ${cargo}/bin/cargo "$@"
      fi
    '';
}
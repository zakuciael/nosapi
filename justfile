[private]
@default:
    just --list --list-heading $'Usage: just <command> [args]\nCommands:\n' --list-prefix $' - '

@_internal_fmt *args:
    treefmt --config-file ./treefmt.toml {{ args }}

[doc('Format all modified files')]
@fmt: _internal_fmt

[doc('Run linters and unit tests')]
@check:
    just _internal_fmt --ci
    cargo clippy --workspace -- --deny warnings
    cargo test --workspace -- --nocapture

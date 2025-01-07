[private]
@default:
    just --list --list-heading $'Usage: just <command> [args]\nCommands:\n' --list-prefix $' - '

[doc('Run treefmt on modified files')]
@fmt *args:
    treefmt {{ args }}

[doc('Check repository for issues')]
@check:
    just fmt --ci
    cargo clippy --workspace -- --deny warnings
    cargo nextest run

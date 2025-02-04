set unstable := true

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

[doc('Run a HTTP that automatically builds and hot-realoads Rust Docs')]
[script]
docs-server:
    declare -a pids=()

    echo "Watching for docs changes..."
    cargo watch -s "cargo doc --no-deps --workspace" &
    pids[0]=$!
    echo "Starting browser-sync server..."
    browser-sync start -w --ss target/doc -s target/doc --directory --no-open &
    pids[1]=$!

    trap 'for pid in ${pids[@]}; do kill ${pid}; done' INT
    wait

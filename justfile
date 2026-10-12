check: lint-no-tracing lint-with-tracing
    echo 'looks good!'

lint-no-tracing:
    cargo clippy --no-default-features

lint-with-tracing:
    cargo clippy

run:
    cargo run

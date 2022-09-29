checks: lint test

lint:
    cargo clippy --workspace --all-targets --all-features --no-deps

test: pytest rust-test

rust-test:
    cargo test --workspace --all-targets --all-features

pytest: pydevelop
    pytest python/tests/

pydevelop:
    maturin develop --manifest-path python/Cargo.toml

deps:
    pip install maturin pytest

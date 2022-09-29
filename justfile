default: lint test

lint:
    cargo clippy --workspace

test: pytest
    cargo test --workspace

pytest: pydevelop
    pytest python/tests/

pydevelop:
    maturin develop --manifest-path python/Cargo.toml
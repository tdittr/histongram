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

corpus:
    mkdir -p corpus
    cd corpus && wget "http://corpus.canterbury.ac.nz/resources/large.tar.gz" && tar -xzf large.tar.gz && rm -f large.tar.gz

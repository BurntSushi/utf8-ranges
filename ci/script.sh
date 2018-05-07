#!/bin/sh

set -ex

cargo build --verbose
cargo doc --verbose

# If we're testing on an older version of Rust, then only check that we
# can build the crate. This is because the dev dependencies might be updated
# more frequently, and therefore might require a newer version of Rust.
#
# This isn't ideal. It's a compromise.
if [ "$TRAVIS_RUST_VERSION" = "1.12.0" ]; then
  exit
fi

# Run tests.
cargo test --verbose

# If we have nightly, test no_std mode by removing.
# the default feature, "std".
if [ "$TRAVIS_RUST_VERSION" = "nightly" ]; then
  cargo test --lib --verbose --no-default-features --features "alloc"
fi

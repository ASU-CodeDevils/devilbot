#!/bin/bash

cd resources
cargo build --release --target aarch64-unknown-linux-musl
(cd target/aarch64-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/)

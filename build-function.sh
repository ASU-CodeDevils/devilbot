#!/bin/bash

cd resources
cargo build --release --target x86_64-unknown-linux-musl
(cd target/x86_64-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/)

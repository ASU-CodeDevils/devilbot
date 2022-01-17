#!/bin/sh

rm -rf .cargo/config
rm -rf resources/.cargo/config
rm -rf node_modules
(cd resources && cargo clean)
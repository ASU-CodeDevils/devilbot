#!/bin/sh

kernel="$(uname)"
arch="$(uname -m)"
linker=""

case $kernel in
    Linux)
        echo "OS = Linux\nArch = ${arch}"
        linker="musl-gcc"
        echo "Linker = ${linker}"
        sudo apt install build-essential musl-tools musl-dev -y
        ;;

    Darwin)
        if [ "${arch}" = "x86_64" ]; then
            if [ "$(sysctl -in sysctl.proc_translated)" = "1" ]; then
                echo "OS = Mac M1 using Rosetta\nArch = ${arch}"
                linker="x86_64-linux-gnu-gcc"
                echo "Linker = ${linker}"
                sudo apt install gcc-x86-64-linux-gnu -y
            else
                echo "OS = Mac Intel\nArch = ${arch}"
                linker="x86_64-linux-musl-gcc"
                echo "Linker = ${linker}"
                ln -s /usr/local/bin/${linker} /usr/local/bin/musl-gcc
            fi
        elif [ "${arch}" = "arm64" ]; then
            echo "OS = Mac M1\nArch = ${arch}"
            linker="x86_64-linux-gnu-gcc"
            echo "Linker = ${linker}"
            sudo apt install gcc-x86-64-linux-gnu -y
        fi        
        ;;

    *)
        echo "Unknown architecture: ${kernel} ${arch}"
        echo "Exiting..."
        exit 0
        ;;
esac

echo "Adding rustup target"
rustup target add ${arch}-unknown-linux-musl

echo "Creating .cargo/config"
mkdir -p .cargo
echo "[target.${arch}-unknown-linux-musl]\nlinker = \"${linker}\"" > .cargo/config

echo "Building Project"
cd resources
cargo build --release --target ${arch}-unknown-linux-musl
(cd target/${arch}-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/)

echo "NPM Install"
npm install
echo "NPM Run Build"
npm run build
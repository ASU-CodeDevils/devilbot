#!/bin/sh

kernel="$(uname)"
arch="$(uname -m)"
linker="x86_64-linux-musl-gcc"
rustflags=""

case $kernel in
    Linux)
        echo "OS = Linux\nArch = ${arch}"
        linker="musl-gcc"
        echo "Linker = ${linker}"
        sudo apt install build-essential musl-tools musl-dev -y
        rustflags="\nrustflags = [ \"-C\", \"target-feature=+crt-static\", \"-C\", \"link-arg=-lgcc\" ]"
        # if [ "${arch}" = "aarch64" ]; then
        #     sudo apt install gcc-x86-64-linux-gnu -y
        # fi
        ;;

    Darwin)
        if [ "${arch}" = "x86_64" ]; then
            if [ "$(sysctl -in sysctl.proc_translated)" = "1" ]; then
                echo "OS = Mac M1 using Rosetta\nArch = ${arch}"
                echo "Linker = ${linker}"
                sudo apt install gcc-x86-64-linux-gnu -y
            else
                echo "OS = Mac Intel\nArch = ${arch}"
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
        echo "Unknown System: ${kernel} ${arch}"
        echo "Exiting..."
        exit 0
        ;;
esac

echo "Adding rustup target"
rustup target add x86_64-unknown-linux-musl

echo "Creating .cargo/config"
mkdir -p .cargo
echo "[target.x86_64-unknown-linux-musl]\nlinker = \"${linker}\"${rustflags}" > .cargo/config

echo "Creating resources/.cargo/config"
mkdir -p ./resources/.cargo
cp -r .cargo ./resources

echo "Building Project"
cd resources
cargo build --release --target x86_64-unknown-linux-musl
(cd target/x86_64-unknown-linux-musl/release && mkdir -p lambda && cp bootstrap lambda/)

echo "NPM Install"
npm install
echo "NPM Run Build"
npm run build
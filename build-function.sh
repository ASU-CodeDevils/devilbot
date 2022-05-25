#!/bin/sh

kernel="$(uname)"
arch="$(uname -m)"
linker="x86_64-linux-gnu-gcc"
rustflags=""

case $kernel in
    Linux)
        echo "OS: Linux\nArch: ${arch}\nLinker: ${linker}"
        sudo apt install build-essential -y
        if [ "${arch}" = "aarch64" ]; then
            export CC_x86_64_unknown_linux_musl=${linker}
            sudo apt install gcc-x86-64-linux-gnu -y
            sudo apt install musl-tools -y
        fi
        ;;

    Darwin)
        if [ "${arch}" = "x86_64" ]; then
            if [ "$(sysctl -n sysctl.proc_translated)" = "1" ]; then
                linker="x86_64-linux-musl-gcc"
                echo "OS: Mac M1 using Rosetta\nArch: ${arch}\nLinker: ${linker}"
                echo "Mac M1 using Rosetta currently WiP... Exiting"
                exit 1
                # pretty sure this should work akin to normal x86_64, but not sure
                # brew install FiloSottile/musl-cross/musl-cross
                # ln -s /usr/local/bin/${linker} /usr/local/bin/musl-gcc
            else
                linker="x86_64-linux-musl-gcc"
                echo "OS: Mac Intel\nArch: ${arch}\nLinker: ${linker}"
                brew install FiloSottile/musl-cross/musl-cross
                ln -s /usr/local/bin/${linker} /usr/local/bin/musl-gcc
            fi
        elif [ "${arch}" = "arm64" ]; then
            linker="x86_64-unknown-linux-gnu-gcc"
            echo "OS: Mac M1\nArch: ${arch}\nLinker: ${linker}"
            brew tap messense/macos-cross-toolchains
            brew install x86_64-unknown-linux-gnu
            export CC_x86_64_unknown_linux_musl=${linker}
        fi        
        ;;

    *)
        echo "Unknown System: ${kernel} ${arch}"
        echo "Exiting..."
        exit 1
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
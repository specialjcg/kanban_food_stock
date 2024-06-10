# Uninstall rustup
rustup self uninstall

# Optionally remove remaining files
rm -rf ~/.cargo ~/.rustup

# Reinstall rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustup --version
cargo --version

# Install toolchains
rustup install stable
rustup install nightly

# Set default toolchain
rustup default stable

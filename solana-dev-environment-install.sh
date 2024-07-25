#!/bin/bash

# Update and upgrade the system, and install required packages
sudo apt-get update && sudo apt-get upgrade -y && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev

# Rust and Cargo Installation
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
echo "PATH after configuring Cargo: $PATH"
cargo --version

# Solana CLI Installation
echo "Installing Solana CLI..."
sh -c "$(curl -sSfL https://release.solana.com/v1.18.17/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
echo "PATH after updating for Solana CLI: $PATH"
solana --version

# NVM (Node Version Manager) Installation
echo "Installing NVM..."
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
echo "PATH after configuring NVM: $PATH"
nvm install 20
node -v
npm -v

# Corepack and Yarn Installation
echo "Enabling Corepack..."
corepack enable
echo "Yarn version:"
yarn --version
export PATH="$HOME/.yarn/bin:$PATH"
echo "PATH after updating for Yarn: $PATH"

# AVM (Anchor Version Manager) Installation
echo "Installing AVM..."
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
export PATH="$HOME/.avm/bin:$PATH"
echo "PATH after updating for AVM: $PATH"
avm --version

# Update .bashrc to make PATH changes permanent
echo "Updating .bashrc for permanent PATH changes..."
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.bashrc
echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.bashrc
echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> ~/.bashrc
echo 'export PATH="$HOME/.yarn/bin:$PATH"' >> ~/.bashrc
echo 'export PATH="$HOME/.avm/bin:$PATH"' >> ~/.bashrc

# Source .bashrc to apply changes
echo "Reloading .bashrc..."
source ~/.bashrc

# Final PATH verification
echo "Final PATH: $PATH"

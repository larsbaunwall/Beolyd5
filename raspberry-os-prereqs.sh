#!/bin/bash

# Update the package lists for upgrades and new package installations
sudo apt-get update

# Install curl if not installed
sudo apt-get install -y curl

# Install build essentials
sudo apt-get install -y build-essential

# Install prerequisite libraries for Tauri
sudo apt-get install -y libwebkit2gtk-4.1-0 libappindicator3-dev libsecret-1-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add cargo to PATH
source $HOME/.cargo/env

# Install Tauri CLI
cargo install tauri-cli --force

# Install Xorg and Matchbox for GUI support
sudo apt-get install -y xorg matchbox-window-manager

# /bin/bash

sudo pacman -S --needed --noconfirm openssh git base-devel

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "$HOME/.bashrc"

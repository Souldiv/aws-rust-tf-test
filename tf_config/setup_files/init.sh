#!/bin/bash
echo "installing rust"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > Rustup.sh
sh Rustup.sh -y
cp -r ~/.{cargo,rustup,bash_profile,profile} /home/ubuntu/
echo 'export PATH=$PATH:$HOME/.cargo/env' >> /etc/profile

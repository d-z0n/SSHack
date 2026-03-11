#!/bin/bash

cargo install --path .

mkdir -p ~/.config/sshack/

cp -r ./themes ~/.config/sshack/
cp ./config.toml ~/.config/sshack/

mkdir -p ~/.sshack

echo -e "\x1b[2J\x1b[H"
echo "sshack installed!"
echo 
echo "make sure '~/.cargo/bin' is in \$PATH"
echo
echo "Run it with 'sshack --help'!"

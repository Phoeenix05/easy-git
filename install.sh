#!/bin/zsh

# cargo build --release && \
cargo install --path . && \
echo "\033[1;32m   Reloading\033[0m zsh shell" && \
exec zsh

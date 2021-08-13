# nvim-rs-hello

Neovim plugin made with Rust

A simple "hello" plugin example. Uses
[nvim-rs](https://github.com/KillTheMule/nvim-rs) and
[tokio](https://tokio.rs/) for RPC communication with
[Neovim](https://neovim.io/).

## Install

Packer install with

    use {
        "baysmith/nvim-rs-hello",
        run = "cargo build --release",
    }


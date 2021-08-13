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

On Windows, Packer may fail to run the build command. The following workaround may work instead.

    use {
        "baysmith/nvim-rs-hello",
        run = "/c cargo build --release",
    }

## Usage

Start the plugin with `:lua require("hello").start()`

The plugin log file `nvim-rs-hello.log` can be found in the plugin directory (displayed with `:lua print(require("hello").plugin_dir())`)

`:lua require("hello").ping("test")` will print a message with the arguments passed.

`:lua require("hello")["repeat"](1, 2, 3)`  will print a repeating message with the arguments passed. Note: the repeating never stops unless the plugin is stopped.

`:lua require("hello").stop()` stops the plugin.

Any function can be called on the plugin, but only the above requests are recognized. However, the log file will show the `RpcNotfication` mesage.

    :lua require("hello").hello("plugin")

will display in log as

    [DEBUG] nvim_rs::neovim:Get message RpcNotification { method: "hello", params: [String(Utf8String { s: Ok("plugin") })] }


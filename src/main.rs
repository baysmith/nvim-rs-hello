use async_trait::async_trait;
use log::*;
use nvim_rs::{compat::tokio::Compat, create::tokio as create, Handler, Neovim};
use rmpv::Value;
use std::default::Default;
use std::error::Error;
use tokio::io::Stdout;
use tokio::time;

#[derive(Clone)]
struct NeovimHandler {}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(&self, name: String, _args: Vec<Value>, neovim: Neovim<Compat<Stdout>>) {
        match name.as_ref() {
            "start" => {
                neovim
                    .command("lua print(\"hello plugin started\")")
                    .await
                    .unwrap();
            }
            "ping" => {
                let args_s = format!("{:?}", _args);
                let s = format!("lua print(\"hello pong {}\")", args_s.replace('"', "\\\""));
                neovim.command(s.as_str()).await.unwrap();
            }
            "repeat" => {
                let mut count = 0;
                tokio::spawn(async move {
                    let mut interval = time::interval(time::Duration::from_secs(3));
                    loop {
                        interval.tick().await;
                        let args_s = format!("{:?}", _args);
                        let s = format!(
                            "lua print(\"hello repeat {} {}\")",
                            count,
                            args_s.replace('"', "\\\"")
                        );
                        neovim.command(s.as_str()).await.unwrap();
                        count += 1;
                    }
                });
            }
            _ => {}
        }
    }
    async fn handle_request(
        &self,
        _name: String,
        _args: Vec<Value>,
        _neovim: Neovim<Compat<Stdout>>,
    ) -> Result<Value, Value> {
        Ok(Value::Nil)
    }
}

#[tokio::main]
async fn main() {
    let plugin_dir = if let Ok(dir) = std::env::var("HELLO_PLUGIN_DIR") {
        dir
    } else {
        std::env::set_var("HELLO_PLUGIN_DIR", ".");
        ".".to_string()
    };
    let config_file = format!("{}/hello-config-log.toml", plugin_dir);

    log_panics::init();
    if let Err(e) = log4rs::init_file(
        format!("{}/hello-config-log.toml", plugin_dir),
        Default::default(),
    ) {
        eprintln!("Error configuring logging with {}: {:?}", config_file, e);
        return;
    }
    let handler = NeovimHandler {};
    let (nvim, io_handler) = create::new_parent(handler).await;
    match io_handler.await {
        Err(join_error) => {
            error!("Error joining IO loop: {}", join_error);
        }
        Ok(Err(error)) => {
            if !error.is_reader_error() {
                nvim.err_writeln(&format!("Error: {}", error))
                    .await
                    .unwrap_or_else(|e| {
                        error!("{}", e);
                    });
            }

            if !error.is_channel_closed() {
                error!("{}", error);
                let mut source = error.source();
                while let Some(e) = source {
                    error!("Caused by: {}", e);
                    source = e.source();
                }
            }
        }
        Ok(Ok(())) => {
            debug!("exit");
        }
    }
}

use serde_json::json;
use tokio::io::{stdin, stdout};

use cln_plugin::{Builder, Error, Plugin};

#[derive(Clone)]
pub struct SimplePlugin {}

/// Simple RPC method to test IP passing
async fn pass_ip(
    _plugin: Plugin<SimplePlugin>,
    _v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    Ok(json!(""))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let builder = Builder::new(stdin(), stdout())
    .rpcmethod(
        "passip",
        "RPC method using an ip as param",
        pass_ip,
    );

    let midstate = if let Some(midstate) = builder.configure().await? {
        midstate
    } else {
        return Ok(());
    };

    let plugin = midstate.start(SimplePlugin{}).await?;
    plugin.join().await
}
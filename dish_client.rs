use std::convert::TryInto;
use std::io;

use libzmq::{prelude::*, ClientBuilder, TcpAddr};
use structopt::StructOpt;

use radio_chat::{self, Result};

#[derive(StructOpt)]
struct Options {
    identity: String
}

fn main() -> Result<()> {
    write();
    Ok(())
}

fn listen() -> Result<()> {
    let options = Options::from_args();
    let endpoint: TcpAddr = format!("{}:{}", options.identity, radio_chat::SERVER_PORT).try_into()?;
    let client = ClientBuilder::new().connect(endpoint).build()?;
    let message = client.recv_msg()?;
    println!("Messag arrivant : {}", message.to_str()?);
    Ok(())
}


fn dispatch_line(line : &str, client : &Client) -> Result<()> {
    client.send(line)?;
    Ok(())
}

fn write() -> Result<()> {
    let options = Options::from_args();
    let endpoint: TcpAddr = format!("{}:{}", options.identity, radio_chat::SERVER_PORT).try_into()?;
    let client = ClientBuilder::new().connect(endpoint).build()?;
    dispatch_line("coucou", client);
    Ok(())
}

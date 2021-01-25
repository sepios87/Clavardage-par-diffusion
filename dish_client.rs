use std::convert::TryInto;
use std::io::{self, BufRead};

use libzmq::{prelude::*, ClientBuilder, Client, TcpAddr};
use structopt::StructOpt;

use radio_chat::{self, Result};

use radio_chat::ContentsMessage;

#[derive(StructOpt)]
struct Options {
    identity: String
}

fn main() -> Result<()> {
    //listen()?;
    write()?;
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
    let message: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
    let personnes: Vec<String> = message[0].split_whitespace().map(|s| s.to_string()).collect();
    let message_obj = ContentsMessage {
        recipients : personnes,
        payload : message[1].clone()
    };
    let serialized_message = serde_json::to_string(&message_obj)?;
    client.send(serialized_message)?;
    Ok(())
}

fn write() -> Result<()> {
    let options = Options::from_args();
    let endpoint: TcpAddr = format!("{}:{}", options.identity, radio_chat::SERVER_PORT).try_into()?;
    let client = ClientBuilder::new().connect(endpoint).build()?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        dispatch_line(&line.unwrap(), &client)?;
    }
    
    Ok(())
}

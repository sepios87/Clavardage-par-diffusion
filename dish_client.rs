use std::convert::TryInto;
use std::io::{self, BufRead};

use libzmq::{prelude::*, ClientBuilder, Client, TcpAddr, Group, DishBuilder};
use structopt::StructOpt;

use std::thread;

use radio_chat::{self, Result, ContentsMessage};

#[derive(StructOpt)]
struct Options {
    identity: String
}

fn main() -> Result<()> {
    thread::spawn(move || {
        listen();
    });
    write()?;
    Ok(())
}

fn listen() -> Result<()> {

    let options = Options::from_args();
    let endpoint: TcpAddr = format!("127.0.0.1:{}", radio_chat::RADIO_PORT).try_into()?;
    let group: Group = options.identity.try_into()?;
    let dish = DishBuilder::new().connect(endpoint).join(group).build()?;

    loop {
        let message = dish.recv_msg()?;
        println!("{}", message.to_str()?);
    }
}


fn dispatch_line(line : &str, client : &Client) -> Result<()> {

    let options = Options::from_args();
    let message: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
    let personnes: Vec<String> = message[0].split_whitespace().map(|s| s.to_string()).collect();
    let message_obj = ContentsMessage {
        sender : options.identity,
        recipients : personnes,
        payload : message[1].clone()
    };
    let serialized_message = serde_json::to_string(&message_obj)?;
    client.send(serialized_message)?;
    Ok(())
}

fn write() -> Result<()> {
    let endpoint: TcpAddr = format!("127.0.0.1:{}", radio_chat::SERVER_PORT).try_into()?;
    let client = ClientBuilder::new().connect(endpoint).build()?;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        dispatch_line(&line.unwrap(), &client)?;
    }
    
    Ok(())
}

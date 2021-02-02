use std::convert::TryInto;
use std::io::{self, BufRead};
use std::net::TcpStream;

use libzmq::{prelude::*, RadioBuilder, Msg, Radio, TcpAddr, Group, DishBuilder};
use structopt::StructOpt;

use radio_chat::{self, Result, ContentsMessage};
use std::thread;

const MIN_PORT: u16 = 12300;
const MAX_PORT: u16 = 12400;

#[derive(StructOpt)]
struct Options {
    identity: String,
    port : u16
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
    let identity = options.identity;
    
    let group: Group = identity.clone().try_into()?;
    let dish = DishBuilder::new().join(group).build()?;
    
    for i in MIN_PORT..MAX_PORT {

            let endpoint_dish: TcpAddr = format!("127.0.0.1:{}", i).try_into()?;

            if dish.connect(endpoint_dish).is_ok() {
                println!("ports ouverts :  {}", i);
            }
    }

    loop {
        let message = dish.recv_msg()?;
        println!("{}", message.to_str()?);
    }

    Ok(())

}


fn dispatch_line(line: &str, identity: &str, radio: &Radio) -> Result<()> {

    let message: Vec<String> = line.split(':').map(|s| s.to_string()).collect();
    let personnes: Vec<String> = message[0].split_whitespace().map(|s| s.to_string()).collect();
    let message_obj = ContentsMessage {
        sender : identity.to_string(),
        recipients : personnes,
        payload : message[1].clone()
    };

    let mut message_radio = Msg::from(format!("Message de {} : {}", message_obj.sender, message_obj.payload));

    for i in message_obj.recipients {
        let group: Group = i.try_into()?;
        message_radio.set_group(group);
        radio.send(message_radio.clone())?;
        println!("message envoyé")
    }
    
    Ok(())
}


fn write() -> Result<()> {

    let options = Options::from_args();
    let endpoint_radio: TcpAddr = format!("0.0.0.0:{}", options.port).try_into()?;
    let radio = RadioBuilder::new().bind(endpoint_radio).build()?;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        dispatch_line(&line.unwrap(), &options.identity, &radio)?;
    }
    
    Ok(())
}
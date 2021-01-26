use std::convert::TryInto;

use libzmq::{prelude::*, Msg, ServerBuilder, RadioBuilder, Radio, Group, TcpAddr};

use radio_chat::{self, Result, ContentsMessage};

fn main() -> Result<()> {

    serve()?;
    Ok(())
    
}

fn handle_request (request : ContentsMessage, radio : &Radio) -> Result<()>{

    let mut message = Msg::from(format!("Message de {} : {}", request.sender, request.payload));

    for i in request.recipients {
        let group: Group = i.try_into()?;
        message.set_group(group);
        radio.send(message.clone())?;
    }
    
    Ok(())

}

fn serve () -> Result<()>{

    let endpoint_server: TcpAddr = format!("0.0.0.0:{}", radio_chat::SERVER_PORT).try_into()?;
    let server = ServerBuilder::new().bind(endpoint_server).build()?;

    let endpoint_radio: TcpAddr = format!("0.0.0.0:{}", radio_chat::RADIO_PORT).try_into()?;
    let radio = RadioBuilder::new().bind(endpoint_radio).build()?;

    loop {
        let received_message = server.recv_msg()?;
        let message = received_message.to_str()?;
        println!("Message arrivant : {}", message);

        let message_deserialize = serde_json::from_str::<ContentsMessage>(message)?;
        
        handle_request(message_deserialize, &radio)?;
    }
}
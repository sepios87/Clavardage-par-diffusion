use std::convert::TryInto;

use libzmq::{prelude::*, Msg, ServerBuilder, RadioBuilder, Group, TcpAddr};

fn main() -> Result<()> {

    serve()?;
    Ok(())
    
}

fn handle_request(request : ContentsMessage, radio: &Radio) -> Result<()>{
    let mut message = serde_json::from_str(ContentsMessage);
    let group: Group = "Limoges".try_into()?;
    message.set_group(group);
    radio.send(message.clone())?;
    thread::sleep(Duration::from_secs(1));
}

fn serve () -> Result<()>{
    let endpoint: TcpAddr = format!("0.0.0.0:{}", examples::SERVER_PORT).try_into()?;
    let server = ServerBuilder::new().bind(endpoint).build()?;
    let radio = RadioBuilder::new().bind(endpoint).build()?;

    loop {

        let received_message = server.recv_msg()?;

        handle_request(received_message, radio);

        //println!("Message arrivant : {}", received_message.to_str()?);
        //let mut message_to_send = Msg::from(PONG);
        //let client_id = received_message.routing_id().expect("Id. client");
        //message_to_send.set_routing_id(client_id);
        //server.send(message_to_send)?;
    }
}
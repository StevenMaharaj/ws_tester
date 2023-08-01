use std::error::Error;

// use log::info;
use tungstenite::{connect, Message};
use url::Url;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let public_endpoint = "wss://ws.okx.com:8443/ws/v5/public";
    // env_logger::init();
    // log::info!("Starting up");

    let url = Url::parse(public_endpoint).expect("not a valid url");
    let (mut socket, _res) = connect(url)?;

    let mess = r#"{ "op": "subscribe", "args": [  {"channel": "bbo-tbt","instId": "ETH-USDT"} ,{ "channel": "bbo-tbt","instId": "BTC-USDT" } ] }"#;
    socket
        .send(Message::Text(mess.to_string()))
        .expect("Error sending message");
    println!("Sent: {}", mess);
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
        // info!("Received: {}", msg);
    }
}

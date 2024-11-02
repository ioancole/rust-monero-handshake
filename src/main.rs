mod network;
mod constants;
mod protocol;
mod utils;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use rand::Rng;
use crate::constants::MAINNET_ADDRS;
use crate::network::BucketHead2;
use crate::protocol::{
    get_handshake_body_bytes,
    deserialize_body_response,
    test_parse_object_2,
    get_header_bytes
};

use crate::constants::COMMAND_HANDSHAKE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_parse_object_2();


    let body_bytes = get_handshake_body_bytes();
    let header_bytes = get_header_bytes(COMMAND_HANDSHAKE, body_bytes.len() as u64);

    println!("{:?}", body_bytes);
    // Ok(())

    let mut stream = TcpStream::connect(MAINNET_ADDRS[0]).await?;
    
    println!("body data bytes: \n{:?}\n\n", &body_bytes);
    
    
    let mut message_bytes = header_bytes;
    message_bytes.extend(body_bytes);
    
    println!("Sending: \n{:?}\n\n", &message_bytes);
    
    stream.write_all(&message_bytes).await?;
    
    // // Read the response
    let mut full_buffer: Vec<u8> = Vec::new();
    
    for i in 0..3 {
        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
    
        println!("\n{:?}", i);
        println!("Received: \n{:?}\n\n", &n);
        println!("\n{:?}\n\n", &buffer);
        if n > 0 {
            let mut a = (&buffer[0..n]).to_vec();
            full_buffer.append(&mut a);
        }
    }
    
    println!("Received: \n{:?}\n\n", &full_buffer.len());
    println!("Received: \n{:?}\n\n", &full_buffer);
    
    Ok(())

    // buffer = vec![0; 1024];
    // let n = stream.read(&mut buffer).await?;
    
    // println!("Received: \n{:?}\n\n", &buffer[..n]);
    
    // println!("\n\n\n");
    
    // // The header byte length is fixed at 33 bytes
    // let header_response_data = &buffer[..33];
    // let header_response: BucketHead2 = bincode::deserialize(header_response_data)?;
    
    // println!("\nReceived response from node {:?}:", TESTNET_ADDRS[0]);
    // println!("Protocol version: {:?}", header_response.m_protocol_version);
    // println!("Response length: {:?}", header_response.m_cb);
    // println!("Response command type: {:?}", header_response.m_command);
    
    
    // // The rest of the response is the body
    // let body_response_data = &buffer[33..n];
    
    // match deserialize_body_response(body_response_data) {
    //     Ok(deserialized_response) => println!("Deserialized body response: {:?}", deserialized_response),
    //     Err(e) => println!("Failed to deserialize body response: {:?}", e),
    // }
    
    
    // println!("{:?}", "asdfasdfasdfadsfdas".as_bytes());
    // Ok(())
}

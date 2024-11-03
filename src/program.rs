use crate::constants::*;

use crate::protocol::{get_handshake_body_bytes, get_header_bytes};

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bincode;

use std::convert::TryInto;
use std::io::{Cursor};
use std::{thread, time};
use std::time::{Instant, Duration};
use crate::network::BucketHead2;
use crate::constants::LEVIN_PROTOCOL_SIGNATURE;
use crate::protocol::deserialize_body_response;

pub async fn run_program(chain: u32) -> Result<(), Box<dyn std::error::Error>> {
    let addresses: Vec<&str>;

    let env_name: String;
    let network_id: Vec<u8>;

    match chain {
        1 => {
            env_name = "MainNet".to_string();
            addresses = MAINNET_ADDRS.to_vec();
            network_id = MAINNET_NETWORK_ID.to_vec();
        },
        2 => {
            env_name = "TestNet".to_string();
            addresses = TESTNET_ADDRS.to_vec();
            network_id = TESTNET_NETWORK_ID.to_vec();
        },
        3 => {
            env_name = "StageNet".to_string();
            addresses = STAGENET_ADDRS.to_vec();
            network_id = STAGENET_NETWORK_ID.to_vec();
        },
        _ => {
            println!("Invalid chain");
            return Ok(());
        }
    }

    println!("\nConnecting to Monero {} chain...", env_name);

    let mut stream: Option<TcpStream> = None;
    
    for i in 0..addresses.len() {
        println!("Connecting to seed node: {}\n", addresses[i]);
        match TcpStream::connect(addresses[i]).await {
            Ok(s) => {
                stream = Some(s);
                break;
            },
            Err(e) => {
                println!("Error connecting to seed node: {}", e);
            }
        }
    }

    let mut stream = match stream {
        Some(s) => s,
        None => {
            println!("Failed to connect to any seed node.");
            return Ok(());
        }
    };

    if let Err(e) = send_handshake(network_id, &mut stream).await {
        eprintln!("Problem sending handshake: {}", e);
    }

    if let Err(e) = read_responses(&mut stream).await {
        eprintln!("Receiving responses: {}", e);
    }

    Ok(())
}

pub async fn send_handshake(network_id: Vec<u8>, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let body_bytes = get_handshake_body_bytes(network_id);
    let header_bytes = get_header_bytes(COMMAND_HANDSHAKE, body_bytes.len() as u64);
    let mut message_bytes = header_bytes;
    message_bytes.extend(body_bytes);

    println!("Sending HANDSHAKE REQUEST to node...\n");

    stream.write_all(&message_bytes).await?;
    Ok(())
}

pub async fn read_responses(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut message_cursor = Cursor::new(Vec::new());

    let now = Instant::now();

    loop {
        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
        message_cursor.write_all(&buffer[0..n]).await?;

        let messages_end_index = process_current_buffer(&message_cursor.get_ref())?;
        
        if messages_end_index > 0 {
            let a = message_cursor.get_ref()[messages_end_index..].to_vec();
            message_cursor = Cursor::new(Vec::new());
            message_cursor.write_all(&a).await;
        }

        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        // Wait two seconds for responses
        if now.elapsed() > Duration::from_secs(2) {
            break;
        }
    }
    
    Ok(())
}

fn process_current_buffer(buffer: &Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {

    if buffer.len() < LEVIN_HEADER_BYTE_LENGTH {
        // Incomplete header
        return Ok(0);
    }

    let mut bytes_consumed = 0;
    let mut message_index = 0;

    loop {
        let header_start_index = message_index;
        let signature_end_index = message_index + 8;

        if signature_end_index > buffer.len() {
            break;
        }

        let signature_bytes: [u8; 8] = buffer[header_start_index..signature_end_index].try_into().expect("slice with incorrect length");
        let signature_value = u64::from_le_bytes(signature_bytes);

        if signature_value != LEVIN_PROTOCOL_SIGNATURE {
            println!("Received invalid signature: {:?}, should be {:?}", signature_value, LEVIN_PROTOCOL_SIGNATURE);
            break;
        }

        let header_end_index = header_start_index + LEVIN_HEADER_BYTE_LENGTH;
        let header = bincode::deserialize::<BucketHead2>(&buffer[header_start_index..header_end_index])?;

        let body_start_index = header_end_index;

        let body_end_index: usize = body_start_index + header.m_cb as usize;

        if body_end_index > buffer.len() {
            break;
        } else {
            let body_bytes = &buffer[body_start_index..body_end_index];
            deserialize_body_response(body_bytes, header.m_command)?;
            message_index = body_end_index;
            bytes_consumed = body_end_index;
        }
    }

    Ok(bytes_consumed)
}
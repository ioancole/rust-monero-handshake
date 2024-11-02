use crate::constants::*;

use crate::protocol::{get_handshake_body_bytes, get_header_bytes};

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;

pub async fn run_program(chain: u32) -> Result<(), Box<dyn std::error::Error>> {

    let mut seed_node_addr: String = MAINNET_ADDRS[0].to_string();
    
    let addresses: Vec<&str>;

    let mut env_name: String = "".to_string();
    let mut network_id: Vec<u8>;

    match chain {
        1 => {
            env_name = "MainNet".to_string();
            addresses = MAINNET_ADDRS.to_vec();
            network_id = MAINNET_NETWORK_ID.to_vec();;
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

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..addresses.len());
    
    let seed_node_addr = addresses[random_index];

    println!("Connecting to Monero {} chain...", env_name);

    let mut stream: Option<TcpStream> = None;
    
    for i in 0..addresses.len() {
        println!("Connecting to seed node: {}", addresses[i]);
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

    stream.write_all(&message_bytes).await?;
    Ok(())
}

pub async fn read_responses(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut full_buffer: Vec<u8> = Vec::new();
    
    for i in 0..10 {
        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
    
        // println!("\n{:?}", i);
        // println!("Received: \n{:?}\n\n", &n);
        // println!("\n{:?}\n\n", &buffer);
        if n > 0 {
            let mut a = (&buffer[0..n]).to_vec();
            full_buffer.append(&mut a);
        }
    }
    
    println!("Received: \n{:?}\n\n", &full_buffer.len());
    println!("Received: \n{:?}\n\n", &full_buffer);
    
    Ok(())
}

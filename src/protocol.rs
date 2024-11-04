
extern crate byteorder;

use std::convert::TryInto;
use std::io::{Cursor, Write};
use serde::de::Error;
use rand::Rng;

use crate::network::{BucketHead2, HandshakeResponse, HandshakeRequest, BasicNodeData, SupportFlagsRequest};
use crate::utils::u32_to_ip;

use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;
use log::{info, warn};

use crate::constants::*;

pub fn get_header_bytes(command_id: u32, body_length: u64) -> Vec<u8> {
    let header = BucketHead2 {
        m_signature: LEVIN_PROTOCOL_SIGNATURE,
        m_cb: body_length,
        m_have_to_return_data: 1,
        m_command: command_id,
        m_return_code: 0,
        m_flags: 1,
        m_protocol_version: LEVIN_PROTOCOL_VERSION,
    };
    bincode::serialize(&header).unwrap()
}

pub fn get_handshake_body_bytes(network_id: Vec<u8>) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let handshake_request = HandshakeRequest {
        node_data: BasicNodeData {
            network_id: network_id,
            my_port: 0,
            peer_id: rng.random(),
            support_flags: 0,
        }
    };

    let mut header_cursor = Cursor::new(Vec::<u8>::new());

    header_cursor.write(&SIGNATURE_A).unwrap();
    header_cursor.write(&SIGNATURE_B).unwrap();
    header_cursor.write(&[PORTABLE_STORAGE_PROTOCOL_VERSION]).unwrap();

    header_cursor.write_u8(0x04).unwrap(); // varint 1

    let node_data_key: String = "node_data".to_string();
    header_cursor.write_u8(node_data_key.len() as u8).unwrap();
    header_cursor.write(node_data_key.as_bytes()).unwrap();

    header_cursor.write_u8(SERIALIZE_TYPE_OBJECT).unwrap();
    header_cursor.write_u8(0x04).unwrap(); // varint 1 key

    let network_id_key: String = "network_id".to_string();
    header_cursor.write_u8(network_id_key.len() as u8).unwrap();
    header_cursor.write(network_id_key.as_bytes()).unwrap();

    header_cursor.write_u8(SERIALIZE_TYPE_STRING).unwrap();
    header_cursor.write_u8(0x40).unwrap(); // varint 16
    header_cursor.write(&handshake_request.node_data.network_id).unwrap();

    header_cursor.into_inner()
}

pub fn deserialize_body_response(body_response_data: &[u8], command_id: u32) -> Result<(), bincode::Error> {

    let signature1 = &body_response_data[..4];
    let signature2 = &body_response_data[4..8];
    let version = &body_response_data[8..9];

    if signature1 != SIGNATURE_A {
        return Err(bincode::Error::custom("Invalid SIGNATURE_A"));
    }

    if signature2 != SIGNATURE_B {
        return Err(bincode::Error::custom("Invalid SIGNATURE_B"));
    }

    if version != [PORTABLE_STORAGE_PROTOCOL_VERSION] {
        return Err(bincode::Error::custom("Invalid version"));
    }

    let body_bytes = &body_response_data[9..];

    match command_id {
        COMMAND_HANDSHAKE => {
            let handshake_response: HandshakeResponse = parse_handshake_response(body_bytes);
            info!("Recieved HANDSHAKE RESPONSE (1001) from node, containing {} Monero nodes:\n", handshake_response.local_peerlist_new.len());
            for i in 0..10 {
                let entry = &handshake_response.local_peerlist_new[i];
                print!("{} - ", i + 1);
                print!("{}:{}\n", u32_to_ip(entry.adr.addr.m_ip), entry.adr.addr.m_port);
            }

            println!("    ...and {} more nodes.\n", handshake_response.local_peerlist_new.len() - 10);
        },
        COMMAND_SUPPORT_FLAGS => {
            let _support_flags: SupportFlagsRequest = bincode::deserialize(body_bytes).ok().unwrap();
            info!("Received SUPPORT FLAGS REQUEST (2006) from node");
        },
        COMMAND_NEW_TRANSACTIONS => {
            info!("Received NEW TRANSACTIONS COMMAND (2002) from node");
        },
        COMMAND_REQUEST_CHAIN => {
            info!("Received REQUEST CHAIN COMMAND (2005) from node");
        },
        COMMAND_RESPONSE_CHAIN_ENTRY => {
            info!("Received REQUEST CHAIN ENTRY COMMAND (2006) from node");
        },
        _ => {
            warn!("COMMAND ID: {:?}", command_id);
            return Err(bincode::Error::custom("Invalid command ID"));
        }
    }

    Ok(())
}

pub fn parse_handshake_response(buff_bytes: &[u8]) -> HandshakeResponse {
    let mut new_bytes_cursor = Cursor::new(Vec::new());

    parse_object(0, &buff_bytes, &mut new_bytes_cursor);
    let new_bytes = new_bytes_cursor.into_inner();

    let handshake_response: HandshakeResponse = bincode::deserialize(&new_bytes).ok().unwrap();

    handshake_response
}

pub fn parse_varint(current_index: usize, buff: &[u8]) -> Vec<usize> {
    let n_keys_first_byte = buff[current_index] as usize;
    let n_bytes_indicator = (n_keys_first_byte & 0b11) + 1;
    let mut varint_bytes = buff[current_index..current_index + n_bytes_indicator].to_vec();

    for _ in 0..(4 - n_bytes_indicator) {
        varint_bytes.push(0);
    }
    let mut n = i32::from_le_bytes(varint_bytes.try_into().unwrap());

    n = n >> 2;
    vec!(n as usize, current_index + n_bytes_indicator)
}

pub fn parse_object(current_index: usize, buff: &[u8], new_bytes_cursor: &mut Cursor<Vec<u8>>) -> usize {
    let n_keys_index = current_index;

    let mut section_end = 0;

    let parsed_varint = parse_varint(n_keys_index, buff);
    let n_keys = parsed_varint[0];
    let mut obj_current_key = parsed_varint[1];

    for _n in 0..n_keys {
        let section_start = obj_current_key;
        section_end = parse_section(section_start, buff, new_bytes_cursor);
        obj_current_key = section_end;
    }

    section_end
}

pub fn parse_section(current_index: usize, buff: &[u8], new_bytes_cursor: &mut Cursor<Vec<u8>>) -> usize {
    let mut section_end = current_index;

    let key_size = buff[current_index] as usize;
    let key_start_index = current_index + 1 as usize;
    let key_end_index = key_start_index + key_size;
    let key_bytes = &buff[key_start_index..key_end_index];

    let key = String::from_utf8_lossy(key_bytes);

    let copy_value: bool;
    match key.as_ref() {
        "rpc_port" => copy_value = false,
        "rpc_credits_per_hash" => copy_value = false,
        "pruning_seed" => copy_value = false,
        "type" => copy_value = false,
        _ => copy_value = true,
    }

    let value_type_index = key_end_index;
    let mut value_type = buff[value_type_index];

    let mut value_start_index = value_type_index + 1;

    if value_type == SERIALIZE_TYPE_STRING && key == "addr" {
        value_start_index = value_start_index + 13 as usize;
        value_type = 6;
    }

    if (value_type & SERIALIZE_TYPE_ARRAY_OF_TYPES_FLAG) > 0 {
        let n_elements_index = value_type_index + 1;
        let n_elements_result = parse_varint(n_elements_index, buff);
        let n_elements = n_elements_result[0];

        let mut object_start = n_elements_result[1];

        let mut bs = [0u8; mem::size_of::<i64>()];
        bs.as_mut()
            .write_i64::<LittleEndian>(n_elements as i64)
            .expect("Unable to write");

        new_bytes_cursor.write_all(&bs).ok();

        if value_type == 0x80 | SERIALIZE_TYPE_OBJECT {
            for _n in 0..n_elements {
                section_end = parse_object(object_start, buff, new_bytes_cursor);
                object_start = section_end;
            }
        } else {
            // Parse array of primary types - not too complicated but not needed for the handshake response.
        }
    }

    match value_type {
        SERIALIZE_TYPE_INT64 => section_end = copy_value_bytes(value_start_index, 8, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_INT32 => section_end = copy_value_bytes(value_start_index, 4, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_INT16 => section_end = copy_value_bytes(value_start_index, 2, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_INT8 => section_end = copy_value_bytes(value_start_index, 1, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_UINT64 => section_end = copy_value_bytes(value_start_index, 8, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_UINT32 => section_end = copy_value_bytes(value_start_index, 4, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_UINT16 => section_end = copy_value_bytes(value_start_index, 2, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_UINT8 => section_end = copy_value_bytes(value_start_index, 1, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_STRING => {
            let value_length_index = value_start_index;

            let parsed_varint = parse_varint(value_length_index, buff);
            let value_length = parsed_varint[0];
            let value_start_index = parsed_varint[1];

            let mut bs = [0u8; 8];
            bs.as_mut()
                .write_i64::<LittleEndian>(value_length as i64)
                .expect("Unable to write");

            if copy_value {
                new_bytes_cursor.write_all(&bs).ok();
                section_end = copy_value_bytes(value_start_index, value_length, buff, new_bytes_cursor, copy_value);
            }
        },
        SERIALIZE_TYPE_BOOL => section_end = copy_value_bytes(value_start_index, 1, buff, new_bytes_cursor, copy_value),
        SERIALIZE_TYPE_OBJECT => section_end = parse_object(value_start_index, buff, new_bytes_cursor),
        _=> {
            // println!("Unrecognized type {:?}", value_type),
        }
    }

    section_end
}

fn copy_value_bytes(value_start_index: usize, value_length: usize, buff: &[u8], new_bytes_cursor: &mut Cursor<Vec<u8>>, copy_value: bool) -> usize {
    let value_end_index = value_start_index + value_length;
    let value_bytes = &buff[value_start_index..value_end_index];
    if copy_value {
        new_bytes_cursor.write_all(&value_bytes).ok();
    }
    value_end_index
}

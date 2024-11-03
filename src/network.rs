use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicNodeData {
    pub my_port: u32,
    pub network_id: Vec<u8>,
    pub peer_id: u64,
    pub support_flags: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreSyncData {
    pub cumulative_difficulty: u64,
    pub cumulative_difficulty_top64: u64,
    pub current_height: u64,
    pub top_id: [u8; 32],
    pub top_version: u8,
}

#[derive(Debug)]
pub struct HandshakeRequest {
    pub node_data: BasicNodeData
}

#[derive(Deserialize, Debug)]
pub struct HandshakeResponse {
    pub local_peerlist_new: Vec<PeerlistEntry>,
    pub node_data: BasicNodeData,
    pub payload_data: CoreSyncData,
}

#[derive(Deserialize, Debug)]
pub struct SupportFlagsRequest {
    pub support_flags: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ipv4NetworkAddress {
    pub m_ip: u32,
    pub m_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ipv4NetworkAddressWrapper {
    pub addr: Ipv4NetworkAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerlistEntry {
    pub adr: Ipv4NetworkAddressWrapper,
    pub id: u64, // PeerId
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketHead2 {
    pub m_signature: u64,
    pub m_cb: u64,
    pub m_have_to_return_data: u8,
    pub m_command: u32,
    pub m_return_code: u32,
    pub m_flags: u32,
    pub m_protocol_version: u32,
}

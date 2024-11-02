

// Hard coded seed node addresses from the Monero repository
pub const MAINNET_ADDRS: [&str; 7] = [
    "176.9.0.187:18080",
    "88.198.163.90:18080",
    "66.85.74.134:18080",
    "51.79.173.165:18080",
    "192.99.8.110:18080",
    "37.187.74.171:18080",
    "77.172.183.193:18080"
];

pub const TESTNET_ADDRS: [&str; 5] = [
    "176.9.0.187:28080",
    "51.79.173.165:28080",
    "192.99.8.110:28080",
    "37.187.74.171:28080",
    "77.172.183.193:28080",
];

pub const STAGENET_ADDRS: [&str; 5] = [
    "176.9.0.187:38080",
    "51.79.173.165:38080",
    "192.99.8.110:38080",
    "37.187.74.171:38080",
    "77.172.183.193:38080",
];

// Portable storage constants
pub const SIGNATURE_A: [u8; 4] = [0x01, 0x11, 0x01, 0x01];
pub const SIGNATURE_B: [u8; 4] = [0x01, 0x01, 0x02, 0x01];
pub const PORTABLE_STORAGE_PROTOCOL_VERSION: u8 = 0x01;


// Levin protocol constants
pub const LEVIN_PROTOCOL_SIGNATURE: u64 = 0x0101010101012101;
pub const LEVIN_PROTOCOL_VERSION: u32 = 0x01;

// Network IDs (UUID)
pub const MAINNET_NETWORK_ID: [u8; 16] = [
    0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04 , 0x41, 0x61,
    0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x10];

pub const TESTNET_NETWORK_ID: [u8; 16] = [
    0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04 , 0x41, 0x61,
    0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x11];

pub const STAGENET_NETWORK_ID: [u8; 16] = [
    0x12 ,0x30, 0xF1, 0x71, 0x61, 0x04 , 0x41, 0x61,
    0x17, 0x31, 0x00, 0x82, 0x16, 0xA1, 0xA1, 0x12];


// Commands and types as defined in https://github.com/monero-project/monero/blob/master/docs/PORTABLE_STORAGE.md
pub const COMMAND_HANDSHAKE: u32 = 1001;
pub const COMMAND_TIMED_SYNC: u32 = 1002;
pub const COMMAND_PING: u32 = 1003;
pub const COMMAND_STAT_INFO: u32 = 1004;
pub const COMMAND_NETWORK_STATE: u32 = 1005;
pub const COMMAND_PEER_ID: u32 = 1006;
pub const COMMAND_SUPPORT_FLAGS: u32 = 1007;

pub const SERIALIZE_TYPE_INT64: u8 = 1;
pub const SERIALIZE_TYPE_INT32: u8 = 2;
pub const SERIALIZE_TYPE_INT16: u8 = 3;
pub const SERIALIZE_TYPE_INT8: u8 = 4;
pub const SERIALIZE_TYPE_UINT64: u8 = 5;
pub const SERIALIZE_TYPE_UINT32: u8 = 6;
pub const SERIALIZE_TYPE_UINT16: u8 = 7;
pub const SERIALIZE_TYPE_UINT8: u8 = 8;
pub const SERIALIZE_TYPE_DOUBLE: u8 = 9;
pub const SERIALIZE_TYPE_STRING: u8 = 10;
pub const SERIALIZE_TYPE_BOOL: u8 = 11;
pub const SERIALIZE_TYPE_OBJECT: u8 = 12;
pub const SERIALIZE_TYPE_ARRAY: u8 = 13;

















// COMMAND_HANDSHAKE
// COMMAND_TIMED_SYNC
// COMMAND_PING
// COMMAND_STAT_INFO: u31004;
// COMMAND_NETWORK_STATE = 1005;
// COMMAND_PEER_ID: u32 6;
// COMMAND_SUPPORT_FLAGS = 1007;
// SERIALIZE_TYPE_INT64: 1;
// SERIALIZE_TYPE_INT32: 2;
// SERIALIZE_TYPE_INT16: 3;
// SERIALIZE_TYPE_INT8: 4;
// SERIALIZE_TYPE_UINT64;
// SERIALIZE_TYPE_UINT32;
// SERIALIZE_TYPE_UINT16;
// SERIALIZE_TYPE_UINT8,
// SERIALIZE_TYPE_DOUBLE;
// SERIALIZE_TYPE_STRING;
// SERIALIZE_TYPE_BOOL: 
// SERIALIZE_TYPE_OBJECT2;
// SERIALIZE_TYPE_ARRAY:;

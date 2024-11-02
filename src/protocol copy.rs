
pub trait Protocol {
    fn get_version(&self) -> u32;
    fn get_handshake(&self) -> HandshakeRequest;
    fn get_handshake_response(&self) -> HandshakeResponse;
    fn get_request(&self) -> RequestMessage;
    fn get_response(&self) -> ResponseMessage;
    fn get_ping(&self) -> PingMessage;
    fn get_pong(&self) -> PongMessage;
    fn get_block(&self) -> BlockMessage;
    fn get_block_request(&self) -> BlockRequestMessage;
    fn get_block_response(&self) -> BlockResponseMessage;
    fn get_tx(&self) -> TxMessage;
    fn get_tx_request(&self) -> TxRequestMessage;
    fn get_tx_response(&self) -> TxResponseMessage;
    fn get_peerlist_request(&self) -> PeerlistRequestMessage;
    fn get_peerlist_response(&self) -> PeerlistResponseMessage;
    fn get_peerlist(&self) -> PeerlistMessage;
    fn get_peerlist_diff_request(&self) -> PeerlistDiffRequestMessage;
    fn get_peerlist_diff_response(&self) -> PeerlistDiffResponseMessage;
    fn get_peerlist_diff(&self) -> PeerlistDiffMessage;
    fn get_core_sync_request(&self) -> CoreSyncRequestMessage;
    fn get_core_sync_response(&self) -> CoreSyncResponseMessage;
    fn get_core_sync(&self) -> CoreSyncMessage;
    fn get_core_sync_diff_request(&self) -> CoreSyncDiffRequestMessage;
    fn get_core_sync_diff_response(&self) -> CoreSyncDiffResponseMessage;
    fn get_core_sync_diff(&self) -> CoreSyncDiffMessage;
    fn get_core_sync_diff_data_request(&self) -> CoreSyncDiffDataRequestMessage;
    fn get_core_sync_diff_data_response(&self) -> CoreSyncDiffDataResponseMessage;
    fn get_core_sync_diff_data(&self) -> CoreSyncDiffDataMessage;
    fn get_core_sync_data_request(&self) -> CoreSyncDataRequestMessage;
    fn get_core_sync_data_response(&self) -> CoreSyncDataResponseMessage;
    fn get_core_sync_data(&self) -> CoreSyncDataMessage;
    fn get_core_sync_data_diff_request(&self) -> CoreSyncDataDiffRequestMessage;
    fn get_core_sync_data_diff_response(&self) -> CoreSyncDataDiffResponseMessage;
    fn get_core_sync_data_diff(&self) -> CoreSyncData;
}

struct ProtocolA impl Protocol {
    
}
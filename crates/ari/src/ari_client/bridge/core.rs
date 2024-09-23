use crate::*;

impl AriClient {
    const BRIDGE_PATH: &str = "bridges";
    const ADD_CHANNEL_PATH: &str = "addChannel";
    const REMOVE_CHANNEL_PATH: &str = "removeChannel";

    /// Create a new bridge.
    pub async fn bridge_create(&self) -> AriClientResult<Bridge> {
        self.authorized_post_json_response([Self::BRIDGE_PATH], ()).await
    }

    /// Shut down a bridge.
    pub async fn bridge_destroy(&self, bridge_id: &BridgeId) -> AriClientResult<()> {
        self.authorized_delete([Self::BRIDGE_PATH], bridge_id).await
    }

    /// Add a channel to a bridge.
    pub async fn bridge_add_channel(&self, bridge_id: &BridgeId, params: BridgeAddChannelParams<'_>) -> AriClientResult<()> {
        self.authorized_post([Self::BRIDGE_PATH, bridge_id.as_ref(), Self::ADD_CHANNEL_PATH], params)
            .await
    }

    /// Remove a channel from a bridge.
    pub async fn bridge_remove_channel(&self, bridge_id: &BridgeId, params: BridgeRemoveChannelParams<'_>) -> AriClientResult<()> {
        self.authorized_post([Self::BRIDGE_PATH, bridge_id.as_ref(), Self::REMOVE_CHANNEL_PATH], params)
            .await
    }

    // // SUGGESTION(gibbz00): combine with bidge_create by making ID optional
    // pub async fn bridge_create_with_id(&self, _bridge_id: &BridgeId, _bridge: &Bridge) -> AriClientResult<Bridge> {
    //     unimplemented!()
    // }

    // pub async fn bridge_get(_client: &AriClient, _bridge_id: &BridgeId) -> AriClientResult<Bridge> {
    //     unimplemented!()
    // }

    // pub async fn bidge_list(&self) -> AriClientResult<Vec<Bridge>> {
    //     unimplemented!()
    // }

    // pub async fn bridge_set_channel_as_video_source(
    //     &self,
    //     _bridge_id: &BridgeId,
    //     _channel_id: &str,
    //     _video_source_id: &str,
    // ) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_unset_video_source(&self, _bridge_id: &BridgeId) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_start_moh(&self, _bridge_id: &BridgeId, _moh_class: &str) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_stop_moh(&self, _bridge_id: &BridgeId) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_play_media(&self, _bridge_id: &BridgeId, _playback: &Playback) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_stop_media(&self, _bridge_id: &BridgeId) -> AriClientResult<()> {
    //     unimplemented!()
    // }

    // pub async fn bridge_start_recording(&self, _bridge_id: &BridgeId, _recording: &LiveRecording) -> AriClientResult<()> {
    //     unimplemented!()
    // }
}

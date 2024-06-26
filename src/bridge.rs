use crate::Result;
use crate::{client::Client, playback::Playback, recording::LiveRecording};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub id: String,
}

impl Bridge {
    #[instrument]
    pub async fn destroy(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn add_channel(&self, _client: &Client, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn remove_channel(&self, _client: &Client, _channel_id: &str) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn set_channel_as_video_source(
        &self,
        _client: &Client,
        _channel_id: &str,
        _video_source_id: &str,
    ) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn unset_video_source(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn start_moh(&self, _client: &Client, _moh_class: &str) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn stop_moh(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn play_media(&self, _client: &Client, _playback: &Playback) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn stop_media(&self, _client: &Client) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn start_recording(
        &self,
        _client: &Client,
        _recording: &LiveRecording,
    ) -> Result<()> {
        unimplemented!()
    }

    #[instrument]
    pub async fn list_bridges(_client: &Client) -> Result<Vec<Bridge>> {
        unimplemented!()
    }

    #[instrument]
    pub async fn create_bridge(_client: &Client, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }

    #[instrument]
    pub async fn create_bridge_with_id(
        _client: &Client,
        _bridge_id: &str,
        _bridge: &Bridge,
    ) -> Result<Bridge> {
        unimplemented!()
    }

    #[instrument]
    pub async fn get_bridge(_client: &Client, _bridge_id: &str) -> Result<Bridge> {
        unimplemented!()
    }
}

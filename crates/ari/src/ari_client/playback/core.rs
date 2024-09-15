use crate::*;

impl AriClient {
    pub async fn playback_get(&self, _playback_id: &PlaybackId) -> AriClientResult<Playback> {
        unimplemented!()
    }

    pub async fn playback_control(&self, _playback_id: &PlaybackId, _operation: Operation) -> AriClientResult<()> {
        unimplemented!()
    }

    pub async fn playback_stop(&self, _playback_id: &PlaybackId) -> AriClientResult<()> {
        unimplemented!()
    }
}

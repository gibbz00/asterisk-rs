use crate::*;

impl AriClient {
    const PLAYBACK_PATH: &str = "playbacks";

    pub async fn playback_stop(&self, playback_id: &PlaybackId) -> AriClientResult<()> {
        self.authorized_delete([Self::PLAYBACK_PATH], playback_id).await
    }

    // pub async fn playback_get(&self, _playback_id: &PlaybackId) -> AriClientResult<Playback> {
    //     unimplemented!()
    // }

    // pub async fn playback_control(&self, _playback_id: &PlaybackId, _operation: Operation) -> AriClientResult<()> {
    //     unimplemented!()
    // }
}

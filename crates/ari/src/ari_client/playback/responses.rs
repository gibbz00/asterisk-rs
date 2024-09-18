use derive_getters::Getters;
use serde::Deserialize;

use crate::*;

/// Object representing the playback of media to a channel
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Playback {
    /// ID for this playback operation
    pub id: PlaybackId,
    /// The URI for the media currently being played back
    pub media_uri: String,
    /// If a list of URIs is being played, the next media URI to be played back
    pub next_media_uri: Option<String>,
    /// URI for the channel or bridge to play the media on
    pub target_uri: String,
    /// For media types that support multiple languages, the language requested for playback
    pub language: Option<String>,
    /// Current state of the playback operation
    pub state: PlaybackState,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackState {
    Queued,
    Playing,
    Continuing,
    Done,
    Failed,
}

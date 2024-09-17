use chrono::{DateTime, Utc};
use derive_more::derive::Deref;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
// Boxing to accommodate for the large size differences between the variants
pub enum AsteriskEvent {
    StasisStart(Box<ChannelEvent<StasisStart>>),
    StasisEnd(Box<ChannelEvent<()>>),
    ChannelCreated(Box<ChannelEvent<()>>),
    ChannelDestroyed(Box<ChannelEvent<ChannelDestroyed>>),
    ChannelHold(Box<ChannelEvent<ChannelHold>>),
    ChannelUnhold(Box<ChannelEvent<()>>),
    ChannelToneDetected(Box<ChannelEvent<()>>),
    ChannelHangupRequest(Box<ChannelEvent<ChannelHangupRequest>>),
    ChannelDialplan(Box<ChannelEvent<ChannelDialplan>>),
    ChannelStateChange(Box<ChannelEvent<()>>),
    ChannelDtmfReceived(Box<ChannelEvent<ChannelDtmfReceived>>),
    ChannelVarset(Box<ChannelEvent<ChannelVarset>>),
    DeviceStateChanged(Box<Event<DeviceStateChanged>>),
    PlaybackStarted(Box<PlaybackEvent<()>>),
    PlaybackFinished(Box<PlaybackEvent<()>>),
}

#[derive(Debug, Clone, Deserialize, Deref)]
pub struct Event<D> {
    pub asterisk_id: String,
    pub application: String,
    #[serde(with = "serde_utils::ari_date_format")]
    pub timestamp: DateTime<Utc>,
    #[deref]
    #[serde(flatten)]
    pub data: D,
}

#[derive(Debug, Clone, Deserialize, Deref)]
pub struct ChannelEvent<D> {
    pub channel: Channel,
    #[deref]
    #[serde(flatten)]
    pub inner: Event<D>,
}

#[derive(Debug, Clone, Deserialize, Deref)]
pub struct PlaybackEvent<D> {
    pub playback: Playback,
    #[deref]
    #[serde(flatten)]
    pub inner: Event<D>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StasisStart {
    pub args: Vec<String>,
    pub replace_channel: Option<Channel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelDestroyed {
    /// Integer representation of the cause of the hangup
    pub cause: i32,
    /// Text representation of the cause of the hangup
    pub cause_txt: String,
}

/// Channel initiated a media hold
#[derive(Debug, Clone, Deserialize)]
pub struct ChannelHold {
    /// The music on hold class that the initiator requested.
    #[serde(rename = "musicclass")]
    pub music_class: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelVarset {
    pub variable: String,
    pub value: String,
    pub channel: Option<Channel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelHangupRequest {
    pub soft: Option<bool>,
    pub cause: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelDialplan {
    pub dialplan_app: String,
    pub dialplan_app_data: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceStateChanged {
    pub device_state: DeviceState,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceState {
    pub name: String,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelDtmfReceived {
    /// DTMF digit received (0-9, A-E, # or *)
    // IMPROVEMENT: typeset
    pub digit: char,
    pub duration_ms: i32,
}

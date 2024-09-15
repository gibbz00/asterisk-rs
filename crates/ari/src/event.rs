use chrono::{DateTime, Utc};
use derive_getters::Getters;
use derive_more::derive::Deref;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum AsteriskEvent {
    StasisStart(ChannelEvent<StasisStart>),
    StasisEnd(ChannelEvent<()>),
    ChannelCreated(ChannelEvent<()>),
    ChannelDestroyed(ChannelEvent<ChannelDestroyed>),
    ChannelHold(ChannelEvent<ChannelHold>),
    ChannelUnhold(ChannelEvent<()>),
    ChannelToneDetected(ChannelEvent<()>),
    ChannelHangupRequest(ChannelEvent<ChannelHangupRequest>),
    ChannelDialplan(ChannelEvent<ChannelDialplan>),
    ChannelStateChange(ChannelEvent<()>),
    ChannelDtmfReceived(ChannelEvent<ChannelDtmfReceived>),
    ChannelVarset(ChannelEvent<ChannelVarset>),
    DeviceStateChanged(Event<DeviceStateChanged>),
    PlaybackStarted(PlaybackEvent<()>),
    PlaybackFinished(PlaybackEvent<()>),
}

#[derive(Debug, Deserialize, Getters, Deref)]
#[serde(rename_all = "snake_case")]
pub struct Event<D> {
    asterisk_id: String,
    application: String,
    timestamp: DateTime<Utc>,
    #[deref]
    #[getter(skip)]
    #[serde(flatten)]
    data: D,
}

#[derive(Debug, Deserialize, Getters, Deref)]
#[serde(rename_all = "snake_case")]
pub struct ChannelEvent<D> {
    channel: Channel,
    #[deref]
    #[getter(skip)]
    #[serde(flatten)]
    event: Event<D>,
}

#[derive(Debug, Deserialize, Getters, Deref)]
#[serde(rename_all = "snake_case")]
pub struct PlaybackEvent<D> {
    playback: Playback,
    #[deref]
    #[getter(skip)]
    #[serde(flatten)]
    event: Event<D>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    args: Vec<String>,
    replace_channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    /// Integer representation of the cause of the hangup
    cause: i32,
    /// Text representation of the cause of the hangup
    cause_txt: String,
}

/// Channel initiated a media hold
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHold {
    /// The music on hold class that the initiator requested.
    #[serde(rename = "musicclass")]
    music_class: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVarset {
    variable: String,
    value: String,
    channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHangupRequest {
    soft: Option<bool>,
    cause: i32,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    dialplan_app: String,
    dialplan_app_data: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceStateChanged {
    device_state: DeviceState,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct DeviceState {
    name: String,
    state: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    /// DTMF digit received (0-9, A-E, # or *)
    // IMPROVEMENT: typeset
    digit: char,
    duration_ms: i32,
}

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use derive_more::derive::Deref;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum AsteriskEvent {
    StasisStart(Box<Event<StasisStart>>),
    StasisEnd(Event<StasisEnd>),
    ChannelCreated(Event<ChannelCreated>),
    ChannelDestroyed(Event<ChannelDestroyed>),
    ChannelHold(Event<ChannelHold>),
    ChannelUnhold(Event<ChannelUnhold>),
    ChannelToneDetected(Event<ChannelToneDetected>),
    ChannelVarset(Event<ChannelVarset>),
    ChannelHangupRequest(Event<ChannelHangupRequest>),
    ChannelDialplan(Event<ChannelDialplan>),
    ChannelStateChange(Event<ChannelStateChange>),
    ChannelDtmfReceived(Event<ChannelDtmfReceived>),
    DeviceStateChanged(Event<DeviceStateChanged>),
    PlaybackStarted(Event<PlaybackStarted>),
    PlaybackFinished(Event<PlaybackFinished>),
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

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisStart {
    args: Vec<String>,
    channel: Channel,
    replace_channel: Option<Channel>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct StasisEnd {
    channel: Channel,
}

/// Notification that a channel has been created
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreated {
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDestroyed {
    /// Integer representation of the cause of the hangup
    cause: i32,
    /// Text representation of the cause of the hangup
    cause_txt: String,
    channel: Channel,
}

/// Channel initiated a media hold
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelHold {
    channel: Channel,
    /// The music on hold class that the initiator requested.
    #[serde(rename = "musicclass")]
    music_class: Option<String>,
}

/// Channel initiated a media unhold
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelUnhold {
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelToneDetected {
    channel: Channel,
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
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDialplan {
    dialplan_app: String,
    dialplan_app_data: String,
    channel: Channel,
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
pub struct ChannelStateChange {
    channel: Channel,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelDtmfReceived {
    /// DTMF digit received (0-9, A-E, # or *)
    // IMPROVEMENT: typeset
    digit: char,
    duration_ms: i32,
    channel: Channel,
}

/// Event showing the start of a media playback operation
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct PlaybackStarted {
    playback: Playback,
}

/// Event showing the completion of a media playback operation
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct PlaybackFinished {
    playback: Playback,
}

use serde::Serialize;

use crate::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeAddChannelParams<'a> {
    /// Ids of channels to add to bridge, at least one is required
    #[serde(rename = "channel", serialize_with = "join_serialize")]
    pub channels: &'a [&'a ChannelId],
    /// Channel's role in the bridge
    // IMPROVEMENT: typeset as enum?
    pub role: Option<&'a str>,
    /// Absorb DTMF coming from this channel, preventing it to pass through to the bridge
    #[serde(default, rename = "absorbDTMF")]
    pub absorb_dtmf: bool,
    /// Mute audio from this channel, preventing it to pass through to the bridge
    #[serde(default)]
    pub mute: bool,
    /// Do not present the identity of the newly connected channel to other bridge members
    #[serde(default)]
    pub inhibit_connected_line_updates: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeRemoveChannelParams<'a> {
    /// Ids of channels to add to bridge, at least one is required
    #[serde(rename = "channel", serialize_with = "join_serialize")]
    pub channels: &'a [&'a ChannelId],
}

use std::collections::HashMap;

use derive_getters::Getters;
use serde::Deserialize;
use serde_with::NoneAsEmptyString;

/// A specific communication connection between Asterisk and an Endpoint.
#[serde_with::serde_as]
#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Channel {
    /// Unique identifier of the channel.
    ///
    /// This is the same as the Uniqueid field in AMI.
    id: String,
    /// Protocol ID from underlying channel driver
    ///
    /// Call-ID for chan_pjsip; will be empty if not applicable or not implemented by driver.
    #[serde_as(as = "NoneAsEmptyString")]
    protocol_id: Option<String>,
    /// Name of the channel, ex. 'SIP/foo-0000a7e3'
    name: String,
    state: ChannelState,
    caller: CallerId,
    connected: CallerId,
    #[serde(rename = "accountcode")]
    account_code: String,
    /// Current location in the dialplan
    dialplan: Dialplan,
    #[serde(rename = "creationtime")]
    creation_time: String,
    language: String,
    #[serde(default, rename = "channelvars")]
    variables: HashMap<String, String>,
    #[serde(rename = "tenantid")]
    tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum ChannelState {
    Down,
    Rsrved,
    OffHook,
    Dialing,
    Ring,
    Ringing,
    Up,
    Busy,
    #[serde(rename = "Dialing Offhook")]
    DialingOffhook,
    #[serde(rename = "Pre-ring")]
    PreRing,
    Unknown,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct RtpStatistics {
    id: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct ChannelVariable {
    id: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct CallerId {
    name: String,
    number: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "snake_case")]
pub struct Dialplan {
    context: String,
    exten: String,
    priority: i32,
    app_name: String,
    app_data: String,
}

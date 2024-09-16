use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::NoneAsEmptyString;

use crate::*;

/// A specific communication connection between Asterisk and an Endpoint.
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
    /// Unique identifier of the channel.
    ///
    /// This is the same as the Uniqueid field in AMI.
    pub id: ChannelId,
    /// Protocol ID from underlying channel driver
    ///
    /// Call-ID for chan_pjsip; will be empty if not applicable or not implemented by driver.
    #[serde_as(as = "NoneAsEmptyString")]
    pub protocol_id: Option<String>,
    /// Name of the channel, ex. 'SIP/foo-0000a7e3'
    pub name: String,
    pub state: ChannelState,
    pub caller: CallerId,
    pub connected: CallerId,
    #[serde(rename = "accountcode")]
    pub account_code: String,
    /// Current location in the dialplan
    pub dialplan: Dialplan,
    #[serde(rename = "creationtime", with = "serde_utils::ari_date_format")]
    pub creation_time: DateTime<Utc>,
    pub language: String,
    #[serde(default, rename = "channelvars")]
    pub variables: HashMap<String, String>,
    #[serde(rename = "tenantid")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct RtpStatistics {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ChannelVariable {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CallerId {
    pub name: String,
    pub number: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dialplan {
    pub context: String,
    pub exten: String,
    pub priority: i32,
    pub app_name: String,
    pub app_data: String,
}

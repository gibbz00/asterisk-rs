use derive_more::derive::AsRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AsRef)]
#[serde(transparent)]
pub struct ChannelId(String);

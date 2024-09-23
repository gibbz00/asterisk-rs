use derive_getters::Getters;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize, Getters)]
pub struct Bridge {
    id: BridgeId,
}

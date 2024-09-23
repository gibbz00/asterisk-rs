use derive_getters::Getters;
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize, Getters)]
pub struct Bridge {
    id: BridgeId,
    // TODO: add remaining fields:
    // https://docs.asterisk.org/Asterisk_22_Documentation/API_Documentation/Asterisk_REST_Interface/Asterisk_REST_Data_Models/
}

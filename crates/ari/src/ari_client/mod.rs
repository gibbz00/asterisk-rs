pub(crate) type AriClientResult<T> = Result<T, AriClientError>;

mod core;
pub use core::{AriClient, AriClientError};

mod bridge;
pub use bridge::*;

mod channel;
pub use channel::*;

mod playback;
pub use playback::*;

mod recording;
pub use recording::*;

mod serde_utils;
pub(crate) use serde_utils::*;

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Display, EnumString, Serialize, Deserialize, Clone)]
pub enum SpotifyClientScope {
    #[strum(serialize = "user-read-private")]
    UserReadPrivate
}
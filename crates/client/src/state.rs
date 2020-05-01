//! Representation of the State of a Zookeeper Server.

use super::result::ZK4LWError;
use std::str::FromStr;
use std::fmt;

const STATE_LEADER: &'static str = "leader";
const STATE_FOLLOWER: &'static str = "follower";
const STATE_STANDALONE: &'static str = "standalone";

/// The state of a Zookeeper server, as reported for example by the Monitor command
pub enum ZK4LWServerState {
    LEADER,
    FOLLOWER,
    STANDALONE,
}

impl FromStr for ZK4LWServerState {
    type Err = ZK4LWError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STATE_LEADER => Ok(ZK4LWServerState::LEADER),
            STATE_FOLLOWER => Ok(ZK4LWServerState::FOLLOWER),
            STATE_STANDALONE => Ok(ZK4LWServerState::STANDALONE),
            _ => Err(ZK4LWError::ParseStringError(s.to_string())),
        }
    }
}

impl fmt::Debug for ZK4LWServerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ZK4LWServerState::LEADER => write!(f, "{}", STATE_LEADER),
            ZK4LWServerState::FOLLOWER => write!(f, "{}", STATE_FOLLOWER),
            ZK4LWServerState::STANDALONE => write!(f, "{}", STATE_STANDALONE),
        }
    }
}
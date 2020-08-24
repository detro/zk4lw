//! Representation of the State of a Zookeeper Server.

use std::{default, fmt, str};

use crate::errors::*;

const STATE_LEADER: &'static str = "leader";
const STATE_FOLLOWER: &'static str = "follower";
const STATE_OBSERVER: &'static str = "observer";
const STATE_STANDALONE: &'static str = "standalone";

/// The state of a Zookeeper server, as reported for example by the Monitor command
#[derive(PartialEq)]
pub enum ZK4LWServerState {
    LEADER,
    FOLLOWER,
    OBSERVER,
    STANDALONE,
}

impl str::FromStr for ZK4LWServerState {
    type Err = ZK4LWError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            STATE_LEADER => Ok(ZK4LWServerState::LEADER),
            STATE_FOLLOWER => Ok(ZK4LWServerState::FOLLOWER),
            STATE_OBSERVER => Ok(ZK4LWServerState::OBSERVER),
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
            ZK4LWServerState::OBSERVER => write!(f, "{}", STATE_OBSERVER),
            ZK4LWServerState::STANDALONE => write!(f, "{}", STATE_STANDALONE),
        }
    }
}

impl default::Default for ZK4LWServerState {
    fn default() -> Self {
        ZK4LWServerState::STANDALONE
    }
}

//! The 4LW Monitor command. Also known as "mntr".
//!
//! This command outputs a list of variables that could be used for
//! monitoring the health of the cluster.
//!
//! Available since: ZooKeeper 3.4.0

use std::collections::HashMap;

use crate::unwrap_or_error;

use super::super::client::*;
use super::super::result::{ZK4LWError, ZK4LWResult};
use super::super::state::ZK4LWServerState;

const COMMAND: &'static str = "mntr";

/// Response to the `mntr` command
///
/// The fields here are what's defined in the docs as of 2018/02/23.
/// Additional fields are stored as strings within zk_extras.
#[derive(Debug)]
pub struct ZK4LWMonitorResponse {
    // version
    pub zk_version: String,
    // latency
    pub zk_avg_latency: i64,
    pub zk_max_latency: i64,
    pub zk_min_latency: i64,
    // packets
    pub zk_packets_received: i64,
    pub zk_packets_sent: i64,
    // connections & requests
    pub zk_num_alive_connections: i64,
    pub zk_outstanding_requests: i64,
    // state
    pub zk_server_state: ZK4LWServerState,
    // znodes
    pub zk_znode_count: i64,
    pub zk_watch_count: i64,
    pub zk_ephemerals_count: i64,
    // data size
    pub zk_approximate_data_size: i64,
    // file descriptors
    pub zk_open_file_descriptor_count: Option<i64>,
    pub zk_max_file_descriptor_count: Option<i64>,
    // followers (only for leader)
    pub zk_followers: Option<i64>,
    pub zk_synced_followers: Option<i64>,
    pub zk_pending_syncs: Option<i64>,
    // proposals (only for leader)
    pub zk_last_proposal_size: Option<i64>,
    pub zk_max_proposal_size: Option<i64>,
    pub zk_min_proposal_size: Option<i64>,
    // unknown/new fields (only for leader)
    pub zk_extras: HashMap<String, String>,
}

/// The Monitor (i.e. "mntr") command
pub struct ZK4LWMonitor;

impl ZK4LWCommand for ZK4LWMonitor {
    type Response = ZK4LWMonitorResponse;

    fn request_str() -> &'static str {
        COMMAND
    }

    fn parse_response(response: &str) -> ZK4LWResult<Self::Response> {
        let mut zk_version: Option<String> = None;
        let mut zk_avg_latency: Option<i64> = None;
        let mut zk_max_latency: Option<i64> = None;
        let mut zk_min_latency: Option<i64> = None;
        let mut zk_packets_received: Option<i64> = None;
        let mut zk_packets_sent: Option<i64> = None;
        let mut zk_num_alive_connections: Option<i64> = None;
        let mut zk_outstanding_requests: Option<i64> = None;
        let mut zk_server_state: Option<ZK4LWServerState> = None;
        let mut zk_znode_count: Option<i64> = None;
        let mut zk_watch_count: Option<i64> = None;
        let mut zk_ephemerals_count: Option<i64> = None;
        let mut zk_approximate_data_size: Option<i64> = None;
        let mut zk_open_file_descriptor_count: Option<i64> = None;
        let mut zk_max_file_descriptor_count: Option<i64> = None;

        let mut zk_followers: Option<i64> = None;
        let mut zk_synced_followers: Option<i64> = None;
        let mut zk_pending_syncs: Option<i64> = None;
        let mut zk_last_proposal_size: Option<i64> = None;
        let mut zk_max_proposal_size: Option<i64> = None;
        let mut zk_min_proposal_size: Option<i64> = None;
        let mut zk_extras = HashMap::new();

        let lines = response.lines();

        for line in lines {
            let mut iter = line.split('\t');
            match (iter.next().map(|s| s.trim()), iter.next().map(|s| s.trim())) {
                (Some(key), Some(value)) => match key {
                    "zk_version" => zk_version = Some(value.into()),
                    "zk_avg_latency" => zk_avg_latency = Some(value.parse()?),
                    "zk_max_latency" => zk_max_latency = Some(value.parse()?),
                    "zk_min_latency" => zk_min_latency = Some(value.parse()?),
                    "zk_packets_received" => zk_packets_received = Some(value.parse()?),
                    "zk_packets_sent" => zk_packets_sent = Some(value.parse()?),
                    "zk_num_alive_connections" => zk_num_alive_connections = Some(value.parse()?),
                    "zk_outstanding_requests" => zk_outstanding_requests = Some(value.parse()?),
                    "zk_server_state" => zk_server_state = Some(value.parse()?),
                    "zk_znode_count" => zk_znode_count = Some(value.parse()?),
                    "zk_watch_count" => zk_watch_count = Some(value.parse()?),
                    "zk_ephemerals_count" => zk_ephemerals_count = Some(value.parse()?),
                    "zk_approximate_data_size" => zk_approximate_data_size = Some(value.parse()?),
                    "zk_open_file_descriptor_count" => zk_open_file_descriptor_count = Some(value.parse()?),
                    "zk_max_file_descriptor_count" => zk_max_file_descriptor_count = Some(value.parse()?),

                    "zk_followers" => zk_followers = Some(value.parse()?),
                    "zk_synced_followers" => zk_synced_followers = Some(value.parse()?),
                    "zk_pending_syncs" => zk_pending_syncs = Some(value.parse()?),
                    "zk_last_proposal_size" => zk_last_proposal_size = Some(value.parse()?),
                    "zk_max_proposal_size" => zk_max_proposal_size = Some(value.parse()?),
                    "zk_min_proposal_size" => zk_min_proposal_size = Some(value.parse()?),
                    _ => {
                        zk_extras.insert(key.into(), value.into());
                    }
                },
                _ => break,
            }
        }

        Ok(Self::Response {
            // version
            zk_version: unwrap_or_error!(zk_version),
            // latency
            zk_avg_latency: unwrap_or_error!(zk_avg_latency),
            zk_max_latency: unwrap_or_error!(zk_max_latency),
            zk_min_latency: unwrap_or_error!(zk_min_latency),
            // packets
            zk_packets_received: unwrap_or_error!(zk_packets_received),
            zk_packets_sent: unwrap_or_error!(zk_packets_sent),
            // connections & requests
            zk_num_alive_connections: unwrap_or_error!(zk_num_alive_connections),
            zk_outstanding_requests: unwrap_or_error!(zk_outstanding_requests),
            // state
            zk_server_state: unwrap_or_error!(zk_server_state),
            // znodes
            zk_znode_count: unwrap_or_error!(zk_znode_count),
            zk_watch_count: unwrap_or_error!(zk_watch_count),
            zk_ephemerals_count: unwrap_or_error!(zk_ephemerals_count),
            // data size
            zk_approximate_data_size: unwrap_or_error!(zk_approximate_data_size),
            // file descriptors
            zk_open_file_descriptor_count,
            zk_max_file_descriptor_count,
            // followers (only for leader)
            zk_followers,
            zk_synced_followers,
            zk_pending_syncs,
            // proposals (only for leader)
            zk_last_proposal_size,
            zk_max_proposal_size,
            zk_min_proposal_size,
            // unknown/new fields (only for leader)
            zk_extras,
        })
    }
}

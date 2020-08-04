//! The 4LW Monitor command. Also known as "mntr".
//!
//! This command outputs a list of variables that could be used for
//! monitoring the health of the cluster.
//!
//! Available since: ZooKeeper 3.4.0

use std::collections::HashMap;

use super::{
    super::{
        client::*,
        parsing::tab_separated_bytes_to_key_value,
        result::{ZK4LWError, ZK4LWResult},
        state::ZK4LWServerState,
    },
    common::ZK4LWMetricSample,
};

const COMMAND: &'static str = "mntr";

/// Response to the `mntr` command
///
/// The fields here are what's defined in the docs as of 2018/02/23.
/// Additional fields are stored as strings within zk_extras.
#[derive(Debug, Default)]
pub struct ZK4LWMonitorResponse {
    // version
    pub version: String,
    pub build_revision: String,
    pub build_date: String,
    // latency
    pub latency: ZK4LWMetricSample,
    // packets
    pub packets_received: i64,
    pub packets_sent: i64,
    // connections & requests
    pub num_alive_connections: i64,
    pub outstanding_requests: i64,
    // state
    pub server_state: ZK4LWServerState,
    // znodes
    pub znode_count: i64,
    pub watch_count: i64,
    pub ephemerals_count: i64,
    // data size
    pub approximate_data_size: i64,
    // file descriptors
    pub open_file_descriptor_count: Option<i64>,
    pub max_file_descriptor_count: Option<i64>,
    // followers (only for leader)
    pub followers: Option<i64>,
    pub synced_followers: Option<i64>,
    pub pending_syncs: Option<i64>,
    // proposals (only for leader)
    pub last_proposal_size: Option<i64>,
    pub max_proposal_size: Option<i64>,
    pub min_proposal_size: Option<i64>,
    // unknown/unmapped fields
    pub misc: HashMap<String, String>,
}

/// The Monitor (i.e. "mntr") command
pub struct ZK4LWMonitor;

impl ZK4LWCommand for ZK4LWMonitor {
    type Response = ZK4LWMonitorResponse;

    fn request_body() -> &'static str {
        COMMAND
    }

    fn build_response(response_body: &str) -> ZK4LWResult<Self::Response> {
        // Parse response body into key/value pairs
        let response_map = tab_separated_bytes_to_key_value(response_body)?;

        // Map by key to a specific field in the response
        let mut response = ZK4LWMonitorResponse::default();
        for (key, val) in response_map.into_iter() {
            match key {
                // NOTE: `zk_version` is too "dense" with details,
                // so we split it into more useful, separate parts
                "zk_version" => {
                    // Extract the 'version'
                    let version_split: Vec<&str> =
                        val.split("-").filter(|x| !x.is_empty()).collect();
                    if version_split.len() != 2 {
                        return Err(ZK4LWError::ParseStringError(format!(
                            "Unable to parse version from string: '{}'",
                            val
                        )));
                    }
                    response.version = version_split.get(0).unwrap().trim().to_string();

                    // Extract the 'build revision' and 'build date'
                    let build_split: Vec<&str> = version_split.get(1).unwrap().split(",").collect();
                    if build_split.len() != 2 {
                        return Err(ZK4LWError::ParseStringError(format!(
                            "Unable to parse build from string: '{}'",
                            val
                        )));
                    }
                    response.build_revision = build_split.get(0).unwrap().trim().to_string();
                    response.build_date = build_split.get(1).unwrap().trim().to_string();
                }
                "zk_avg_latency" => response.latency.avg = val.parse()?,
                "zk_max_latency" => response.latency.max = val.parse()?,
                "zk_min_latency" => response.latency.min = val.parse()?,
                "zk_packets_received" => response.packets_received = val.parse()?,
                "zk_packets_sent" => response.packets_sent = val.parse()?,
                "zk_num_alive_connections" => response.num_alive_connections = val.parse()?,
                "zk_outstanding_requests" => response.outstanding_requests = val.parse()?,
                "zk_server_state" => response.server_state = val.parse()?,
                "zk_znode_count" => response.znode_count = val.parse()?,
                "zk_watch_count" => response.watch_count = val.parse()?,
                "zk_ephemerals_count" => response.ephemerals_count = val.parse()?,
                "zk_approximate_data_size" => response.approximate_data_size = val.parse()?,
                "zk_open_file_descriptor_count" => {
                    response.open_file_descriptor_count = Some(val.parse()?)
                }
                "zk_max_file_descriptor_count" => {
                    response.max_file_descriptor_count = Some(val.parse()?)
                }
                "zk_followers" => response.followers = Some(val.parse()?),
                "zk_synced_followers" => response.synced_followers = Some(val.parse()?),
                "zk_pending_syncs" => response.pending_syncs = Some(val.parse()?),
                "zk_last_proposal_size" => response.last_proposal_size = Some(val.parse()?),
                "zk_max_proposal_size" => response.max_proposal_size = Some(val.parse()?),
                "zk_min_proposal_size" => response.min_proposal_size = Some(val.parse()?),
                _ => {
                    response.misc.insert(key.into(), val.into());
                }
            }
        }

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::client::ZK4LWCommand;
    use crate::commands::mntr::ZK4LWMonitor;
    use crate::state::ZK4LWServerState::LEADER;

    #[test]
    fn should_build_response_from_zk34_monitor_response_body() {
        let mntr_34_resp_body = fs::read_to_string("../../fixtures/3.4/mntr.response").unwrap();
        let mntr_34_resp = ZK4LWMonitor::build_response(mntr_34_resp_body.as_str()).unwrap();

        assert_eq!(mntr_34_resp.version, "3.4.14");
        assert_eq!(
            mntr_34_resp.build_revision,
            "4c25d480e66aadd371de8bd2fd8da255ac140bcf"
        );
        assert_eq!(mntr_34_resp.build_date, "built on 03/06/2019 16:18 GMT");
        assert_eq!(mntr_34_resp.latency.avg, 0.0);
        assert_eq!(mntr_34_resp.latency.min, 0);
        assert_eq!(mntr_34_resp.latency.max, 0);
        assert_eq!(mntr_34_resp.packets_received, 2);
        assert_eq!(mntr_34_resp.packets_sent, 1);
        assert_eq!(mntr_34_resp.num_alive_connections, 1);
        assert_eq!(mntr_34_resp.outstanding_requests, 0);
        assert_eq!(mntr_34_resp.server_state, LEADER);
        assert_eq!(mntr_34_resp.znode_count, 4);
        assert_eq!(mntr_34_resp.watch_count, 0);
        assert_eq!(mntr_34_resp.ephemerals_count, 0);
        assert_eq!(mntr_34_resp.approximate_data_size, 27);
        assert_eq!(mntr_34_resp.open_file_descriptor_count.unwrap(), 36);
        assert_eq!(mntr_34_resp.max_file_descriptor_count.unwrap(), 1048576);
        assert_eq!(mntr_34_resp.followers.unwrap(), 4);
        assert_eq!(mntr_34_resp.synced_followers.unwrap(), 2);
        assert_eq!(mntr_34_resp.pending_syncs.unwrap(), 0);
        assert_eq!(mntr_34_resp.last_proposal_size.unwrap(), -1);
        assert_eq!(mntr_34_resp.min_proposal_size.unwrap(), -1);
        assert_eq!(mntr_34_resp.max_proposal_size.unwrap(), -1);
        assert_eq!(mntr_34_resp.misc.len(), 1);
        assert_eq!(
            mntr_34_resp
                .misc
                .get("zk_fsync_threshold_exceed_count")
                .unwrap(),
            "0"
        );
    }

    #[test]
    fn should_build_response_from_zk35_monitor_response_body() {
        let mntr_35_resp_body = fs::read_to_string("../../fixtures/3.5/mntr.response").unwrap();
        let mntr_35_resp = ZK4LWMonitor::build_response(mntr_35_resp_body.as_str()).unwrap();

        assert_eq!(mntr_35_resp.version, "3.5.7");
        assert_eq!(
            mntr_35_resp.build_revision,
            "f0fdd52973d373ffd9c86b81d99842dc2c7f660e"
        );
        assert_eq!(mntr_35_resp.build_date, "built on 02/10/2020 11:30 GMT");
        assert_eq!(mntr_35_resp.latency.avg, 0.0);
        assert_eq!(mntr_35_resp.latency.min, 0);
        assert_eq!(mntr_35_resp.latency.max, 0);
        assert_eq!(mntr_35_resp.packets_received, 2);
        assert_eq!(mntr_35_resp.packets_sent, 1);
        assert_eq!(mntr_35_resp.num_alive_connections, 1);
        assert_eq!(mntr_35_resp.outstanding_requests, 0);
        assert_eq!(mntr_35_resp.server_state, LEADER);
        assert_eq!(mntr_35_resp.znode_count, 5);
        assert_eq!(mntr_35_resp.watch_count, 0);
        assert_eq!(mntr_35_resp.ephemerals_count, 0);
        assert_eq!(mntr_35_resp.approximate_data_size, 297);
        assert_eq!(mntr_35_resp.open_file_descriptor_count.unwrap(), 58);
        assert_eq!(mntr_35_resp.max_file_descriptor_count.unwrap(), 1048576);
        assert_eq!(mntr_35_resp.followers.unwrap(), 4);
        assert_eq!(mntr_35_resp.synced_followers.unwrap(), 2);
        assert_eq!(mntr_35_resp.pending_syncs.unwrap(), 0);
        assert_eq!(mntr_35_resp.last_proposal_size.unwrap(), -1);
        assert_eq!(mntr_35_resp.min_proposal_size.unwrap(), -1);
        assert_eq!(mntr_35_resp.max_proposal_size.unwrap(), -1);
        assert_eq!(mntr_35_resp.misc.len(), 0);
    }

    #[test]
    fn should_build_response_from_zk36_monitor_response_body() {
        let mntr_36_resp_body = fs::read_to_string("../../fixtures/3.6/mntr.response").unwrap();
        let mntr_36_resp = ZK4LWMonitor::build_response(mntr_36_resp_body.as_str()).unwrap();

        assert_eq!(mntr_36_resp.version, "3.6.1");
        assert_eq!(
            mntr_36_resp.build_revision,
            "104dcb3e3fb464b30c5186d229e00af9f332524b"
        );
        assert_eq!(mntr_36_resp.build_date, "built on 04/21/2020 15:01 GMT");
        assert_eq!(mntr_36_resp.latency.avg, 0.0);
        assert_eq!(mntr_36_resp.latency.min, 0);
        assert_eq!(mntr_36_resp.latency.max, 0);
        assert_eq!(mntr_36_resp.packets_received, 2);
        assert_eq!(mntr_36_resp.packets_sent, 3);
        assert_eq!(mntr_36_resp.num_alive_connections, 1);
        assert_eq!(mntr_36_resp.outstanding_requests, 0);
        assert_eq!(mntr_36_resp.server_state, LEADER);
        assert_eq!(mntr_36_resp.znode_count, 5);
        assert_eq!(mntr_36_resp.watch_count, 0);
        assert_eq!(mntr_36_resp.ephemerals_count, 0);
        assert_eq!(mntr_36_resp.approximate_data_size, 297);
        assert_eq!(mntr_36_resp.open_file_descriptor_count.unwrap(), 67);
        assert_eq!(mntr_36_resp.max_file_descriptor_count.unwrap(), 1048576);
        // TODO absent:
        //assert_eq!(mntr_36_resp.followers.unwrap(), 4);
        assert_eq!(mntr_36_resp.synced_followers.unwrap(), 2);
        assert_eq!(mntr_36_resp.pending_syncs.unwrap(), 0);
        assert_eq!(mntr_36_resp.last_proposal_size.unwrap(), -1);
        assert_eq!(mntr_36_resp.min_proposal_size.unwrap(), -1);
        assert_eq!(mntr_36_resp.max_proposal_size.unwrap(), -1);
        // TODO lots of fields to add:
        //assert_eq!(mntr_36_resp.misc.len(), 0);
    }
}

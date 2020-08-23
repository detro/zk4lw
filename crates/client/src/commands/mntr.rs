//! The 4LW Monitor command. Also known as "mntr".
//!
//! This command outputs a list of variables that could be used for
//! monitoring the health of the cluster.
//!
//! Available since: ZooKeeper 3.4.0

use std::collections::HashMap;

use crate::{client::*, commands::common::*, errors::*, parsing::*, result::*, state::*};

const COMMAND: &'static str = "mntr";

/// Response to the `mntr` command
///
/// The fields are "divided" into 3 "classes":
///
/// * mapped & always present: they will always be in the response,
///   and the value will always be valid
/// * mapped & sometimes present: they will always be in the response,
///   but their value is an `Option` that will be set to `None` if missing -
///   a value can be missing either because the node is not a "Leader",
///   or because the value is not reported by the specific instance of ZooKeeper
///   against which the command was executed
/// * unmapped: when a value hasn't been mapped (yet), its stored in the
///   `misc` field, that is an hash-map - the idea is that we then release
///   an update that maps it
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
    // connections
    pub num_alive_connections: i64,
    pub connection_drop_count: Option<i64>,
    pub connection_drop_probability: Option<f64>,
    pub connection_rejected: Option<i64>,
    pub connection_request_count: Option<i64>,
    pub connection_revalidate_count: Option<i64>,
    pub sessionless_connections_expired: Option<i64>,
    // watchers
    pub add_dead_watcher_stall_time: Option<i64>,
    pub dead_watchers_cleared: Option<i64>,
    pub dead_watchers_queued: Option<i64>,
    // requests
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
    pub open_file_descriptor_count: i64,
    pub max_file_descriptor_count: i64,
    /// Learner members of the ensemble can either be "Followers" or "Observers"
    /// NOTE: in version of ZK < 3.6.x, this field was known as "followers".
    /// See: https://cwiki.apache.org/confluence/display/ZOOKEEPER/Upgrade+FAQ#UpgradeFAQ-Metric
    pub learners: Option<i64>,
    pub synced_followers: Option<i64>,
    pub pending_syncs: Option<i64>,
    pub synced_non_voting_followers: Option<i64>,
    pub synced_observers: Option<i64>,
    // proposals
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
                // latency
                "zk_avg_latency" => response.latency.avg = val.parse()?,
                "zk_max_latency" => response.latency.max = val.parse()?,
                "zk_min_latency" => response.latency.min = val.parse()?,
                // packets
                "zk_packets_received" => response.packets_received = val.parse()?,
                "zk_packets_sent" => response.packets_sent = val.parse()?,
                // connections
                "zk_num_alive_connections" => response.num_alive_connections = val.parse()?,
                "zk_connection_drop_count" => response.connection_drop_count = Some(val.parse()?),
                "zk_connection_drop_probability" => {
                    response.connection_drop_probability = Some(val.parse()?)
                }
                "zk_connection_rejected" => response.connection_rejected = Some(val.parse()?),
                "zk_connection_request_count" => {
                    response.connection_request_count = Some(val.parse()?)
                }
                "zk_connection_revalidate_count" => {
                    response.connection_revalidate_count = Some(val.parse()?)
                }
                "zk_sessionless_connections_expired" => {
                    response.sessionless_connections_expired = Some(val.parse()?)
                }
                // watchers
                "zk_add_dead_watcher_stall_time" => {
                    response.add_dead_watcher_stall_time = Some(val.parse()?)
                }
                "zk_dead_watchers_cleared" => response.dead_watchers_cleared = Some(val.parse()?),
                "zk_dead_watchers_queued" => response.dead_watchers_queued = Some(val.parse()?),
                // requests
                "zk_outstanding_requests" => response.outstanding_requests = val.parse()?,
                // state
                "zk_server_state" => response.server_state = val.parse()?,
                // znodes
                "zk_znode_count" => response.znode_count = val.parse()?,
                "zk_watch_count" => response.watch_count = val.parse()?,
                "zk_ephemerals_count" => response.ephemerals_count = val.parse()?,
                // data size
                "zk_approximate_data_size" => response.approximate_data_size = val.parse()?,
                // file descriptors
                "zk_open_file_descriptor_count" => {
                    response.open_file_descriptor_count = val.parse()?
                }
                "zk_max_file_descriptor_count" => {
                    response.max_file_descriptor_count = val.parse()?
                }
                // followers
                "zk_followers" => response.learners = Some(val.parse()?), //< NOTE: synonym of "zk_learners" in ZK < 3.6.x
                "zk_learners" => response.learners = Some(val.parse()?),
                "zk_synced_followers" => response.synced_followers = Some(val.parse()?),
                "zk_pending_syncs" => response.pending_syncs = Some(val.parse()?),
                "zk_synced_non_voting_followers" => response.synced_non_voting_followers = Some(val.parse()?),
                "zk_synced_observers" => response.synced_observers = Some(val.parse()?),
                // proposals
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
        assert_eq!(mntr_34_resp.packets_received, 5);
        assert_eq!(mntr_34_resp.packets_sent, 4);
        assert_eq!(mntr_34_resp.num_alive_connections, 1);
        assert_eq!(mntr_34_resp.outstanding_requests, 0);
        assert_eq!(mntr_34_resp.server_state, LEADER);
        assert_eq!(mntr_34_resp.znode_count, 4);
        assert_eq!(mntr_34_resp.watch_count, 0);
        assert_eq!(mntr_34_resp.ephemerals_count, 0);
        assert_eq!(mntr_34_resp.approximate_data_size, 27);
        assert_eq!(mntr_34_resp.open_file_descriptor_count, 36);
        assert_eq!(mntr_34_resp.max_file_descriptor_count, 1048576);
        assert_eq!(mntr_34_resp.learners.unwrap(), 4);
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

        assert_eq!(mntr_35_resp.version, "3.5.8");
        assert_eq!(
            mntr_35_resp.build_revision,
            "f439ca583e70862c3068a1f2a7d4d068eec33315"
        );
        assert_eq!(mntr_35_resp.build_date, "built on 05/04/2020 15:07 GMT");
        assert_eq!(mntr_35_resp.latency.avg, 0.0);
        assert_eq!(mntr_35_resp.latency.min, 0);
        assert_eq!(mntr_35_resp.latency.max, 0);
        assert_eq!(mntr_35_resp.packets_received, 3);
        assert_eq!(mntr_35_resp.packets_sent, 2);
        assert_eq!(mntr_35_resp.num_alive_connections, 1);
        assert_eq!(mntr_35_resp.outstanding_requests, 0);
        assert_eq!(mntr_35_resp.server_state, LEADER);
        assert_eq!(mntr_35_resp.znode_count, 5);
        assert_eq!(mntr_35_resp.watch_count, 0);
        assert_eq!(mntr_35_resp.ephemerals_count, 0);
        assert_eq!(mntr_35_resp.approximate_data_size, 297);
        assert_eq!(mntr_35_resp.open_file_descriptor_count, 58);
        assert_eq!(mntr_35_resp.max_file_descriptor_count, 1048576);
        assert_eq!(mntr_35_resp.learners.unwrap(), 4);
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
        assert_eq!(mntr_36_resp.packets_received, 4);
        assert_eq!(mntr_36_resp.packets_sent, 9);
        assert_eq!(mntr_36_resp.num_alive_connections, 1);
        assert_eq!(mntr_36_resp.outstanding_requests, 0);
        assert_eq!(mntr_36_resp.server_state, LEADER);
        assert_eq!(mntr_36_resp.znode_count, 5);
        assert_eq!(mntr_36_resp.watch_count, 0);
        assert_eq!(mntr_36_resp.ephemerals_count, 0);
        assert_eq!(mntr_36_resp.approximate_data_size, 297);
        assert_eq!(mntr_36_resp.open_file_descriptor_count, 67);
        assert_eq!(mntr_36_resp.max_file_descriptor_count, 1048576);
        assert_eq!(mntr_36_resp.learners.unwrap(), 4);
        assert_eq!(mntr_36_resp.synced_followers.unwrap(), 2);
        assert_eq!(mntr_36_resp.pending_syncs.unwrap(), 0);
        assert_eq!(mntr_36_resp.synced_non_voting_followers.unwrap(), 0);
        assert_eq!(mntr_36_resp.synced_observers.unwrap(), 2);
        assert_eq!(mntr_36_resp.last_proposal_size.unwrap(), -1);
        assert_eq!(mntr_36_resp.min_proposal_size.unwrap(), -1);
        assert_eq!(mntr_36_resp.max_proposal_size.unwrap(), -1);
        // TODO lots of fields to add:
        // assert_eq!(mntr_36_resp.misc.len(), 0);
    }
}

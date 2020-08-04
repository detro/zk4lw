//! 4LW common values/types returned by commands

use std::default::Default;

/// Sample of Metric, as returned by ZooKeeper
///
/// It's associated with a specific Metric, and represents a "snapshot" of it
/// at the time of collection.
///
/// Not all fields are present, but it depends on the specific ZooKeeper version.
#[derive(Debug)]
pub struct ZK4LWMetricSample {
    /// Average
    pub avg: f64,
    /// Maximum
    pub max: i64,
    /// Minimum
    pub min: i64,
    /// Count
    pub count: Option<i64>,
    /// Sum
    pub sum: Option<i64>,
    /// 50 Percentile
    pub p50: Option<i64>,
    /// 95 Percentile
    pub p95: Option<i64>,
    /// 99 Percentile
    pub p99: Option<i64>,
    /// 999 Percentile
    pub p999: Option<i64>,
}

impl ZK4LWMetricSample {
    pub fn new(avg: f64, max: i64, min: i64) -> Self {
        ZK4LWMetricSample {
            avg,
            max,
            min,
            ..Default::default()
        }
    }
}

impl Default for ZK4LWMetricSample {
    fn default() -> Self {
        ZK4LWMetricSample {
            avg: 0.0,
            max: 0,
            min: 0,
            count: None,
            sum: None,
            p50: None,
            p95: None,
            p99: None,
            p999: None,
        }
    }
}

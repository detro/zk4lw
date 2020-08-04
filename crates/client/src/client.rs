//! The Zookeeper "Four Letter Words" commands client.
//!
//! This module includes both the Trait implemented by commands
//! and the client to execute those commands.

use std::{
    io::{Read, Write},
    net, str,
};

use super::result::*;

/// Trait that defines how a Zookeeper "Four Letter Words" command looks like
pub trait ZK4LWCommand {
    /// Response produced by a successful execution of the command
    type Response;

    /// Returns an `str` needed to send the request
    fn request_body() -> &'static str;

    /// Builds and returns a `ZK4LWResult` of `Response` type
    ///
    /// # Arguments
    /// * `response_body` - A `str` slice containing the raw response body from a given request
    fn build_response(response_body: &str) -> ZK4LWResult<Self::Response>;
}

/// The Zookeeper "Four Letter Words" client
pub struct ZK4LWClient {
    host: String,
    port: u16,
}

impl ZK4LWClient {
    /// Create a new ZK 4LW client
    ///
    /// # Arguments
    /// * `host` - host in the ZK Ensemble to send commands to; can be both an IP or a Hostname
    /// * `port` - port to send commands on
    pub fn new<S: Into<String>>(host: S, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// Execute the given command and return a result containing the response
    pub fn execute<C: ZK4LWCommand>(&self) -> ZK4LWResult<C::Response>
    where
        C: ZK4LWCommand,
    {
        // Connect TCP socket to ZooKeeper server
        let mut stream = net::TcpStream::connect((self.host.as_str(), self.port))?;

        // Send 4LW command
        stream.write_all(C::request_body().as_bytes())?;

        // Read the whole response to buffer
        let mut response_buffer = Vec::new();
        stream.read_to_end(&mut response_buffer)?;

        // Convert buffer to &str
        let response_body = str::from_utf8(&response_buffer)?;

        // Produce final response
        C::build_response(response_body)
    }
}

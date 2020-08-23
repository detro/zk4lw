//! Result of a the execution of a 4LW command.

use std::result;

use crate::errors::*;

/// Result produced by the execution of a `ZK4LWCommand` using the `ZK4LWClient`
pub type ZK4LWResult<T> = result::Result<T, ZK4LWError>;

//! Private wire vocabulary for the rail's test-control listener.

use serde::{Deserialize, Serialize};

use super::FaultScript;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RailTestControlRequest {
    SelectFault(FaultScript),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum RailTestControlResponse {
    Selected,
}

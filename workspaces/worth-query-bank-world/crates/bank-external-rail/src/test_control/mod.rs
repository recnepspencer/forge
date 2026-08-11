//! Test-only control plane for selecting the rail's next fault posture.
//!
//! This listener is physically distinct from the production dispatch
//! listener. Production callers cannot name a fault in their request.

mod client;
mod fault_script;
mod listener;
mod protocol;
mod selection;

pub use client::{select_fault, RailTestControlFailure};
pub use fault_script::FaultScript;
pub(crate) use listener::handle_test_control_connection;
pub(crate) use selection::FaultSelection;

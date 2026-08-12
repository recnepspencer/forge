//! The Bank external rail's own process-local server: listener, ledger, and
//! per-connection dispatch.

mod completed_effects;
mod dispatch;
mod fault_behavior;
mod ledger;
mod listener;

pub use listener::RailServer;

//! Naming conventions for fintech workflow certification rows and artifacts.

mod artifacts;
mod rows;

pub(crate) use artifacts::{artifact_alias, read_alias, replay_alias};
pub(crate) use rows::{invariant_id, scenario_name, workflow_name};

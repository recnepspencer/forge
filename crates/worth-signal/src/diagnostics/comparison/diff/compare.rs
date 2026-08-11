mod assembly;
mod execution;
mod explanation;
mod failure;
mod graph;
mod history;
mod lineage;
mod plan;
mod replay;

pub use assembly::compare_flows;
pub use execution::compare_execution_reports;
pub use explanation::compare_explanations;
pub use failure::compare_failures;
pub use graph::compare_graphs;
pub use history::compare_execution_history;
pub use lineage::compare_lineage_records;
pub use plan::compare_plans;
pub use replay::compare_replay_slices;

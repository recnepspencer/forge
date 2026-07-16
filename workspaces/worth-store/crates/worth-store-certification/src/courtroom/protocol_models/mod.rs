#![doc = include_str!("authority_compile_fail_proofs.md")]

mod backend_matrix;
pub mod closeout;
mod compaction_visibility;
mod durability_recovery;
mod import_publication;
mod lease_reclaim;
pub mod mutants;
mod quarantine_readmission;
mod replication_admission;
mod runner_workflow;
mod shared_frontiers;
mod source_precedence;

pub use compaction_visibility::{
    adjudicate_compaction_visibility_refinement, CompactionVisibilityRefinementEvidence,
};

pub mod bounded_wal_tail;
pub mod checkpoint_cutover;
pub mod crash_boundary;
mod denial;
mod readmission;
pub mod recovery_source;
pub mod replay_index;

pub use bounded_wal_tail::BoundedWalTailLayoutReport;
pub use checkpoint_cutover::{
    ensure_recovery_entry_allowed, reject_locator_projection, CheckpointCutoverLayoutReport,
    CheckpointRecoveryManifestLayoutReport,
};
pub use crash_boundary::CrashBoundaryLayoutReport;
pub use denial::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};
pub use readmission::RecoveryReadmissionLayoutReport;
pub use recovery_source::{reject_decision_row, RecoverySourceLayoutReport};
pub use replay_index::{ReplayIndexLayoutCounters, ReplayIndexLayoutReport};

pub(crate) use bounded_wal_tail::lookup_recovery_tail_range;
pub(crate) use crash_boundary::admit_partial_publication_classification;
pub(crate) use recovery_source::project_recovery_source_layout;
pub(crate) use replay_index::admit_recovery_source_replay_index;

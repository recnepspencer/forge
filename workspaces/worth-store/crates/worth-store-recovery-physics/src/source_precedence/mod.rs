mod admission;
mod candidate;
mod checkpoint_base;
mod checkpoint_covered_wal;
mod compaction_product;
mod current_previous_root;
mod page_facts;
mod page_lsn_skip_apply;
mod physical_source;
mod residue;
mod selection;
mod wal_artifacts;
mod wal_tail;

pub use admission::admit_physical_root_slot;
pub use candidate::{
    PhysicalRootCandidateDenial, PhysicalRootSlotObservation, PhysicalRootSourceCandidate,
};
pub use checkpoint_base::{PhysicalCheckpointBase, PhysicalCheckpointBaseDenial};
pub use checkpoint_covered_wal::CheckpointCoveredWalArtifact;
pub use compaction_product::SelectedCompactionProduct;
pub use current_previous_root::{
    select_current_previous_root, PhysicalRootSelectionDenial, SelectedPhysicalRoot,
    SelectedPhysicalRootRole,
};
pub use page_facts::{
    admit_physical_page_facts, PhysicalManifestBlockCandidate, PhysicalPageFactDenial,
    SelectedPhysicalPageFacts,
};
pub use page_lsn_skip_apply::PageLsnSkipApplyDecision;
pub use physical_source::PhysicalRecoverySource;
pub use residue::{PhysicalRecoveryResidue, PhysicalRecoveryResidueKind};
pub use selection::{
    select_physical_recovery_sources, PhysicalSourceSelection, PhysicalSourceSelectionDenial,
    PhysicalSourceSelectionTrace,
};
pub use wal_artifacts::{
    inspect_physical_wal_artifacts, InspectedPhysicalWalArtifacts, PhysicalWalArtifactCorruption,
    PhysicalWalArtifactInspectionDenial,
};
pub use wal_tail::{
    admit_physical_wal_tail, PhysicalWalSegmentCandidate, SelectedPhysicalWalTail,
    SelectedPhysicalWalTailDenial,
};

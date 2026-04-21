use serde::Serialize;

use crate::evidence::StoreCounterSnapshot;
use crate::media::DurableBackendFamily;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6CounterContract {
    pub aspect_layout_plan_count: u64,
    pub aspect_layout_admitted_count: u64,
    pub aspect_layout_fallback_count: u64,
    pub aspect_layout_rejected_count: u64,
    pub aspect_layout_slice_read_count: u64,
    pub aspect_layout_block_decode_count: u64,
    pub aspect_layout_control_replay_breadth: u64,
    pub aspect_layout_whole_state_fallback_count: u64,
    pub structural_block_lookup_count: u64,
    pub structural_block_reuse_admission_count: u64,
    pub structural_block_reuse_hit_count: u64,
    pub structural_block_reuse_miss_count: u64,
    pub chunk_model_freeze_count: u64,
    pub physical_chunk_export_count: u64,
    pub physical_chunk_width_count: u64,
    pub physical_chunk_determinism_violation_count: u64,
    pub milestone_6_proof_only_prepare_count: u64,
    pub milestone_6_on_demand_materialize_count: u64,
    pub milestone_6_policy_eager_resolution_count: u64,
    pub milestone_6_policy_eager_publish_count: u64,
    pub milestone_6_policy_eager_reuse_existing_count: u64,
    pub milestone_7_layout_reference_admission_count: u64,
    pub milestone_9_physical_chunk_reference_admission_count: u64,
}

impl Milestone6CounterContract {
    pub(crate) fn from_snapshot(counter_snapshot: &StoreCounterSnapshot) -> Self {
        Self {
            aspect_layout_plan_count: counter_snapshot.aspect_layout_plan_count,
            aspect_layout_admitted_count: counter_snapshot.aspect_layout_admitted_count,
            aspect_layout_fallback_count: counter_snapshot.aspect_layout_fallback_count,
            aspect_layout_rejected_count: counter_snapshot.aspect_layout_rejected_count,
            aspect_layout_slice_read_count: counter_snapshot.aspect_layout_slice_read_count,
            aspect_layout_block_decode_count: counter_snapshot.aspect_layout_block_decode_count,
            aspect_layout_control_replay_breadth: counter_snapshot
                .aspect_layout_control_replay_breadth,
            aspect_layout_whole_state_fallback_count: counter_snapshot
                .aspect_layout_whole_state_fallback_count,
            structural_block_lookup_count: counter_snapshot.structural_block_lookup_count,
            structural_block_reuse_admission_count: counter_snapshot
                .structural_block_reuse_admission_count,
            structural_block_reuse_hit_count: counter_snapshot.structural_block_reuse_hit_count,
            structural_block_reuse_miss_count: counter_snapshot.structural_block_reuse_miss_count,
            chunk_model_freeze_count: counter_snapshot.chunk_model_freeze_count,
            physical_chunk_export_count: counter_snapshot.physical_chunk_export_count,
            physical_chunk_width_count: counter_snapshot.physical_chunk_width_count,
            physical_chunk_determinism_violation_count: counter_snapshot
                .physical_chunk_determinism_violation_count,
            milestone_6_proof_only_prepare_count: counter_snapshot
                .milestone_6_proof_only_prepare_count,
            milestone_6_on_demand_materialize_count: counter_snapshot
                .milestone_6_on_demand_materialize_count,
            milestone_6_policy_eager_resolution_count: counter_snapshot
                .milestone_6_policy_eager_resolution_count,
            milestone_6_policy_eager_publish_count: counter_snapshot
                .milestone_6_policy_eager_publish_count,
            milestone_6_policy_eager_reuse_existing_count: counter_snapshot
                .milestone_6_policy_eager_reuse_existing_count,
            milestone_7_layout_reference_admission_count: counter_snapshot
                .milestone_7_layout_reference_admission_count,
            milestone_9_physical_chunk_reference_admission_count: counter_snapshot
                .milestone_9_physical_chunk_reference_admission_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureContract {
    pub backend_family: DurableBackendFamily,
    pub aspect_layout_read: Milestone6AccessStructureClaim,
    pub structural_block_reuse: Milestone6AccessStructureClaim,
    pub chunk_model_freeze: Milestone6AccessStructureClaim,
    pub milestone_7_layout_reference: Milestone6AccessStructureClaim,
    pub milestone_9_physical_chunk_reference: Milestone6AccessStructureClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureClaim {
    pub access_structure: String,
    pub guarantee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureVerification {
    pub backend_family: DurableBackendFamily,
    pub aspect_layout_read: Milestone6AccessStructureVerificationPath,
    pub structural_block_reuse: Milestone6AccessStructureVerificationPath,
    pub chunk_model_freeze: Milestone6AccessStructureVerificationPath,
    pub milestone_7_layout_reference: Milestone6AccessStructureVerificationPath,
    pub milestone_9_physical_chunk_reference: Milestone6AccessStructureVerificationPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone6AccessStructureVerificationPath {
    pub verified_at_open: bool,
    pub verification_basis: Option<String>,
    pub verification_gap: Option<String>,
}

impl Milestone6AccessStructureVerificationPath {
    pub(crate) fn verified(verification_basis: impl Into<String>) -> Self {
        Self {
            verified_at_open: true,
            verification_basis: Some(verification_basis.into()),
            verification_gap: None,
        }
    }
    pub(crate) fn debt(verification_gap: impl Into<String>) -> Self {
        Self {
            verified_at_open: false,
            verification_basis: None,
            verification_gap: Some(verification_gap.into()),
        }
    }
}

impl Milestone6AccessStructureContract {
    pub(crate) fn for_backend_family(backend_family: DurableBackendFamily) -> Self {
        let backend_label = match backend_family {
            DurableBackendFamily::InMemory => "in-memory Milestone 6 derived layout registry",
            DurableBackendFamily::LocalFileAtomicRewrite => {
                "local-file Milestone 6 derived layout registry rebuilt atomically per write"
            }
            DurableBackendFamily::SqliteTransactional => {
                "sqlite Milestone 6 derived layout rows keyed by artifact id"
            }
        };
        Self {
            backend_family,
            aspect_layout_read: Milestone6AccessStructureClaim {
                access_structure: format!("{backend_label}: scope-to-slice membership records keyed by canonical target/scope/projection identity"),
                guarantee: "admitted aspect layout reads are durably certified through explicit Milestone 6 scope membership records rather than only through the materialization blob".to_string(),
            },
            structural_block_reuse: Milestone6AccessStructureClaim {
                access_structure: format!("{backend_label}: semantic structural-block records keyed by cross-branch structural block identity"),
                guarantee: "structural block reuse is durably certified through explicit semantic structural-block records carrying cross-branch block identity, equivalence version, canonical slice membership, and supporting layout publication references".to_string(),
            },
            chunk_model_freeze: Milestone6AccessStructureClaim {
                access_structure: format!("{backend_label}: chunk-membership records keyed by physical chunk identity"),
                guarantee: "frozen chunk layout is durably certified through explicit Milestone 6 chunk-membership records rather than only through the materialization blob".to_string(),
            },
            milestone_7_layout_reference: Milestone6AccessStructureClaim {
                access_structure: format!("{backend_label}: compile-time milestone 7 reference wrappers derived from admitted layout proofs"),
                guarantee: "milestone 7 layout references never expose slice, block, or placement internals".to_string(),
            },
            milestone_9_physical_chunk_reference: Milestone6AccessStructureClaim {
                access_structure: format!("{backend_label}: compile-time milestone 9 chunk wrappers derived from deterministic chunk witnesses"),
                guarantee: "milestone 9 references expose only physical chunk identity plus determinism metadata, never authority or mutation rights".to_string(),
            },
        }
    }
}

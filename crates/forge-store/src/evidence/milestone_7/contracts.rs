use crate::{evidence::StoreCounterSnapshot, media::DurableBackendFamily};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7AccessStructureContract {
    pub backend_family: DurableBackendFamily,
    pub schema_boundary_fetch: Milestone7AccessStructureClaim,
    pub lineage_lookup: Milestone7AccessStructureClaim,
    pub cursor_resume: Milestone7AccessStructureClaim,
    pub embedded_checkpoint_fetch: Milestone7AccessStructureClaim,
    pub commit_coupled_support_publication: Milestone7AccessStructureClaim,
    pub cursor_identity_admission: Milestone7AccessStructureClaim,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7AccessStructureClaim {
    pub access_structure: String,
    pub guarantee: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7AccessStructureVerification {
    pub backend_family: DurableBackendFamily,
    pub schema_boundary_fetch: Milestone7AccessStructureVerificationPath,
    pub lineage_lookup: Milestone7AccessStructureVerificationPath,
    pub cursor_resume: Milestone7AccessStructureVerificationPath,
    pub embedded_checkpoint_fetch: Milestone7AccessStructureVerificationPath,
    pub commit_coupled_support_publication: Milestone7AccessStructureVerificationPath,
    pub cursor_identity_admission: Milestone7AccessStructureVerificationPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7AccessStructureVerificationPath {
    pub verified_at_open: bool,
    pub verification_basis: Option<String>,
    pub verification_gap: Option<String>,
}

impl Milestone7AccessStructureVerificationPath {
    pub fn verified(verification_basis: impl Into<String>) -> Self {
        Self { verified_at_open: true, verification_basis: Some(verification_basis.into()), verification_gap: None }
    }
    pub fn debt(verification_gap: impl Into<String>) -> Self {
        Self { verified_at_open: false, verification_basis: None, verification_gap: Some(verification_gap.into()) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7CounterContract {
    pub commit_support_publication_count: u64,
    pub commit_support_publication_gap_count: u64,
    pub commit_support_summary_build_count: u64,
    pub schema_boundary_fetch_count: u64,
    pub schema_boundary_index_lookup_count: u64,
    pub schema_boundary_rows_read: u64,
    pub schema_boundary_resolution_count: u64,
    pub lineage_lookup_count: u64,
    pub lineage_identity_lookup_count: u64,
    pub lineage_event_rows_read: u64,
    pub lineage_resolution_breadth: u64,
    pub cursor_resume_count: u64,
    pub cursor_identity_lookup_count: u64,
    pub cursor_resume_support_rows_read: u64,
    pub cursor_ack_count: u64,
    pub subscriber_checkpoint_write_count: u64,
    pub embedded_checkpoint_basis_read_count: u64,
    pub checkpoint_shape_reject_count: u64,
    pub support_artifact_recovery_gap_count: u64,
    pub state_delta_apply_count: u64,
    pub state_delta_touched_family_count: u64,
    pub state_delta_touched_record_count: u64,
    pub durable_barrier_verified_count: u64,
}

impl Milestone7CounterContract {
    pub(crate) fn from_snapshot(counter_snapshot: &StoreCounterSnapshot) -> Self {
        Self {
            commit_support_publication_count: counter_snapshot.commit_support_publication_count,
            commit_support_publication_gap_count: counter_snapshot.commit_support_publication_gap_count,
            commit_support_summary_build_count: counter_snapshot.commit_support_summary_build_count,
            schema_boundary_fetch_count: counter_snapshot.schema_boundary_fetch_count,
            schema_boundary_index_lookup_count: counter_snapshot.schema_boundary_index_lookup_count,
            schema_boundary_rows_read: counter_snapshot.schema_boundary_rows_read,
            schema_boundary_resolution_count: counter_snapshot.schema_boundary_resolution_count,
            lineage_lookup_count: counter_snapshot.lineage_lookup_count,
            lineage_identity_lookup_count: counter_snapshot.lineage_identity_lookup_count,
            lineage_event_rows_read: counter_snapshot.lineage_event_rows_read,
            lineage_resolution_breadth: counter_snapshot.lineage_resolution_breadth,
            cursor_resume_count: counter_snapshot.cursor_resume_count,
            cursor_identity_lookup_count: counter_snapshot.cursor_identity_lookup_count,
            cursor_resume_support_rows_read: counter_snapshot.cursor_resume_support_rows_read,
            cursor_ack_count: counter_snapshot.cursor_ack_count,
            subscriber_checkpoint_write_count: counter_snapshot.subscriber_checkpoint_write_count,
            embedded_checkpoint_basis_read_count: counter_snapshot.embedded_checkpoint_basis_read_count,
            checkpoint_shape_reject_count: counter_snapshot.checkpoint_shape_reject_count,
            support_artifact_recovery_gap_count: counter_snapshot.support_artifact_recovery_gap_count,
            state_delta_apply_count: counter_snapshot.state_delta_apply_count,
            state_delta_touched_family_count: counter_snapshot.state_delta_touched_family_count,
            state_delta_touched_record_count: counter_snapshot.state_delta_touched_record_count,
            durable_barrier_verified_count: counter_snapshot.durable_barrier_verified_count,
        }
    }
}

impl Milestone7AccessStructureContract {
    pub(crate) fn for_backend_family(backend_family: DurableBackendFamily) -> Self {
        let backend_label = match backend_family {
            DurableBackendFamily::InMemory => "in-memory BTreeMap indexes",
            DurableBackendFamily::LocalFileAtomicRewrite => {
                "local-file authoritative image maps rebuilt atomically per write"
            }
            DurableBackendFamily::SqliteTransactional => {
                "sqlite primary-key and transactional row indexes"
            }
        };
        Self {
            backend_family,
            schema_boundary_fetch: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: exact schema support artifact id address by commit-scoped key"),
                guarantee: "schema-boundary fetch is admitted only via exact support identity plus counted lookup/read work".to_string(),
            },
            lineage_lookup: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: exact lineage support artifact address by exact artifact id and commit-scoped key"),
                guarantee: "lineage lookup is admitted only via exact support identity plus counted identity/breadth work".to_string(),
            },
            cursor_resume: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: durable cursor identity map keyed by cursor id"),
                guarantee: "cursor resume is admitted only when continuation starts from exact cursor identity and counted support-row reads".to_string(),
            },
            embedded_checkpoint_fetch: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: embedded checkpoint map keyed by checkpoint id"),
                guarantee: "embedded checkpoint fetch is admitted only through exact checkpoint identity and counted basis reads".to_string(),
            },
            commit_coupled_support_publication: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: deterministic support artifact ids coupled to canonical commit append"),
                guarantee: "support publication is admitted only as commit-coupled exactly-once append with summary-build accounting".to_string(),
            },
            cursor_identity_admission: Milestone7AccessStructureClaim {
                access_structure: format!("{backend_label}: durable cursor identity admission keyed by cursor id"),
                guarantee: "cursor identity admission is admitted only through acknowledged cursor writes that dominate checkpoint publication".to_string(),
            },
        }
    }
}

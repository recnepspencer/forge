use crate::{
    authority::AuthoritativeExportBundle,
    delta::ComplexityStatus,
    evidence::StoreCounterSnapshot,
    media::{DurableBackendFamily, DurableMediaReport},
    recovery::{SupportArtifactFamily, SupportArtifactRecoveryReport},
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7CertificationBundle {
    pub backend_family: DurableBackendFamily,
    pub history_digest: String,
    pub artifact_digest: String,
    pub replay_digest: String,
    pub support_truth_digest: String,
    pub diagnostics_digest: String,
    pub support_artifact_recovery_report: SupportArtifactRecoveryReport,
    pub certification_summary: SupportDurabilityCertificationSummary,
    pub access_structure_contract: Milestone7AccessStructureContract,
    pub access_structure_verification: Milestone7AccessStructureVerification,
    pub complexity_status: Milestone7ComplexitySurface,
    pub counter_contract: Milestone7CounterContract,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDurabilityCertificationSummary {
    pub clean_restart_support: bool,
    pub exactly_once_support_publication: bool,
    pub support_rebuild_required_count: usize,
    pub support_quarantine_required_count: usize,
    pub schema_support_entry_count: usize,
    pub lineage_support_entry_count: usize,
    pub cursor_support_entry_count: usize,
    pub embedded_checkpoint_entry_count: usize,
    pub related_commit_entry_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7ComplexitySurface {
    pub schema_boundary_fetch: Milestone7ComplexityPathStatus,
    pub lineage_lookup: Milestone7ComplexityPathStatus,
    pub cursor_resume: Milestone7ComplexityPathStatus,
    pub embedded_checkpoint_fetch: Milestone7ComplexityPathStatus,
    pub commit_coupled_support_publication: Milestone7ComplexityPathStatus,
    pub cursor_identity_admission: Milestone7ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone7ComplexityPathStatus {
    pub status: ComplexityStatus,
    pub proof_basis: Option<String>,
    pub debt_reason: Option<String>,
}

impl Milestone7ComplexityPathStatus {
    fn verified(proof_basis: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Verified,
            proof_basis: Some(proof_basis.into()),
            debt_reason: None,
        }
    }

    fn debt(debt_reason: impl Into<String>) -> Self {
        Self {
            status: ComplexityStatus::Debt,
            proof_basis: None,
            debt_reason: Some(debt_reason.into()),
        }
    }
}

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
        Self {
            verified_at_open: true,
            verification_basis: Some(verification_basis.into()),
            verification_gap: None,
        }
    }

    pub fn debt(verification_gap: impl Into<String>) -> Self {
        Self {
            verified_at_open: false,
            verification_basis: None,
            verification_gap: Some(verification_gap.into()),
        }
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

impl Milestone7CertificationBundle {
    pub fn new(
        primary_export: &AuthoritativeExportBundle,
        control_export: &AuthoritativeExportBundle,
        media_report: DurableMediaReport,
        support_artifact_recovery_report: SupportArtifactRecoveryReport,
        access_structure_verification: Milestone7AccessStructureVerification,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let backend_family = media_report.backend_family();
        let primary_canonical = primary_export.clone().into_canonicalized();
        let control_canonical = control_export.clone().into_canonicalized();
        let primary_history_digest = stable_digest(&primary_canonical.commit_envelopes);
        let primary_artifact_digest = stable_digest(&SupportArtifactDigestBasis {
            commit_support_summaries: &primary_canonical.commit_support_summaries,
            schema_support_records: &primary_canonical.schema_support_records,
            lineage_support_records: &primary_canonical.lineage_support_records,
            durable_cursor_identity_records: &primary_canonical.durable_cursor_identity_records,
            subscriber_checkpoint_records: &primary_canonical.subscriber_checkpoint_records,
        });
        let replay_digest = stable_digest(&control_canonical);
        let certification_summary = SupportDurabilityCertificationSummary {
            clean_restart_support: support_artifact_recovery_report.is_clean(),
            exactly_once_support_publication: primary_artifact_digest
                == stable_digest(&SupportArtifactDigestBasis {
                    commit_support_summaries: &control_canonical.commit_support_summaries,
                    schema_support_records: &control_canonical.schema_support_records,
                    lineage_support_records: &control_canonical.lineage_support_records,
                    durable_cursor_identity_records: &control_canonical
                        .durable_cursor_identity_records,
                    subscriber_checkpoint_records: &control_canonical.subscriber_checkpoint_records,
                }),
            support_rebuild_required_count: support_artifact_recovery_report.rebuilds().len(),
            support_quarantine_required_count: support_artifact_recovery_report.quarantines().len(),
            schema_support_entry_count: support_artifact_recovery_report
                .entries()
                .iter()
                .filter(|entry| matches!(entry.family(), SupportArtifactFamily::SchemaSupport))
                .count(),
            lineage_support_entry_count: support_artifact_recovery_report
                .entries()
                .iter()
                .filter(|entry| matches!(entry.family(), SupportArtifactFamily::LineageSupport))
                .count(),
            cursor_support_entry_count: support_artifact_recovery_report
                .entries()
                .iter()
                .filter(|entry| matches!(entry.family(), SupportArtifactFamily::CursorSupport))
                .count(),
            embedded_checkpoint_entry_count: support_artifact_recovery_report
                .entries()
                .iter()
                .filter(|entry| matches!(entry.family(), SupportArtifactFamily::EmbeddedCheckpoint))
                .count(),
            related_commit_entry_count: support_artifact_recovery_report
                .entries()
                .iter()
                .filter(|entry| entry.related_commit_id().is_some())
                .count(),
        };
        let counter_contract = Milestone7CounterContract {
            commit_support_publication_count: counter_snapshot.commit_support_publication_count,
            commit_support_publication_gap_count: counter_snapshot
                .commit_support_publication_gap_count,
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
            embedded_checkpoint_basis_read_count: counter_snapshot
                .embedded_checkpoint_basis_read_count,
            checkpoint_shape_reject_count: counter_snapshot.checkpoint_shape_reject_count,
            support_artifact_recovery_gap_count: counter_snapshot
                .support_artifact_recovery_gap_count,
            state_delta_apply_count: counter_snapshot.state_delta_apply_count,
            state_delta_touched_family_count: counter_snapshot.state_delta_touched_family_count,
            state_delta_touched_record_count: counter_snapshot.state_delta_touched_record_count,
            durable_barrier_verified_count: counter_snapshot.durable_barrier_verified_count,
        };
        let access_structure_contract =
            Milestone7AccessStructureContract::for_backend_family(backend_family);
        let complexity_status = Milestone7ComplexitySurface::derive(
            &certification_summary,
            &counter_contract,
            &access_structure_contract,
            &access_structure_verification,
        );
        Self {
            backend_family,
            history_digest: primary_history_digest.clone(),
            artifact_digest: primary_artifact_digest.clone(),
            replay_digest,
            support_truth_digest: stable_digest(&SupportTruthDigestBasis {
                history_digest: primary_history_digest,
                artifact_digest: primary_artifact_digest,
                replay_digest: stable_digest(&control_canonical),
                support_artifact_recovery_report: &support_artifact_recovery_report,
                certification_summary: &certification_summary,
            }),
            diagnostics_digest: stable_digest(&DiagnosticsDigestBasis {
                support_artifact_recovery_report: &support_artifact_recovery_report,
                certification_summary: &certification_summary,
                access_structure_verification: &access_structure_verification,
                complexity_status: &complexity_status,
                counter_contract: &counter_contract,
            }),
            support_artifact_recovery_report,
            certification_summary,
            access_structure_contract,
            access_structure_verification,
            complexity_status,
            counter_contract,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 7 certification serialization")
    }
}

impl Milestone7ComplexitySurface {
    fn derive(
        certification_summary: &SupportDurabilityCertificationSummary,
        counter_contract: &Milestone7CounterContract,
        access_structure_contract: &Milestone7AccessStructureContract,
        access_structure_verification: &Milestone7AccessStructureVerification,
    ) -> Self {
        Self {
            schema_boundary_fetch: derive_verified_path(
                &access_structure_contract.schema_boundary_fetch,
                &access_structure_verification.schema_boundary_fetch,
            ),
            lineage_lookup: derive_verified_path(
                &access_structure_contract.lineage_lookup,
                &access_structure_verification.lineage_lookup,
            ),
            cursor_resume: if !access_structure_verification.cursor_resume.verified_at_open {
                Milestone7ComplexityPathStatus::debt(
                    access_structure_verification
                        .cursor_resume
                        .verification_gap
                        .clone()
                        .unwrap_or_else(|| {
                            "cursor resume access structure was not verified at open".to_string()
                        }),
                )
            } else if counter_contract.cursor_identity_lookup_count
                >= counter_contract.cursor_resume_count
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract.cursor_resume.access_structure,
                    access_structure_contract.cursor_resume.guarantee,
                    access_structure_verification
                        .cursor_resume
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt(
                    "cursor resume exceeds observed cursor identity lookup coverage; missing exact cursor identity admission evidence",
                )
            },
            embedded_checkpoint_fetch: derive_verified_path(
                &access_structure_contract.embedded_checkpoint_fetch,
                &access_structure_verification.embedded_checkpoint_fetch,
            ),
            commit_coupled_support_publication: if !access_structure_verification
                .commit_coupled_support_publication
                .verified_at_open
            {
                Milestone7ComplexityPathStatus::debt(
                    access_structure_verification
                        .commit_coupled_support_publication
                        .verification_gap
                        .clone()
                        .unwrap_or_else(|| {
                            "commit-coupled support publication access structure was not verified at open"
                                .to_string()
                        }),
                )
            } else if certification_summary.exactly_once_support_publication
                && counter_contract.commit_support_summary_build_count
                    >= counter_contract.commit_support_publication_count
                && counter_contract.commit_support_publication_gap_count == 0
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract
                        .commit_coupled_support_publication
                        .access_structure,
                    access_structure_contract
                        .commit_coupled_support_publication
                        .guarantee,
                    access_structure_verification
                        .commit_coupled_support_publication
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt(
                    "missing exactly-once commit support publication proof or publication-gap-free summary coupling",
                )
            },
            cursor_identity_admission: if !access_structure_verification
                .cursor_identity_admission
                .verified_at_open
            {
                Milestone7ComplexityPathStatus::debt(
                    access_structure_verification
                        .cursor_identity_admission
                        .verification_gap
                        .clone()
                        .unwrap_or_else(|| {
                            "cursor identity admission access structure was not verified at open"
                                .to_string()
                        }),
                )
            } else if counter_contract.subscriber_checkpoint_write_count
                <= counter_contract.cursor_ack_count
            {
                Milestone7ComplexityPathStatus::verified(format!(
                    "{}; {}; {}",
                    access_structure_contract
                        .cursor_identity_admission
                        .access_structure,
                    access_structure_contract
                        .cursor_identity_admission
                        .guarantee,
                    access_structure_verification
                        .cursor_identity_admission
                        .verification_basis
                        .as_deref()
                        .unwrap_or_default()
                ))
            } else {
                Milestone7ComplexityPathStatus::debt(
                    "subscriber checkpoints outpaced acknowledged cursor admissions; missing exact cursor identity admission proof",
                )
            },
        }
    }
}

fn derive_verified_path(
    contract: &Milestone7AccessStructureClaim,
    verification: &Milestone7AccessStructureVerificationPath,
) -> Milestone7ComplexityPathStatus {
    if verification.verified_at_open {
        Milestone7ComplexityPathStatus::verified(format!(
            "{}; {}; {}",
            contract.access_structure,
            contract.guarantee,
            verification
                .verification_basis
                .as_deref()
                .unwrap_or_default()
        ))
    } else {
        Milestone7ComplexityPathStatus::debt(
            verification.verification_gap.clone().unwrap_or_else(|| {
                "required access structure was not verified at open".to_string()
            }),
        )
    }
}

impl Milestone7AccessStructureContract {
    fn for_backend_family(backend_family: DurableBackendFamily) -> Self {
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
                access_structure: format!(
                    "{backend_label}: exact schema support artifact id address by commit-scoped key"
                ),
                guarantee:
                    "schema-boundary fetch is admitted only via exact support identity plus counted lookup/read work"
                        .to_string(),
            },
            lineage_lookup: Milestone7AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: exact lineage support artifact address by exact artifact id and commit-scoped key"
                ),
                guarantee:
                    "lineage lookup is admitted only via exact support identity plus counted identity/breadth work"
                        .to_string(),
            },
            cursor_resume: Milestone7AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: durable cursor identity map keyed by cursor id"
                ),
                guarantee:
                    "cursor resume is admitted only when continuation starts from exact cursor identity and counted support-row reads"
                        .to_string(),
            },
            embedded_checkpoint_fetch: Milestone7AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: embedded checkpoint map keyed by checkpoint id"
                ),
                guarantee:
                    "embedded checkpoint fetch is admitted only through exact checkpoint identity and counted basis reads"
                        .to_string(),
            },
            commit_coupled_support_publication: Milestone7AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: deterministic support artifact ids coupled to canonical commit append"
                ),
                guarantee:
                    "support publication is admitted only as commit-coupled exactly-once append with summary-build accounting"
                        .to_string(),
            },
            cursor_identity_admission: Milestone7AccessStructureClaim {
                access_structure: format!(
                    "{backend_label}: durable cursor identity admission keyed by cursor id"
                ),
                guarantee:
                    "cursor identity admission is admitted only through acknowledged cursor writes that dominate checkpoint publication"
                        .to_string(),
            },
        }
    }
}

#[derive(Serialize)]
struct SupportArtifactDigestBasis<'a> {
    commit_support_summaries: &'a [crate::backend::records::CommitSupportSummaryRecord],
    schema_support_records: &'a [crate::backend::records::SchemaSupportRecord],
    lineage_support_records: &'a [crate::backend::records::LineageSupportRecord],
    durable_cursor_identity_records: &'a [crate::backend::records::DurableCursorIdentityRecord],
    subscriber_checkpoint_records: &'a [crate::backend::records::SubscriberCheckpointRecord],
}

#[derive(Serialize)]
struct DiagnosticsDigestBasis<'a> {
    support_artifact_recovery_report: &'a SupportArtifactRecoveryReport,
    certification_summary: &'a SupportDurabilityCertificationSummary,
    access_structure_verification: &'a Milestone7AccessStructureVerification,
    complexity_status: &'a Milestone7ComplexitySurface,
    counter_contract: &'a Milestone7CounterContract,
}

#[derive(Serialize)]
struct SupportTruthDigestBasis<'a> {
    history_digest: String,
    artifact_digest: String,
    replay_digest: String,
    support_artifact_recovery_report: &'a SupportArtifactRecoveryReport,
    certification_summary: &'a SupportDurabilityCertificationSummary,
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("milestone 7 digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

use crate::{
    authority::AuthoritativeExportBundle,
    evidence::StoreCounterSnapshot,
    media::{DurableBackendFamily, DurableMediaReport},
    recovery::{SupportArtifactFamily, SupportArtifactRecoveryReport},
};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    complexity::Milestone7ComplexitySurface,
    contracts::{
        Milestone7AccessStructureContract, Milestone7AccessStructureVerification,
        Milestone7CounterContract,
    },
};

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
        let counter_contract = Milestone7CounterContract::from_snapshot(&counter_snapshot);
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

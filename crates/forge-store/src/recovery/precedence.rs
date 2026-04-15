use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    publication::{
        observe_durable_recovery_publication, DurableRecoveryPublicationObservation,
        PublicationClassification,
    },
    wal::{DurableMutationId, WalRecord},
};
use forge_relational::facade::{history::CommitId, replay::CanonicalCommitEnvelope};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecoverySourceKind {
    PublishedAuthoritativeTruth,
    HostedRuntimeCanonicalResult,
    IntentOnly,
    RequiresRebuild,
    RequiresQuarantine,
    MaintenanceResidue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoverySourceReport {
    durable_mutation_id: DurableMutationId,
    source_kind: RecoverySourceKind,
    publication_classification: PublicationClassification,
    reason: String,
}

impl RecoverySourceReport {
    pub fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }

    pub fn source_kind(&self) -> RecoverySourceKind {
        self.source_kind
    }

    pub fn publication_classification(&self) -> PublicationClassification {
        self.publication_classification
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RecoverySourceSet {
    durable_mutation_id: DurableMutationId,
    observation: DurableRecoveryPublicationObservation,
}

impl RecoverySourceSet {
    pub(crate) fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }

    pub(crate) fn observation(&self) -> &DurableRecoveryPublicationObservation {
        &self.observation
    }
}

pub(crate) struct RecoverySourceSelection {
    source_kind: RecoverySourceKind,
    commit_id: Option<CommitId>,
    canonical_envelope: Option<CanonicalCommitEnvelope>,
    acknowledgment_eligible: bool,
    report: RecoverySourceReport,
}

impl RecoverySourceSelection {
    pub(crate) fn source_kind(&self) -> RecoverySourceKind {
        self.source_kind
    }

    pub(crate) fn commit_id(&self) -> Option<CommitId> {
        self.commit_id
    }

    pub(crate) fn canonical_envelope(&self) -> Option<&CanonicalCommitEnvelope> {
        self.canonical_envelope.as_ref()
    }

    pub(crate) fn acknowledgment_eligible(&self) -> bool {
        self.acknowledgment_eligible
    }

    pub(crate) fn report(&self) -> &RecoverySourceReport {
        &self.report
    }
}

pub(crate) fn build_recovery_source_set(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    wal_records: &[&WalRecord],
    backend_report: crate::media::DurableMediaReport,
) -> Result<RecoverySourceSet, StoreError> {
    Ok(RecoverySourceSet {
        durable_mutation_id,
        observation: observe_durable_recovery_publication(
            state,
            durable_mutation_id,
            wal_records,
            backend_report,
        )?,
    })
}

pub(crate) fn select_recovery_source(
    state: &StoreState,
    source_set: &RecoverySourceSet,
) -> Result<RecoverySourceSelection, StoreError> {
    let observation = source_set.observation();
    let classification = observation.publication().classification();
    let family_states = observation.publication().family_states();
    let acknowledgment_eligible = family_states.iter().any(|state| {
        state.family() == crate::PublicationFamily::AcknowledgmentEligibility
            && state.state() == crate::PublicationState::Published
    });
    let commit_id = observation.commit_id();
    let canonical_envelope = observation.canonical_envelope().cloned();
    let authoritative_commit_published = family_states.iter().any(|state| {
        state.family() == crate::PublicationFamily::AuthoritativeCommitAppendUnit
            && state.state() == crate::PublicationState::Published
    });
    let branch_head_published = family_states.iter().any(|state| {
        state.family() == crate::PublicationFamily::BranchHeadPublication
            && state.state() == crate::PublicationState::Published
    });

    let (source_kind, reason) = match classification {
        PublicationClassification::RetainTrusted => (
            RecoverySourceKind::PublishedAuthoritativeTruth,
            "authoritative publication families are fully published and trusted".to_string(),
        ),
        PublicationClassification::FinishPublication => {
            if authoritative_commit_published && branch_head_published {
                (
                    RecoverySourceKind::PublishedAuthoritativeTruth,
                    "authoritative truth is already published; only acknowledgment eligibility remains incomplete"
                        .to_string(),
                )
            } else if authoritative_commit_published && !branch_head_published {
                (
                    RecoverySourceKind::RequiresQuarantine,
                    "authoritative commit append is present without a published branch head"
                        .to_string(),
                )
            } else if let Some(canonical_envelope) = &canonical_envelope {
                (
                    RecoverySourceKind::HostedRuntimeCanonicalResult,
                    format!(
                        "hosted runtime canonical result for commit {} must finish publication",
                        canonical_envelope.commit.commit_id.0
                    ),
                )
            } else if observation.intent_present() {
                (
                    RecoverySourceKind::IntentOnly,
                    "only intent residue remains admitted for recovery".to_string(),
                )
            } else {
                (
                    RecoverySourceKind::MaintenanceResidue,
                    "publication is incomplete but no admitted canonical result remains"
                        .to_string(),
                )
            }
        }
        PublicationClassification::RequireRebuild => (
            RecoverySourceKind::RequiresRebuild,
            "publication family state requires authoritative rebuild before recovery may trust it"
                .to_string(),
        ),
        PublicationClassification::RequireQuarantine => (
            RecoverySourceKind::RequiresQuarantine,
            "publication family state requires quarantine before ordinary recovery may trust it"
                .to_string(),
        ),
        PublicationClassification::DiscardUnpublished => (
            RecoverySourceKind::IntentOnly,
            "unpublished residue may be discarded".to_string(),
        ),
    };

    if source_kind == RecoverySourceKind::PublishedAuthoritativeTruth {
        let commit_id = commit_id.ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::RecoveryRequiresFullRebuild,
                format!(
                    "durable mutation {} published without a recoverable commit id",
                    source_set.durable_mutation_id().0
                ),
            )
        })?;
        if !state.has_commit(commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::RecoveryRequiresFullRebuild,
                format!(
                    "durable mutation {} references missing authoritative commit {} during recovery",
                    source_set.durable_mutation_id().0,
                    commit_id.0
                ),
            ));
        }
    }

    if source_kind == RecoverySourceKind::HostedRuntimeCanonicalResult
        && canonical_envelope.is_none()
    {
        return Err(StoreError::new(
            StoreErrorKind::RecoverySourcePrecedenceViolation,
            format!(
                "durable mutation {} selected hosted runtime finish-publication recovery without an admitted canonical result",
                source_set.durable_mutation_id().0
            ),
        ));
    }

    if source_kind == RecoverySourceKind::IntentOnly && !observation.intent_present() {
        return Err(StoreError::new(
            StoreErrorKind::RecoverySourcePrecedenceViolation,
            format!(
                "durable mutation {} resolved to intent-only recovery without an admitted intent record",
                source_set.durable_mutation_id().0
            ),
        ));
    }

    let report = RecoverySourceReport {
        durable_mutation_id: source_set.durable_mutation_id(),
        source_kind,
        publication_classification: classification,
        reason: format!(
            "{}; family_states={:?}",
            reason,
            family_states
                .iter()
                .map(|state| (state.family(), state.state()))
                .collect::<Vec<_>>()
        ),
    };

    Ok(RecoverySourceSelection {
        source_kind,
        commit_id,
        canonical_envelope,
        acknowledgment_eligible,
        report,
    })
}

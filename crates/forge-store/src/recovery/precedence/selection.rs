use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    publication::PublicationClassification,
};

use super::model::{
    RecoverySourceKind, RecoverySourceReport, RecoverySourceSelection, RecoverySourceSet,
};

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

    verify_source_requirements(
        state,
        source_set,
        source_kind,
        commit_id,
        canonical_envelope.as_ref(),
    )?;

    let report = RecoverySourceReport {
        durable_mutation_id: source_set.durable_mutation_id(),
        mutation_identity: source_set.mutation_identity().clone(),
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

fn verify_source_requirements(
    state: &StoreState,
    source_set: &RecoverySourceSet,
    source_kind: RecoverySourceKind,
    commit_id: Option<forge_relational::facade::history::CommitId>,
    canonical_envelope: Option<&forge_relational::facade::replay::CanonicalCommitEnvelope>,
) -> Result<(), StoreError> {
    let observation = source_set.observation();
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
    Ok(())
}

use crate::{
    backend::records::{SnapshotBasisRecord, SnapshotImageRecord, StoreState},
    failure::StoreError,
    media::{DurabilityBarrierClass, DurableMediaReport},
    wal::{DurableMutationId, DurablePublicationPhase},
};
use forge_relational::facade::history::CommitId;

use super::{
    admit_local_snapshot_basis_source, admit_local_snapshot_image_source, admit_local_wal_record,
    DurablePublicationFacts, ObservedPublicationFamilyState, PublicationBarrierContract,
    PublicationClassification, PublicationFamily, PublicationState, PublicationStrategy,
    PublicationWriteOutcome,
};

fn observed_family_state(
    family: PublicationFamily,
    contract: PublicationBarrierContract,
    present: bool,
    media_report: Option<DurableMediaReport>,
    source_admitted: bool,
) -> ObservedPublicationFamilyState {
    let observed_content_barrier = media_report.map(|report| report.content_barrier());
    let observed_metadata_barrier = media_report.map(|report| report.metadata_barrier());
    let state = if !present {
        if family == PublicationFamily::AcknowledgmentEligibility {
            PublicationState::BarrierCompleteButNotPublished
        } else {
            PublicationState::Unpublished
        }
    } else if !source_admitted {
        PublicationState::PublicationGap
    } else if let Some(report) = media_report {
        let content_ok = report.content_barrier() >= contract.required_content_barrier();
        let metadata_ok = contract
            .required_metadata_barrier()
            .map(|required| report.metadata_barrier() >= required)
            .unwrap_or(true);
        if content_ok && metadata_ok {
            PublicationState::Published
        } else {
            PublicationState::PartiallyDurable
        }
    } else {
        PublicationState::Published
    };

    ObservedPublicationFamilyState::new(
        family,
        state,
        contract,
        observed_content_barrier,
        observed_metadata_barrier,
        source_admitted,
    )
}

fn minimum_durable_truth_barrier(backend_report: DurableMediaReport) -> DurabilityBarrierClass {
    backend_report.ack_required_barrier()
}

fn contract_for_family(
    family: PublicationFamily,
    backend_report: DurableMediaReport,
) -> PublicationBarrierContract {
    let minimum_durable_barrier = minimum_durable_truth_barrier(backend_report);
    match family {
        PublicationFamily::WalIntent
        | PublicationFamily::WalCanonicalResult
        | PublicationFamily::WalPublicationProgress
        | PublicationFamily::AcknowledgmentEligibility => PublicationBarrierContract::new(
            PublicationStrategy::AppendOnly,
            minimum_durable_barrier,
            None,
            false,
            matches!(
                family,
                PublicationFamily::WalIntent
                    | PublicationFamily::WalCanonicalResult
                    | PublicationFamily::WalPublicationProgress
                    | PublicationFamily::AcknowledgmentEligibility
            ),
        ),
        PublicationFamily::AuthoritativeCommitAppendUnit
        | PublicationFamily::BranchHeadPublication => PublicationBarrierContract::new(
            PublicationStrategy::TransactionPublished,
            minimum_durable_barrier,
            Some(backend_report.metadata_barrier()),
            false,
            true,
        ),
        PublicationFamily::SnapshotBasis | PublicationFamily::SnapshotImage => {
            PublicationBarrierContract::new(
                PublicationStrategy::TransactionPublished,
                backend_report.ack_required_barrier(),
                Some(backend_report.metadata_barrier()),
                false,
                false,
            )
        }
    }
}

pub(crate) fn classify_durable_publication(
    backend_report: DurableMediaReport,
    facts: DurablePublicationFacts,
) -> PublicationWriteOutcome {
    let families = vec![
        observed_family_state(
            PublicationFamily::WalIntent,
            contract_for_family(PublicationFamily::WalIntent, backend_report),
            facts.intent_present,
            Some(backend_report),
            facts.intent_present,
        ),
        observed_family_state(
            PublicationFamily::WalCanonicalResult,
            contract_for_family(PublicationFamily::WalCanonicalResult, backend_report),
            facts.canonical_result_present,
            Some(backend_report),
            facts.canonical_result_present,
        ),
        observed_family_state(
            PublicationFamily::WalPublicationProgress,
            contract_for_family(PublicationFamily::WalPublicationProgress, backend_report),
            facts.authoritative_progress_present,
            Some(backend_report),
            facts.authoritative_progress_present,
        ),
        observed_family_state(
            PublicationFamily::AuthoritativeCommitAppendUnit,
            contract_for_family(
                PublicationFamily::AuthoritativeCommitAppendUnit,
                backend_report,
            ),
            facts.authoritative_commit_present,
            Some(backend_report),
            facts.authoritative_commit_present,
        ),
        observed_family_state(
            PublicationFamily::BranchHeadPublication,
            contract_for_family(PublicationFamily::BranchHeadPublication, backend_report),
            facts.branch_head_present,
            Some(backend_report),
            facts.branch_head_present,
        ),
        observed_family_state(
            PublicationFamily::AcknowledgmentEligibility,
            contract_for_family(PublicationFamily::AcknowledgmentEligibility, backend_report),
            facts.acknowledgment_marker_present,
            Some(backend_report),
            facts.acknowledgment_marker_present,
        ),
    ];
    let non_ack_states = families
        .iter()
        .filter(|state| state.family() != PublicationFamily::AcknowledgmentEligibility)
        .collect::<Vec<_>>();
    let prerequisites_published = non_ack_states
        .iter()
        .all(|state| state.state() == PublicationState::Published);
    let has_gap = families
        .iter()
        .any(|state| state.state() == PublicationState::PublicationGap);
    let has_partial = families
        .iter()
        .any(|state| state.state() == PublicationState::PartiallyDurable);
    let classification = if has_gap {
        PublicationClassification::RequireRebuild
    } else if has_partial {
        PublicationClassification::RequireQuarantine
    } else if prerequisites_published && facts.acknowledgment_marker_present {
        PublicationClassification::RetainTrusted
    } else if prerequisites_published
        || facts.authoritative_commit_present
        || facts.branch_head_present
        || facts.canonical_result_present
    {
        PublicationClassification::FinishPublication
    } else {
        PublicationClassification::DiscardUnpublished
    };

    PublicationWriteOutcome::new(
        backend_report.backend_family(),
        classification,
        classification == PublicationClassification::RetainTrusted,
        prerequisites_published && !has_gap && !has_partial,
        families,
    )
}

pub(crate) fn classify_snapshot_publication(
    backend_report: DurableMediaReport,
    basis: Option<SnapshotBasisRecord>,
    image: Option<SnapshotImageRecord>,
) -> Result<PublicationWriteOutcome, StoreError> {
    let admitted_basis = basis.map(admit_local_snapshot_basis_source).transpose()?;
    let admitted_image = image.map(admit_local_snapshot_image_source).transpose()?;
    let basis_present = admitted_basis.is_some();
    let image_present = admitted_image.is_some();

    let basis_state = observed_family_state(
        PublicationFamily::SnapshotBasis,
        contract_for_family(PublicationFamily::SnapshotBasis, backend_report),
        basis_present,
        Some(backend_report),
        basis_present,
    );
    let image_state = observed_family_state(
        PublicationFamily::SnapshotImage,
        contract_for_family(PublicationFamily::SnapshotImage, backend_report),
        image_present,
        Some(backend_report),
        image_present,
    );

    let classification = match (basis_present, image_present) {
        (true, true) => PublicationClassification::RetainTrusted,
        (true, false) => PublicationClassification::RequireRebuild,
        (false, true) => PublicationClassification::RequireQuarantine,
        (false, false) => PublicationClassification::DiscardUnpublished,
    };
    Ok(PublicationWriteOutcome::new(
        backend_report.backend_family(),
        classification,
        classification == PublicationClassification::RetainTrusted,
        false,
        vec![basis_state, image_state],
    ))
}

pub(crate) fn durable_publication_facts(
    state: &StoreState,
    durable_mutation_id: DurableMutationId,
    expected_commit_id: Option<CommitId>,
) -> Result<DurablePublicationFacts, StoreError> {
    let mut intent_present = false;
    let mut canonical_result_present = false;
    let mut authoritative_progress_present = false;
    let mut acknowledgment_marker_present = false;
    let mut published_commit_id = expected_commit_id;

    for record in state.wal_records_for_mutation(durable_mutation_id) {
        let admitted = admit_local_wal_record(record)?;
        match &admitted.inner().payload {
            crate::wal::WalRecordPayload::DurableMutationIntent(_) => intent_present = true,
            crate::wal::WalRecordPayload::HostedRuntimeCommitResult(result) => {
                canonical_result_present = true;
                published_commit_id =
                    published_commit_id.or(Some(result.canonical_envelope.commit.commit_id));
            }
            crate::wal::WalRecordPayload::BulkCheckpointPublicationIntent(_) => {}
            crate::wal::WalRecordPayload::DurablePublicationProgress(progress) => {
                if progress.phase == DurablePublicationPhase::AuthoritativeAppendPublished {
                    authoritative_progress_present = true;
                }
                if progress.phase == DurablePublicationPhase::AcknowledgmentEligible {
                    acknowledgment_marker_present = true;
                }
                published_commit_id = published_commit_id.or(progress.commit_id);
            }
            crate::wal::WalRecordPayload::RecoveryDecision(_) => {}
        }
    }

    let authoritative_commit_present = published_commit_id
        .map(|commit_id| state.has_commit(commit_id))
        .unwrap_or(false);
    let branch_head_present = published_commit_id
        .map(|commit_id| {
            state
                .branch_head_records
                .values()
                .any(|record| record.head_commit_id == Some(commit_id))
        })
        .unwrap_or(false);

    Ok(DurablePublicationFacts {
        intent_present,
        canonical_result_present,
        authoritative_progress_present,
        authoritative_commit_present,
        branch_head_present,
        acknowledgment_marker_present,
    })
}

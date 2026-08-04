use super::super::outcome::SubscriptionSupportCertificationLaneOutcome;
use super::expectations::{invalid_lane, require_no_primary_cause, require_rejection};
use crate::failure::StoreError;
use crate::subscription_support::SubscriptionResumeClassification::{
    Degraded, Exact, NotResumable, RebuildRequired,
};
use crate::SupportBatchProofKind;

pub(super) fn validate_store_global_density(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome
        .counter_snapshot
        .support_store_global_debt_rejections()
        == 0
    {
        return invalid_lane(
            outcome,
            "store-global rejection lane must bind the store-global debt counter",
        );
    }
    Ok(())
}

pub(super) fn validate_foreground_work(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome.counter_snapshot.support_hot_path_rejections() == 0 {
        return invalid_lane(
            outcome,
            "foreground rejection lane must bind the hot-path rejection counter",
        );
    }
    Ok(())
}

pub(super) fn validate_batch_receipt_reuse(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_no_primary_cause(outcome)?;
    if outcome.classification.is_some() {
        return invalid_lane(
            outcome,
            "batch receipt reuse lane must not masquerade as a resume classification",
        );
    }
    let Some(report) = outcome.batch_receipt_reuse_report() else {
        return invalid_lane(
            outcome,
            "batch receipt reuse lane must carry explicit reuse evidence",
        );
    };
    let required_proofs = [
        SupportBatchProofKind::CompatibilityReceipt,
        SupportBatchProofKind::BasisEvidence,
        SupportBatchProofKind::CursorCheckpointEvidence,
        SupportBatchProofKind::PortabilityScopeEvidence,
    ];
    if report.density_class() == crate::SupportProgramDensityClass::StoreGlobalDebt
        || report.affected_entries() == 0
        || report.reused_proofs() != required_proofs
        || outcome.counter_snapshot.support_batch_receipt_reuse_count()
            != required_proofs.len() as u64
    {
        return invalid_lane(
            outcome,
            "batch receipt reuse lane must prove the full named reuse set exactly once each",
        );
    }
    Ok(())
}

pub(super) fn validate_action_recovery(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_no_primary_cause(outcome)?;
    if outcome
        .counter_snapshot
        .support_action_interrupted_recovery_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "action publication crash recovery lane must bind the recovery counter",
        );
    }
    match outcome.classification {
        Some(Exact | Degraded | RebuildRequired | NotResumable) | None => {}
    }
    Ok(())
}

pub(super) fn validate_global_scan(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome
        .counter_snapshot
        .support_global_scan_recovery_rejection_count()
        == 0
    {
        return invalid_lane(
            outcome,
            "global-scan recovery forbidden lane must bind the explicit rejection counter",
        );
    }
    Ok(())
}

pub(super) fn validate_hidden_exact_loss(
    outcome: &SubscriptionSupportCertificationLaneOutcome,
) -> Result<(), StoreError> {
    require_rejection(outcome)?;
    if outcome
        .counter_snapshot
        .operational_verdict_translation_rejections()
        == 0
        || outcome.counter_snapshot.support_hidden_exact_loss_count() != 0
    {
        return invalid_lane(
            outcome,
            "hidden exact-loss lane must bind translation rejection while hidden exact-loss count remains zero",
        );
    }
    Ok(())
}

use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, WorkloadEvidenceSupport,
};

use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;
use super::denial::{
    PlanarBooleanDownstreamSplitConsumptionDenial,
    PlanarBooleanDownstreamSplitConsumptionDenialKind as Kind,
};
use super::input::PlanarBooleanDownstreamSplitConsumptionInput;

pub(crate) fn validate_downstream_split_consumption_input(
    input: &PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    reject_missing(
        input.split_ledger_receipt().receipt_identity(),
        Kind::MissingSplitLedgerReceipt,
        "split ledger receipt",
        *counters,
        "downstream split consumption requires a Query-owned split ledger receipt",
    )?;
    reject_missing(
        input.decision_log_receipt().receipt_identity(),
        Kind::MissingDecisionLogReceipt,
        "decision log receipt",
        *counters,
        "downstream split consumption requires the Query-owned decision-log receipt",
    )?;
    reject_missing(
        input.validation_receipt().receipt_identity(),
        Kind::MissingValidationReceipt,
        "split-chain validation receipt",
        *counters,
        "downstream split consumption requires split-chain validation authority",
    )?;
    reject_missing(
        input.persistent_naming_receipt().receipt_identity(),
        Kind::MissingPersistentNamingReceipt,
        "persistent naming receipt",
        *counters,
        "downstream split consumption requires persistent naming authority",
    )?;
    reject_missing(
        input.replay_parity_receipt().receipt_identity(),
        Kind::MissingReplayParityReceipt,
        "replay parity receipt",
        *counters,
        "downstream split consumption requires retained replay parity authority",
    )?;
    reject_missing(
        input.spatial_touch_authority().digest().as_str(),
        Kind::MissingSpatialTouchAuthority,
        "spatial touch authority",
        *counters,
        "downstream split consumption requires admitted spatial touch authority",
    )?;
    reject_missing(
        input.spatial_lookup().lookup_key().as_str(),
        Kind::MissingSpatialLookupProduct,
        "spatial evidence lookup product",
        *counters,
        "downstream split consumption requires spatial evidence lookup authority",
    )?;

    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_decision_log_receipt_identity(),
        input.decision_log_receipt().receipt_identity(),
        Kind::ForeignDecisionLogReceipt,
        "decision-log-receipt",
        counters,
        "split ledger and decision log receipt must describe the same split decisions",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_chain_validation_receipt_identity(),
        input.validation_receipt().receipt_identity(),
        Kind::ForeignValidationReceipt,
        "split-chain-validation-receipt",
        counters,
        "split ledger and validation receipt must describe the same split chain proof",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .split_persistent_naming_receipt_identity(),
        input.persistent_naming_receipt().receipt_identity(),
        Kind::ForeignPersistentNamingReceipt,
        "persistent-naming-receipt",
        counters,
        "split ledger and persistent naming receipt must describe the same named split artifacts",
    )?;
    reject_receipt_mismatch(
        input.split_ledger_receipt().receipt_identity(),
        input
            .replay_parity_receipt()
            .original_split_ledger_receipt_identity(),
        Kind::ForeignReplayParityReceipt,
        "replay-parity-ledger-receipt",
        counters,
        "replay parity receipt must certify the split ledger being consumed downstream",
    )?;
    reject_receipt_mismatch(
        input
            .split_ledger_receipt()
            .downstream_consumption_identity(),
        input
            .replay_parity_receipt()
            .original_downstream_consumption_identity(),
        Kind::ForeignReplayParityReceipt,
        "replay-parity-downstream-identity",
        counters,
        "replay parity receipt must certify the downstream identity being consumed",
    )?;

    reject_spatial_touch_mismatch(input, counters)?;
    reject_spatial_lookup_mismatch(input, counters)?;
    Ok(())
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    mut counters: PlanarBooleanDownstreamSplitConsumptionCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    if observed.is_empty() {
        counters.rejected_missing_receipt();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            kind,
            rejected_identity,
            "non-empty receipt identity",
            observed,
            counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_receipt_mismatch(
    expected: &str,
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    if expected != observed {
        counters.rejected_foreign_receipt();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            kind,
            rejected_identity,
            expected,
            observed,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_spatial_touch_mismatch(
    input: &PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    let authority = input.spatial_touch_authority();
    if authority.boolean_stage() != BooleanEvidenceStageKind::Split {
        counters.rejected_non_receipt_evidence();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            Kind::NonReceiptBackedBooleanSplitEvidence,
            "spatial-touch-boolean-stage",
            "Split",
            format!("{:?}", authority.boolean_stage()),
            *counters,
            "downstream split consumption requires Split spatial touch authority",
        ));
    }
    reject_receipt_mismatch(
        input.split_ledger_receipt().receipt_identity(),
        authority.evidence_identity(),
        Kind::ForeignSpatialTouchAuthority,
        "spatial-touch-evidence-identity",
        counters,
        "spatial touch authority must admit the split ledger receipt being consumed downstream",
    )
}

fn reject_spatial_lookup_mismatch(
    input: &PlanarBooleanDownstreamSplitConsumptionInput<'_>,
    counters: &mut PlanarBooleanDownstreamSplitConsumptionCounters,
) -> Result<(), PlanarBooleanDownstreamSplitConsumptionDenial> {
    let lookup = input.spatial_lookup();
    if lookup.boolean_stage() != BooleanEvidenceStageKind::Split {
        counters.rejected_non_receipt_evidence();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            Kind::NonReceiptBackedBooleanSplitEvidence,
            "spatial-lookup-boolean-stage",
            "Split",
            format!("{:?}", lookup.boolean_stage()),
            *counters,
            "downstream split consumption requires Split spatial lookup authority",
        ));
    }
    if lookup.support() != WorkloadEvidenceSupport::Admitted {
        counters.rejected_non_receipt_evidence();
        return Err(PlanarBooleanDownstreamSplitConsumptionDenial::new(
            Kind::NonReceiptBackedBooleanSplitEvidence,
            "spatial-lookup-support",
            "Admitted",
            format!("{:?}", lookup.support()),
            *counters,
            "downstream split consumption requires admitted spatial lookup authority",
        ));
    }
    reject_receipt_mismatch(
        input.spatial_touch_authority().evidence_identity(),
        lookup.evidence_identity(),
        Kind::ForeignSpatialLookupProduct,
        "spatial-lookup-evidence-identity",
        counters,
        "spatial lookup product must describe the admitted spatial touch authority",
    )?;
    reject_receipt_mismatch(
        input.spatial_touch_authority().stage_index_identity(),
        lookup.lookup_key().stage_index_identity(),
        Kind::ForeignSpatialLookupProduct,
        "spatial-lookup-stage-index-identity",
        counters,
        "spatial lookup product must preserve the admitted stage-index identity",
    )
}

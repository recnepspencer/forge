use super::counters::PlanarBooleanOverlapReadinessLoopLedgerBindingCounters;
use super::denial::{
    PlanarBooleanOverlapReadinessLoopLedgerBindingDenial,
    PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind as Kind,
};
use super::input::PlanarBooleanOverlapRegionExtractionRequestInput;

pub(crate) fn validate_overlap_request_input(
    input: &PlanarBooleanOverlapRegionExtractionRequestInput<'_>,
    counters: &mut PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
) -> Result<(), PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
    let readiness_consumer = input.readiness_consumer();
    let loop_ledger_receipt = input.loop_ledger_receipt();
    reject_missing(
        loop_ledger_receipt.receipt_identity(),
        Kind::MissingLoopLedgerReceiptIdentity,
        "loop ledger receipt",
        counters,
        "overlap request requires a real loop-ledger receipt lineage",
    )?;
    reject_missing(
        loop_ledger_receipt.request_identity(),
        Kind::MissingLoopLedgerRequestIdentity,
        "loop ledger request",
        counters,
        "overlap request requires loop-ledger request provenance",
    )?;
    reject_missing(
        loop_ledger_receipt.selected_plan_digest(),
        Kind::SelectedPlanDigestMismatch,
        "selected plan digest",
        counters,
        "overlap request requires loop-ledger selected-plan provenance",
    )?;
    reject_missing(
        loop_ledger_receipt.decision_log_identity(),
        Kind::MissingLoopDecisionLogIdentity,
        "loop decision log",
        counters,
        "overlap request requires loop decision-log provenance",
    )?;
    reject_missing(
        loop_ledger_receipt.loop_identity_map_identity(),
        Kind::MissingLoopIdentityMapIdentity,
        "loop identity map",
        counters,
        "overlap request requires loop identity-map provenance",
    )?;
    reject_missing(
        loop_ledger_receipt.persistent_name_map_identity(),
        Kind::MissingPersistentNameMapIdentity,
        "loop persistent name map",
        counters,
        "overlap request requires persistent-name propagation provenance",
    )?;
    reject_missing(
        loop_ledger_receipt.subshape_signature_map_identity(),
        Kind::MissingSubshapeSignatureMapIdentity,
        "loop subshape signature map",
        counters,
        "overlap request requires loop subshape-signature provenance",
    )?;
    reject_missing(
        readiness_consumer.selected_route_identity_digest(),
        Kind::SelectedRouteIdentityMismatch,
        "selected route identity",
        counters,
        "overlap request requires admitted selected-route authority",
    )?;
    reject_missing(
        readiness_consumer.selected_family_identity(),
        Kind::SelectedFamilyIdentityMismatch,
        "selected family identity",
        counters,
        "overlap request requires admitted selected-family authority",
    )?;
    reject_missing(
        readiness_consumer.selected_product_identity_digest(),
        Kind::SelectedProductIdentityMismatch,
        "selected product identity",
        counters,
        "overlap request requires admitted selected-product authority",
    )?;
    if matches!(
        readiness_consumer.selected_witness_identity_digest(),
        Some(selected_witness_identity_digest) if selected_witness_identity_digest.is_empty()
    ) {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            Kind::SelectedWitnessIdentityMismatch,
            "selected witness identity",
            *counters,
            "overlap request requires admitted selected-witness authority",
        ));
    }
    reject_missing(
        readiness_consumer.touched_closure_digest(),
        Kind::TouchedClosureMismatch,
        "touched closure digest",
        counters,
        "overlap request requires admitted touched-closure authority",
    )?;
    reject_missing(
        readiness_consumer.selected_plan_digest(),
        Kind::SelectedPlanDigestMismatch,
        "selected plan digest",
        counters,
        "overlap request requires admitted selected-plan authority",
    )?;
    if readiness_consumer.overlap_identity_digests().is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            Kind::OverlapIdentityMismatch,
            "overlap identity digests",
            *counters,
            "overlap request requires admitted overlap-identity authority",
        ));
    }
    reject_missing(
        readiness_consumer.topology_query_posture_digest(),
        Kind::TopologyQueryPostureMismatch,
        "topology Query posture digest",
        counters,
        "overlap request requires admitted topology Query posture",
    )?;
    reject_missing(
        readiness_consumer.spatial_query_posture_digest(),
        Kind::SpatialQueryPostureMismatch,
        "spatial Query posture digest",
        counters,
        "overlap request requires admitted spatial Query posture",
    )?;
    reject_missing(
        readiness_consumer.residue_digest(),
        Kind::ResidueMismatch,
        "residue digest",
        counters,
        "overlap request requires admitted residue authority",
    )?;
    reject_missing(
        readiness_consumer.source_firewall_digest(),
        Kind::SourceFirewallMismatch,
        "source firewall digest",
        counters,
        "overlap request requires admitted source-firewall authority",
    )?;
    reject_missing(
        readiness_consumer.architecture_claim_digest(),
        Kind::ArchitectureClaimMismatch,
        "architecture claim digest",
        counters,
        "overlap request requires admitted architecture-claim authority",
    )?;
    reject_mismatch(
        readiness_consumer.selected_route_identity_digest(),
        loop_ledger_receipt.selected_route_identity_digest(),
        Kind::SelectedRouteIdentityMismatch,
        "selected route identity",
        counters,
        "overlap request requires the admitted selected-route identity to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.selected_family_identity(),
        loop_ledger_receipt.selected_family_identity(),
        Kind::SelectedFamilyIdentityMismatch,
        "selected family identity",
        counters,
        "overlap request requires the admitted selected-family identity to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.selected_product_identity_digest(),
        loop_ledger_receipt.selected_product_identity_digest(),
        Kind::SelectedProductIdentityMismatch,
        "selected product identity",
        counters,
        "overlap request requires the admitted selected-product identity to match loop-ledger provenance",
    )?;
    reject_optional_mismatch(
        readiness_consumer.selected_witness_identity_digest(),
        loop_ledger_receipt.selected_witness_identity_digest(),
        Kind::SelectedWitnessIdentityMismatch,
        "selected witness identity",
        counters,
        "overlap request requires the admitted selected-witness identity to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.touched_closure_digest(),
        loop_ledger_receipt.touched_closure_digest(),
        Kind::TouchedClosureMismatch,
        "touched closure digest",
        counters,
        "overlap request requires the admitted touched-closure digest to match loop-ledger provenance",
    )?;
    reject_vec_mismatch(
        readiness_consumer.overlap_identity_digests(),
        loop_ledger_receipt.overlap_identity_digests(),
        Kind::OverlapIdentityMismatch,
        "overlap identity digests",
        counters,
        "overlap request requires admitted overlap-identity digests to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.topology_query_posture_digest(),
        loop_ledger_receipt.topology_query_posture_digest(),
        Kind::TopologyQueryPostureMismatch,
        "topology Query posture digest",
        counters,
        "overlap request requires admitted topology Query posture to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.spatial_query_posture_digest(),
        loop_ledger_receipt.spatial_query_posture_digest(),
        Kind::SpatialQueryPostureMismatch,
        "spatial Query posture digest",
        counters,
        "overlap request requires admitted spatial Query posture to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.residue_digest(),
        loop_ledger_receipt.residue_digest(),
        Kind::ResidueMismatch,
        "residue digest",
        counters,
        "overlap request requires admitted residue digest to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.source_firewall_digest(),
        loop_ledger_receipt.source_firewall_digest(),
        Kind::SourceFirewallMismatch,
        "source firewall digest",
        counters,
        "overlap request requires admitted source-firewall digest to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.architecture_claim_digest(),
        loop_ledger_receipt.architecture_claim_digest(),
        Kind::ArchitectureClaimMismatch,
        "architecture claim digest",
        counters,
        "overlap request requires admitted architecture-claim digest to match loop-ledger provenance",
    )?;
    reject_mismatch(
        readiness_consumer.selected_plan_digest(),
        loop_ledger_receipt.selected_plan_digest(),
        Kind::SelectedPlanDigestMismatch,
        "selected plan digest",
        counters,
        "overlap request requires the admitted readiness selected-plan digest to match the loop-ledger selected-plan provenance",
    )?;
    Ok(())
}

fn reject_optional_mismatch(
    left: Option<&str>,
    right: Option<&str>,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
    if left != right {
        counters.rejected_provenance_mismatch();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_vec_mismatch(
    left: &[String],
    right: &[String],
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
    if left != right {
        counters.rejected_provenance_mismatch();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_missing(
    observed: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
    if observed.is_empty() {
        counters.rejected_missing_authority();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

fn reject_mismatch(
    left: &str,
    right: &str,
    kind: Kind,
    rejected_identity: &'static str,
    counters: &mut PlanarBooleanOverlapReadinessLoopLedgerBindingCounters,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanOverlapReadinessLoopLedgerBindingDenial> {
    if left != right {
        counters.rejected_provenance_mismatch();
        return Err(PlanarBooleanOverlapReadinessLoopLedgerBindingDenial::new(
            kind,
            rejected_identity,
            *counters,
            human_reason,
        ));
    }
    Ok(())
}

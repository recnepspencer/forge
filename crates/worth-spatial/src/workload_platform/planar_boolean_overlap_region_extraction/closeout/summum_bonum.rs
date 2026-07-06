use std::collections::{BTreeMap, BTreeSet};

use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as Kind,
};
use super::subcases::{
    PlanarBooleanOverlapRegionSummumBonumSubcaseKind as SubcaseKind,
    PlanarBooleanOverlapRegionSummumBonumSubcaseRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapReadinessLoopLedgerBinding,
    PlanarBooleanOverlapRegionCheckpointParityReceipt, PlanarBooleanOverlapRegionDecisionKind,
    PlanarBooleanOverlapRegionEvidenceReceipt, PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapRegionReplayParityReceipt, PlanarBooleanOverlapRegionReplayParityRowKind,
};

const PHASE_FIFTEEN_FENCE_SUBCASE: &str = "phase_fifteen_fence";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    readiness_inputs_consumed: usize,
    overlap_ledger_receipts_consumed: usize,
    replay_rows_verified: usize,
    decision_rows_verified: usize,
    ledger_rows_verified: usize,
    boundary_only_rows_verified: usize,
    area_rows_verified: usize,
    mixed_boundary_area_rows_verified: usize,
    pairwise_rediscovery_attempts: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    readiness: &'a TouchedGraphParityReadinessInput,
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
    overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
    replay_parity_receipt: &'a PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: &'a PlanarBooleanOverlapRegionCheckpointParityReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseout {
    closeout_identity: String,
    request_identity: String,
    readiness_handoff_identity: String,
    readiness_consumer_identity: String,
    readiness_binding_identity: String,
    loop_ledger_receipt_identity: String,
    overlap_ledger_receipt_identity: String,
    overlap_ledger_identity: String,
    overlap_decision_log_identity: String,
    overlap_ordering_basis_identity: String,
    overlap_region_identity_map_identity: String,
    persistent_name_propagation_map_identity: String,
    subshape_signature_map_identity: String,
    replay_identity: String,
    checkpoint_identity: String,
    replay_evidence_identity: String,
    subcases: Vec<PlanarBooleanOverlapRegionSummumBonumSubcaseRow>,
    counters: PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
}

impl<'a> PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        readiness: &'a TouchedGraphParityReadinessInput,
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
        overlap_ledger_bundle: &'a PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_parity_receipt: &'a PlanarBooleanOverlapRegionReplayParityReceipt,
        checkpoint_parity_receipt: &'a PlanarBooleanOverlapRegionCheckpointParityReceipt,
    ) -> Self {
        Self {
            readiness,
            readiness_consumer,
            readiness_binding,
            overlap_ledger_bundle,
            evidence_receipt,
            replay_parity_receipt,
            checkpoint_parity_receipt,
        }
    }
}

impl PlanarBooleanOverlapRegionSummumBonumCloseout {
    pub fn certify(
        input: PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
        verify_readiness_consumer(input.readiness, input.readiness_consumer)?;
        verify_binding(
            input.readiness,
            input.readiness_consumer,
            input.readiness_binding,
        )?;
        verify_receipt_chain(
            input.readiness_binding,
            input.overlap_ledger_bundle,
            input.evidence_receipt,
        )?;
        let replay_rows_verified = verify_replay_and_checkpoint(
            input.replay_parity_receipt,
            input.checkpoint_parity_receipt,
            input.evidence_receipt,
        )?;
        verify_canonical_identities(input.overlap_ledger_bundle, input.evidence_receipt)?;

        let counters = build_counters(input.overlap_ledger_bundle, replay_rows_verified);
        let subcases = certify_subcases(
            input.overlap_ledger_bundle,
            input.replay_parity_receipt,
            counters,
        )?;
        let receipt = input.overlap_ledger_bundle.receipt();
        let ledger = input.overlap_ledger_bundle.ledger();
        Ok(Self {
            closeout_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-overlap-region-summum-bonum-closeout".to_string(),
                    format!("request:{}", input.evidence_receipt.request_identity()),
                    format!("overlap-ledger-receipt:{}", receipt.receipt_identity()),
                    format!("overlap-ledger:{}", ledger.ledger_identity()),
                    format!("replay:{}", input.replay_parity_receipt.replay_identity()),
                    format!(
                        "checkpoint:{}",
                        input.checkpoint_parity_receipt.checkpoint_identity()
                    ),
                ],
            ),
            request_identity: input.evidence_receipt.request_identity().to_string(),
            readiness_handoff_identity: input
                .evidence_receipt
                .readiness_handoff_identity()
                .to_string(),
            readiness_consumer_identity: input
                .evidence_receipt
                .readiness_consumer_identity()
                .to_string(),
            readiness_binding_identity: input
                .evidence_receipt
                .readiness_binding_identity()
                .to_string(),
            loop_ledger_receipt_identity: input
                .evidence_receipt
                .loop_ledger_receipt_identity()
                .to_string(),
            overlap_ledger_receipt_identity: receipt.receipt_identity().to_string(),
            overlap_ledger_identity: ledger.ledger_identity().to_string(),
            overlap_decision_log_identity: input
                .overlap_ledger_bundle
                .decision_log()
                .decision_log_identity()
                .to_string(),
            overlap_ordering_basis_identity: ledger.ordering_basis_identity().to_string(),
            overlap_region_identity_map_identity: receipt
                .overlap_region_identity_map_identity()
                .to_string(),
            persistent_name_propagation_map_identity: receipt
                .persistent_name_propagation_map_identity()
                .to_string(),
            subshape_signature_map_identity: receipt.subshape_signature_map_identity().to_string(),
            replay_identity: input.replay_parity_receipt.replay_identity().to_string(),
            checkpoint_identity: input
                .checkpoint_parity_receipt
                .checkpoint_identity()
                .to_string(),
            replay_evidence_identity: input
                .checkpoint_parity_receipt
                .replay_evidence_identity()
                .to_string(),
            subcases,
            counters,
        })
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }
    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }
    pub fn overlap_region_identity_map_identity(&self) -> &str {
        &self.overlap_region_identity_map_identity
    }
    pub fn persistent_name_propagation_map_identity(&self) -> &str {
        &self.persistent_name_propagation_map_identity
    }
    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }
    pub fn replay_identity(&self) -> &str {
        &self.replay_identity
    }
    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }
    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
    pub fn counters(&self) -> PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
        self.counters
    }
    pub fn subcases(&self) -> &[PlanarBooleanOverlapRegionSummumBonumSubcaseRow] {
        &self.subcases
    }

    pub fn subcase(
        &self,
        kind: SubcaseKind,
    ) -> Option<&PlanarBooleanOverlapRegionSummumBonumSubcaseRow> {
        self.subcases.iter().find(|row| row.kind() == kind)
    }

    pub fn is_canonical(&self) -> bool {
        !self.closeout_identity.is_empty()
            && !self.overlap_region_identity_map_identity.is_empty()
            && !self.persistent_name_propagation_map_identity.is_empty()
            && !self.subshape_signature_map_identity.is_empty()
            && self.counters.pairwise_rediscovery_attempts == 0
    }
}

impl PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    pub fn readiness_inputs_consumed(self) -> usize {
        self.readiness_inputs_consumed
    }
    pub fn overlap_ledger_receipts_consumed(self) -> usize {
        self.overlap_ledger_receipts_consumed
    }
    pub fn replay_rows_verified(self) -> usize {
        self.replay_rows_verified
    }
    pub fn decision_rows_verified(self) -> usize {
        self.decision_rows_verified
    }
    pub fn ledger_rows_verified(self) -> usize {
        self.ledger_rows_verified
    }
    pub fn boundary_only_rows_verified(self) -> usize {
        self.boundary_only_rows_verified
    }
    pub fn area_rows_verified(self) -> usize {
        self.area_rows_verified
    }
    pub fn mixed_boundary_area_rows_verified(self) -> usize {
        self.mixed_boundary_area_rows_verified
    }
    pub fn pairwise_rediscovery_attempts(self) -> usize {
        self.pairwise_rediscovery_attempts
    }
}

fn verify_readiness_consumer(
    readiness: &TouchedGraphParityReadinessInput,
    consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let matches = readiness.selected_route_identity_digest()
        == consumer.selected_route_identity_digest()
        && readiness.selected_family_identity() == consumer.selected_family_identity()
        && readiness.selected_product_identity_digest()
            == consumer.selected_product_identity_digest()
        && readiness.selected_witness_identity_digest()
            == consumer.selected_witness_identity_digest()
        && readiness.touched_closure_digest() == consumer.touched_closure_digest()
        && readiness.selected_plan_digest() == consumer.selected_plan_digest()
        && readiness.topology_query_posture_digest() == consumer.topology_query_posture_digest()
        && readiness.spatial_query_posture_digest() == consumer.spatial_query_posture_digest()
        && readiness.residue_digest() == consumer.residue_digest()
        && readiness.source_firewall_digest() == consumer.source_firewall_digest()
        && readiness.architecture_claim_digest() == consumer.architecture_claim_digest();
    if matches {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReadinessConsumerMismatch,
            PHASE_FIFTEEN_FENCE_SUBCASE,
            "phase-16 closeout requires one matching readiness handoff and readiness consumer",
        ))
    }
}

fn verify_binding(
    readiness: &TouchedGraphParityReadinessInput,
    consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    binding: &PlanarBooleanOverlapReadinessLoopLedgerBinding,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let matches = binding.selected_route_identity_digest()
        == readiness.selected_route_identity_digest()
        && binding.selected_route_identity_digest() == consumer.selected_route_identity_digest()
        && binding.selected_family_identity() == readiness.selected_family_identity()
        && binding.selected_family_identity() == consumer.selected_family_identity()
        && binding.selected_product_identity_digest()
            == readiness.selected_product_identity_digest()
        && binding.selected_product_identity_digest()
            == consumer.selected_product_identity_digest()
        && binding.selected_witness_identity_digest()
            == readiness.selected_witness_identity_digest()
        && binding.selected_witness_identity_digest()
            == consumer.selected_witness_identity_digest()
        && binding.selected_plan_digest() == readiness.selected_plan_digest()
        && binding.selected_plan_digest() == consumer.selected_plan_digest()
        && binding.touched_closure_digest() == readiness.touched_closure_digest()
        && binding.touched_closure_digest() == consumer.touched_closure_digest()
        && binding.topology_query_posture_digest() == readiness.topology_query_posture_digest()
        && binding.topology_query_posture_digest() == consumer.topology_query_posture_digest()
        && binding.spatial_query_posture_digest() == readiness.spatial_query_posture_digest()
        && binding.spatial_query_posture_digest() == consumer.spatial_query_posture_digest()
        && binding.residue_digest() == readiness.residue_digest()
        && binding.residue_digest() == consumer.residue_digest()
        && binding.source_firewall_digest() == readiness.source_firewall_digest()
        && binding.source_firewall_digest() == consumer.source_firewall_digest()
        && binding.architecture_claim_digest() == readiness.architecture_claim_digest()
        && binding.architecture_claim_digest() == consumer.architecture_claim_digest();
    if matches {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::ReadinessBindingMismatch, "synthetic_readiness_or_mismatched_loop_ledger_is_rejected", "phase-16 closeout requires the carried readiness-to-loop binding to match readiness authority"))
    }
}

fn verify_receipt_chain(
    binding: &PlanarBooleanOverlapReadinessLoopLedgerBinding,
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let receipt = bundle.receipt();
    if binding.loop_ledger_receipt_identity() != evidence_receipt.loop_ledger_receipt_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::LoopLedgerMismatch,
            "synthetic_readiness_or_mismatched_loop_ledger_is_rejected",
            "phase-16 closeout requires one readiness-bound loop-ledger receipt identity",
        ));
    }
    if receipt.receipt_identity() != evidence_receipt.overlap_ledger_receipt_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OverlapLedgerMismatch,
            "synthetic_overlap_ledger_is_rejected",
            "phase-16 closeout requires one overlap-ledger receipt carried by evidence",
        ));
    }
    if receipt.request_identity() != evidence_receipt.request_identity() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::RequestIdentityMismatch,
            "synthetic_overlap_ledger_is_rejected",
            "phase-16 closeout requires one request identity across overlap ledger and evidence",
        ));
    }
    Ok(())
}

fn verify_replay_and_checkpoint(
    replay_parity_receipt: &PlanarBooleanOverlapRegionReplayParityReceipt,
    checkpoint_parity_receipt: &PlanarBooleanOverlapRegionCheckpointParityReceipt,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<usize, PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    if replay_parity_receipt.checkpoint_receipt() != checkpoint_parity_receipt {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::ReplayParityMismatch,
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
            "phase-16 closeout requires replay parity to carry the same checkpoint receipt",
        ));
    }
    if checkpoint_parity_receipt.replay_evidence_identity()
        != evidence_receipt.replay_evidence_identity()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::CheckpointParityMismatch, SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(), "phase-16 closeout requires checkpoint parity to stay bound to retained replay evidence"));
    }
    for kind in [
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapEvidenceReceipt,
        PlanarBooleanOverlapRegionReplayParityRowKind::RequestIdentity,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessHandoff,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessConsumer,
        PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessBinding,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapDecisionLog,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapLedgerReceipt,
        PlanarBooleanOverlapRegionReplayParityRowKind::OverlapIdentityMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::PersistentNamePropagationMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::SubshapeSignatureMap,
        PlanarBooleanOverlapRegionReplayParityRowKind::RetainedReplayCheckpoint,
    ] {
        if replay_parity_receipt
            .rows()
            .iter()
            .all(|row| row.kind() != kind)
        {
            return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
                Kind::MissingReplayParityRow,
                SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames.spec_name(),
                format!("phase-16 closeout is missing replay row {kind:?}"),
            ));
        }
    }
    Ok(replay_parity_receipt.rows().len())
}

fn verify_canonical_identities(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    evidence_receipt: &PlanarBooleanOverlapRegionEvidenceReceipt,
) -> Result<(), PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let receipt = bundle.receipt();
    let identities = [
        receipt.overlap_region_identity_map_identity(),
        receipt.persistent_name_propagation_map_identity(),
        receipt.subshape_signature_map_identity(),
        evidence_receipt.overlap_region_identity_map_identity(),
        evidence_receipt.persistent_name_propagation_map_identity(),
        evidence_receipt.subshape_signature_map_identity(),
    ];
    if identities.iter().all(|identity| !identity.is_empty()) {
        Ok(())
    } else {
        Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::MissingCanonicalIdentity, PHASE_FIFTEEN_FENCE_SUBCASE, "phase-16 closeout requires canonical overlap identity, persistent-name, and subshape-signature authority"))
    }
}

fn build_counters(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replay_rows_verified: usize,
) -> PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    let boundary_only_rows_verified = bundle
        .ledger()
        .rows()
        .iter()
        .filter(|row| row.correspondence_only())
        .count();
    let area_rows_verified = bundle
        .ledger()
        .rows()
        .iter()
        .filter(|row| row.area_overlap_component_identity().is_some())
        .count();
    let mixed_boundary_area_rows_verified =
        usize::from(boundary_only_rows_verified > 0 && area_rows_verified > 0);
    let bundle_counters = bundle.counters();
    let pairwise_rediscovery_attempts = bundle_counters.decision_rows_admitted().saturating_sub(
        bundle_counters.identity_rows_examined() + bundle_counters.ledger_rows_admitted(),
    );
    PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
        readiness_inputs_consumed: 1,
        overlap_ledger_receipts_consumed: 1,
        replay_rows_verified,
        decision_rows_verified: bundle_counters.decision_rows_admitted(),
        ledger_rows_verified: bundle_counters.ledger_rows_admitted(),
        boundary_only_rows_verified,
        area_rows_verified,
        mixed_boundary_area_rows_verified,
        pairwise_rediscovery_attempts,
    }
}

fn certify_subcases(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replay_parity_receipt: &PlanarBooleanOverlapRegionReplayParityReceipt,
    counters: PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
) -> Result<
    Vec<PlanarBooleanOverlapRegionSummumBonumSubcaseRow>,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
> {
    let ledger_rows = bundle.ledger().rows();
    let boundary_only_rows = ledger_rows
        .iter()
        .filter(|row| row.correspondence_only())
        .count();
    let area_rows = ledger_rows
        .iter()
        .filter(|row| row.area_overlap_component_identity().is_some())
        .count();
    let all_boundary_only_are_non_area = ledger_rows
        .iter()
        .filter(|row| row.correspondence_only())
        .all(|row| row.area_overlap_component_identity().is_none());
    if boundary_only_rows == 0 || !all_boundary_only_are_non_area {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::BoundaryOnlyAreaAdmission, SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea.spec_name(), "phase-16 closeout requires carried boundary-only rows that never admit an area-overlap component"));
    }

    let mut region_to_winding = BTreeMap::new();
    for row in ledger_rows {
        match region_to_winding.insert(row.region_identity(), row.canonical_winding_identity()) {
            Some(previous) if previous != row.canonical_winding_identity() => {
                return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::OppositeSenseWindingInstability, SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding.spec_name(), "phase-16 closeout requires one stable canonical winding identity per admitted overlap region"));
            }
            _ => {}
        }
    }
    if area_rows == 0 || region_to_winding.is_empty() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::OppositeSenseWindingInstability, SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding.spec_name(), "phase-16 closeout requires admitted area-overlap rows with canonical winding authority"));
    }

    let has_lineage = ledger_rows
        .iter()
        .all(|row| !row.lineage_identities().is_empty());
    let has_nested_lineage = ledger_rows.iter().any(|row| {
        row.lineage_identities().len() > 1 || row.canonical_source_loop_identities().len() > 1
    });
    if !has_lineage || !has_nested_lineage {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::NestedIdentityInstability, SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity.spec_name(), "phase-16 closeout requires carried overlap rows whose lineage preserves nested region identity"));
    }

    let decision_kinds = bundle
        .decision_log()
        .rows()
        .iter()
        .map(|row| row.kind())
        .collect::<BTreeSet<PlanarBooleanOverlapRegionDecisionKind>>();
    if area_rows == 0
        || boundary_only_rows == 0
        || !decision_kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::BoundaryOnly)
        || !decision_kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Area)
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::MixedBoundaryAreaCollapse, SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse.spec_name(), "phase-16 closeout requires carried boundary-only and area-overlap authority without collapsing them into one outcome"));
    }

    if bundle.ledger().ordering_basis_identity().is_empty()
        || replay_parity_receipt.rows().is_empty()
        || bundle.ledger().ledger_identity().is_empty()
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OrderingParityInstability,
            SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest.spec_name(),
            "phase-16 closeout requires a stable ordering basis and ledger digest surface",
        ));
    }

    if counters.pairwise_rediscovery_attempts != 0
        || counters.decision_rows_verified < counters.ledger_rows_verified
        || counters.boundary_only_rows_verified == 0
        || counters.area_rows_verified == 0
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(Kind::OverlapStormShapeViolation, SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery.spec_name(), "phase-16 closeout requires workload-backed storm counters that stay on the indexed overlap-ledger path"));
    }

    Ok(vec![
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea,
            format!("boundary_only_rows={boundary_only_rows}"),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding,
            format!("stable_regions={}", region_to_winding.len()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity,
            "nested lineage carried by admitted overlap rows",
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse,
            format!("boundary_only_rows={boundary_only_rows}; area_rows={area_rows}"),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::BenignLoopOrderVariationPreservesLedgerDigest,
            format!(
                "ordering_basis={}",
                bundle.ledger().ordering_basis_identity()
            ),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::CheckpointReplayPreservesRegionIdentityAndNames,
            format!("replay_rows={}", replay_parity_receipt.rows().len()),
        ),
        PlanarBooleanOverlapRegionSummumBonumSubcaseRow::new(
            SubcaseKind::OverlapStormUsesIndexNotPairwiseRediscovery,
            format!(
                "decision_rows={}; ledger_rows={}",
                counters.decision_rows_verified, counters.ledger_rows_verified
            ),
        ),
    ])
}

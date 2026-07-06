use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReconstructionParticipationSupport,
    PlanarBooleanLoopReplayParityInput, PlanarBooleanLoopReplayParityReceipt,
};
use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle, PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapAdjacencyIndexInput, PlanarBooleanOverlapArrangementGraphInput,
    PlanarBooleanOverlapCellContainmentInput, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField, PlanarBooleanOverlapCellWindingFieldInput,
    PlanarBooleanOverlapIslandCandidateInput, PlanarBooleanOverlapIslandComponentBundle,
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryInput,
    PlanarBooleanOverlapRegionAdjacencyIndex, PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapRegionExtractionRequestInput, PlanarBooleanOverlapRegionLedgerReceipt,
    PlanarBooleanPreRegionNormalizationBundle, PlanarBooleanSharedAreaAdmissionBundle,
};
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;

use crate::workload_composition::CompletedBooleanLoopReconstructionHandoff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanOverlapReplayCertifiedPeerDenial {
    LoopReplayParityRejected(String),
    MissingReplayLoopProducts,
    ReplayRequestRejected(String),
    ReplayParticipationRejected(String),
    ReplayAdjacencyRejected(String),
    ReplayArrangementRejected(String),
    ReplayContainmentRejected(String),
    ReplayWindingRejected(String),
    ReplayIslandComponentRejected(String),
    ReplayBoundaryContactRejected(String),
    ReplaySharedAreaRejected(String),
    ReplayPreRegionNormalizationRejected(String),
    ReplayCandidatePromotionRejected(String),
    ReplayCanonicalWindingRejected(String),
    ReplayIdentityLineageRejected(String),
    ReplayLedgerRejected(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanOverlapReplayCertifiedPeer {
    loop_replay_parity_receipt: PlanarBooleanLoopReplayParityReceipt,
    replay_receipts: ReplayReceiptSet,
    replayed_overlap_request: PlanarBooleanOverlapRegionExtractionRequest,
    replayed_overlap_ledger_receipt: PlanarBooleanOverlapRegionLedgerReceipt,
}

impl PlanarBooleanOverlapReplayCertifiedPeer {
    pub(crate) fn certify_from_loop_handoffs(
        original_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
        replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
        readiness_consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        replay_receipts: &ReplayReceiptSet,
    ) -> Result<Self, PlanarBooleanOverlapReplayCertifiedPeerDenial> {
        let loop_replay_parity_receipt = ComparePlanarBooleanLoopReplayParity::compare(
            PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
                original_loop_handoff.loop_ledger_receipt(),
                replayed_loop_handoff.loop_ledger_receipt(),
                original_loop_handoff.evidence_receipt(),
                replayed_loop_handoff.evidence_receipt(),
                replay_receipts,
            )
            .map_err(|denial| {
                PlanarBooleanOverlapReplayCertifiedPeerDenial::LoopReplayParityRejected(format!(
                    "{denial:?}"
                ))
            })?,
        )
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::LoopReplayParityRejected(format!(
                "{denial:?}"
            ))
        })?;
        let replayed_overlap_request = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                readiness_consumer,
                replayed_loop_handoff.loop_ledger_receipt(),
            ),
        )
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayRequestRejected(format!(
                "{denial:?}"
            ))
        })?;
        let replayed_overlap_ledger_receipt = derive_replayed_overlap_ledger_receipt(
            replayed_loop_handoff,
            &replayed_overlap_request,
        )?;
        Ok(Self {
            loop_replay_parity_receipt,
            replay_receipts: replay_receipts.clone(),
            replayed_overlap_request,
            replayed_overlap_ledger_receipt,
        })
    }

    pub(crate) fn loop_replay_parity_receipt(&self) -> &PlanarBooleanLoopReplayParityReceipt {
        &self.loop_replay_parity_receipt
    }

    pub(crate) fn replay_receipts(&self) -> &ReplayReceiptSet {
        &self.replay_receipts
    }

    pub(crate) fn replayed_overlap_request(&self) -> &PlanarBooleanOverlapRegionExtractionRequest {
        &self.replayed_overlap_request
    }

    pub(crate) fn replayed_overlap_ledger_receipt(
        &self,
    ) -> &PlanarBooleanOverlapRegionLedgerReceipt {
        &self.replayed_overlap_ledger_receipt
    }
}

fn derive_replayed_overlap_ledger_receipt(
    replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replayed_overlap_request: &PlanarBooleanOverlapRegionExtractionRequest,
) -> Result<PlanarBooleanOverlapRegionLedgerReceipt, PlanarBooleanOverlapReplayCertifiedPeerDenial>
{
    let replayed_loop_products = replayed_loop_handoff
        .products()
        .ok_or(PlanarBooleanOverlapReplayCertifiedPeerDenial::MissingReplayLoopProducts)?;
    let participation_support =
        PlanarBooleanLoopReconstructionParticipationSupport::admit_from_ledger_and_products(
            replayed_loop_products.loop_ledger(),
            replayed_loop_products.role_outcomes(),
            replayed_loop_products.island_partition(),
            replayed_loop_products.persistent_name_propagation_map(),
            replayed_loop_products
                .source_provenance()
                .fragment_membership_map(),
            replayed_loop_products
                .source_provenance()
                .overlap_chain_lineage_map(),
            replayed_loop_products
                .source_provenance()
                .source_loop_carriers(),
        )
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayParticipationRejected(format!(
                "{denial:?}"
            ))
        })?;
    let participation = PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            replayed_overlap_request,
            &participation_support,
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayParticipationRejected(format!(
            "{denial:?}"
        ))
    })?;
    let adjacency = PlanarBooleanOverlapRegionAdjacencyIndex::admit(
        PlanarBooleanOverlapAdjacencyIndexInput::from_participation_products(
            participation.loop_participation_map(),
            participation.island_participation_map(),
            participation.chain_lineage_map(),
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayAdjacencyRejected(format!(
            "{denial:?}"
        ))
    })?;
    let arrangement = PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            &adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayArrangementRejected(format!(
            "{denial:?}"
        ))
    })?;
    let shared_area_bundle = shared_area_bundle_from_arrangement(&arrangement)?;
    let pre_region_normalization_bundle =
        PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
            &shared_area_bundle,
            participation.chain_lineage_map(),
        )
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayPreRegionNormalizationRejected(
                format!("{denial:?}"),
            )
        })?;
    let overlap_region_candidates = pre_region_normalization_bundle
        .promote_overlap_region_candidates(&shared_area_bundle)
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayCandidatePromotionRejected(
                format!("{denial:?}"),
            )
        })?;
    let canonical_winding_bundle = overlap_region_candidates
        .normalize_post_admission_canonical_winding()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayCanonicalWindingRejected(format!(
                "{denial:?}"
            ))
        })?;
    let identity_lineage_bundle = canonical_winding_bundle
        .mint_overlap_region_identity_lineage()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayIdentityLineageRejected(format!(
                "{denial:?}"
            ))
        })?;
    let replayed_overlap_ledger_bundle = identity_lineage_bundle
        .mint_overlap_region_ledger()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayLedgerRejected(format!(
                "{denial:?}"
            ))
        })?;
    Ok(replayed_overlap_ledger_bundle.receipt().clone())
}

fn shared_area_bundle_from_arrangement(
    arrangement: &PlanarBooleanCoplanarOverlapArrangementGraph,
) -> Result<PlanarBooleanSharedAreaAdmissionBundle, PlanarBooleanOverlapReplayCertifiedPeerDenial> {
    let containment = PlanarBooleanOverlapCellContainmentMap::admit(
        PlanarBooleanOverlapCellContainmentInput::from_arrangement(arrangement),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayContainmentRejected(format!(
            "{denial:?}"
        ))
    })?;
    let winding = PlanarBooleanOverlapCellWindingField::admit(
        PlanarBooleanOverlapCellWindingFieldInput::from_arrangement(arrangement, &containment),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayWindingRejected(format!("{denial:?}"))
    })?;
    let island_component_bundle = PlanarBooleanOverlapIslandComponentBundle::admit(
        PlanarBooleanOverlapIslandCandidateInput::from_cell_classification(
            arrangement,
            &containment,
            &winding,
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayIslandComponentRejected(format!(
            "{denial:?}"
        ))
    })?;
    let boundary_contact_bundle: PlanarBooleanBoundaryContactClassificationBundle =
        island_component_bundle
            .classify_boundary_contact_components()
            .map_err(|denial| {
                PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayBoundaryContactRejected(
                    format!("{denial:?}"),
                )
            })?;
    boundary_contact_bundle
        .admit_shared_area_components(&containment, &winding)
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplaySharedAreaRejected(format!(
                "{denial:?}"
            ))
        })
}

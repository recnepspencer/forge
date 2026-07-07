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
    PlanarBooleanOverlapRegionExtractionRequestInput,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionLedgerReceipt,
    PlanarBooleanPostAdmissionNormalizationBundle, PlanarBooleanPreRegionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
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
    replayed_shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
    replayed_canonical_winding_bundle: PlanarBooleanPostAdmissionNormalizationBundle,
    replayed_overlap_ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
}

struct ReplayedOverlapAuthorityProducts {
    shared_area_bundle: PlanarBooleanSharedAreaAdmissionBundle,
    canonical_winding_bundle: PlanarBooleanPostAdmissionNormalizationBundle,
    ledger_bundle: PlanarBooleanOverlapRegionLedgerAssemblyBundle,
}

impl PlanarBooleanOverlapReplayCertifiedPeer {
    pub(crate) fn certify_from_loop_handoffs(
        original_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
        replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
        readiness_consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        replay_receipts: &ReplayReceiptSet,
    ) -> Result<Self, PlanarBooleanOverlapReplayCertifiedPeerDenial> {
        let loop_replay_parity_receipt = certify_loop_replay_parity(
            original_loop_handoff,
            replayed_loop_handoff,
            replay_receipts,
        )?;
        let replayed_overlap_request =
            admit_replayed_overlap_request(readiness_consumer, replayed_loop_handoff)?;
        let replayed_products = derive_replayed_overlap_authority_products(
            replayed_loop_handoff,
            &replayed_overlap_request,
        )?;
        Ok(Self {
            loop_replay_parity_receipt,
            replay_receipts: replay_receipts.clone(),
            replayed_overlap_request,
            replayed_shared_area_bundle: replayed_products.shared_area_bundle,
            replayed_canonical_winding_bundle: replayed_products.canonical_winding_bundle,
            replayed_overlap_ledger_bundle: replayed_products.ledger_bundle,
        })
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
        self.replayed_overlap_ledger_bundle.receipt()
    }

    pub(crate) fn replayed_overlap_ledger_bundle(
        &self,
    ) -> &PlanarBooleanOverlapRegionLedgerAssemblyBundle {
        &self.replayed_overlap_ledger_bundle
    }

    pub(crate) fn replayed_shared_area_bundle(&self) -> &PlanarBooleanSharedAreaAdmissionBundle {
        &self.replayed_shared_area_bundle
    }

    pub(crate) fn replayed_canonical_winding_bundle(
        &self,
    ) -> &PlanarBooleanPostAdmissionNormalizationBundle {
        &self.replayed_canonical_winding_bundle
    }
}

fn certify_loop_replay_parity(
    original_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replay_receipts: &ReplayReceiptSet,
) -> Result<PlanarBooleanLoopReplayParityReceipt, PlanarBooleanOverlapReplayCertifiedPeerDenial> {
    let parity_input = PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
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
    })?;

    ComparePlanarBooleanLoopReplayParity::compare(parity_input).map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::LoopReplayParityRejected(format!(
            "{denial:?}"
        ))
    })
}

fn admit_replayed_overlap_request(
    readiness_consumer: &TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> Result<
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanOverlapReplayCertifiedPeerDenial,
> {
    PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            readiness_consumer,
            replayed_loop_handoff.loop_ledger_receipt(),
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayRequestRejected(format!("{denial:?}"))
    })
}

fn derive_replayed_overlap_authority_products(
    replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replayed_overlap_request: &PlanarBooleanOverlapRegionExtractionRequest,
) -> Result<ReplayedOverlapAuthorityProducts, PlanarBooleanOverlapReplayCertifiedPeerDenial> {
    let participation =
        recover_replayed_overlap_participation(replayed_loop_handoff, replayed_overlap_request)?;
    let adjacency = build_replayed_overlap_adjacency(&participation)?;
    let arrangement = build_replayed_overlap_arrangement(&adjacency)?;
    let shared_area_bundle = shared_area_bundle_from_arrangement(&arrangement)?;
    let canonical_winding_bundle =
        normalize_replayed_overlap_regions(&participation, &shared_area_bundle)?;
    let ledger_bundle = mint_replayed_overlap_ledger(&canonical_winding_bundle)?;
    Ok(ReplayedOverlapAuthorityProducts {
        shared_area_bundle,
        canonical_winding_bundle,
        ledger_bundle,
    })
}

fn recover_replayed_overlap_participation(
    replayed_loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replayed_overlap_request: &PlanarBooleanOverlapRegionExtractionRequest,
) -> Result<PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapReplayCertifiedPeerDenial>
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
    PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            replayed_overlap_request,
            &participation_support,
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayParticipationRejected(format!(
            "{denial:?}"
        ))
    })
}

fn build_replayed_overlap_adjacency(
    participation: &PlanarBooleanOverlapParticipationRecovery,
) -> Result<PlanarBooleanOverlapRegionAdjacencyIndex, PlanarBooleanOverlapReplayCertifiedPeerDenial>
{
    PlanarBooleanOverlapRegionAdjacencyIndex::admit(
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
    })
}

fn build_replayed_overlap_arrangement(
    adjacency: &PlanarBooleanOverlapRegionAdjacencyIndex,
) -> Result<
    PlanarBooleanCoplanarOverlapArrangementGraph,
    PlanarBooleanOverlapReplayCertifiedPeerDenial,
> {
    PlanarBooleanCoplanarOverlapArrangementGraph::admit(
        PlanarBooleanOverlapArrangementGraphInput::from_adjacency(
            adjacency,
            adjacency.ordering_basis(),
        ),
    )
    .map_err(|denial| {
        PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayArrangementRejected(format!(
            "{denial:?}"
        ))
    })
}

fn normalize_replayed_overlap_regions(
    participation: &PlanarBooleanOverlapParticipationRecovery,
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
) -> Result<
    PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanOverlapReplayCertifiedPeerDenial,
> {
    let pre_region_normalization_bundle =
        PlanarBooleanPreRegionNormalizationBundle::from_shared_area_admission(
            shared_area_bundle,
            participation.chain_lineage_map(),
        )
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayPreRegionNormalizationRejected(
                format!("{denial:?}"),
            )
        })?;
    let overlap_region_candidates = pre_region_normalization_bundle
        .promote_overlap_region_candidates(shared_area_bundle)
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayCandidatePromotionRejected(
                format!("{denial:?}"),
            )
        })?;
    overlap_region_candidates
        .normalize_post_admission_canonical_winding()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayCanonicalWindingRejected(format!(
                "{denial:?}"
            ))
        })
}

fn mint_replayed_overlap_ledger(
    canonical_winding_bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
) -> Result<
    PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapReplayCertifiedPeerDenial,
> {
    let identity_lineage_bundle = canonical_winding_bundle
        .mint_overlap_region_identity_lineage()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayIdentityLineageRejected(format!(
                "{denial:?}"
            ))
        })?;
    identity_lineage_bundle
        .mint_overlap_region_ledger()
        .map_err(|denial| {
            PlanarBooleanOverlapReplayCertifiedPeerDenial::ReplayLedgerRejected(format!(
                "{denial:?}"
            ))
        })
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

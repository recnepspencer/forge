use topology::facade::{
    current_query_backed_consumer_residue_manifest, current_topology_consumer_residue_manifest,
};
use worth_spatial::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout_residue_manifest;
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::current_spatial_consumer_residue_manifest;

use super::cluster_ledger::{
    WorthWorkloadOrdinaryConsumerClusterKind, WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
use super::error::{
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
};
use crate::workload_composition::{
    planner_owned_routing::current_public_closeout_consumer_residue_manifest,
    ConflictBatchAdmissionInventory, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionReplacementPhase, ConflictBatchAdmissionSurfaceIdentity,
};

pub(super) fn collect_residue_rows(
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<
    Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow>,
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
> {
    let mut rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.disposition()
                != crate::workload_composition::ConflictBatchAdmissionDisposition::Migrate
        })
        .filter_map(classify_inventory_row_for_closeout)
        .map(|(cluster_kind, row)| {
            WorthWorkloadOrdinaryConsumerSweepResidueRow::from_inventory_row(cluster_kind, row)
        })
        .collect::<Vec<_>>();
    rows.extend(
        current_topology_consumer_residue_manifest()
            .iter()
            .map(WorthWorkloadOrdinaryConsumerSweepResidueRow::from_topology_residue_row),
    );
    rows.extend(
        current_query_backed_consumer_residue_manifest()
            .iter()
            .map(WorthWorkloadOrdinaryConsumerSweepResidueRow::from_query_backed_residue_row),
    );
    rows.extend(
        current_public_closeout_consumer_residue_manifest()
            .map_err(|error| {
                WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
                    format!(
                        "ordinary sweep closeout requires current public closeout residue manifest: {error:?}"
                    ),
                )
            })?
            .iter()
            .map(WorthWorkloadOrdinaryConsumerSweepResidueRow::from_public_closeout_residue_row),
    );
    rows.extend(
        current_evidence_lookup_public_closeout_residue_manifest()
            .iter()
            .map(
                WorthWorkloadOrdinaryConsumerSweepResidueRow::from_evidence_lookup_public_closeout_residue_row,
            ),
    );
    rows.extend(
        current_spatial_consumer_residue_manifest()
            .iter()
            .map(WorthWorkloadOrdinaryConsumerSweepResidueRow::from_spatial_residue_row),
    );
    Ok(rows)
}

fn classify_inventory_row_for_closeout(
    row: &ConflictBatchAdmissionInventoryRow,
) -> Option<(
    WorthWorkloadOrdinaryConsumerClusterKind,
    &ConflictBatchAdmissionInventoryRow,
)> {
    match row.replacement_phase() {
        ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep => {
            phase_eleven_cluster(row.surface_identity()).map(|cluster| (cluster, row))
        }
        ConflictBatchAdmissionReplacementPhase::PhaseThirteenPublicReadCloseoutCutover => {
            phase_thirteen_cluster(row.surface_identity()).map(|cluster| (cluster, row))
        }
        ConflictBatchAdmissionReplacementPhase::PhaseTwelveFirewallDeletion
        | ConflictBatchAdmissionReplacementPhase::NotReplacedCertificationOnly
        | ConflictBatchAdmissionReplacementPhase::BlockedOnQueryCapability => {
            residue_cluster(row.surface_identity()).map(|cluster| (cluster, row))
        }
        _ => None,
    }
}

fn phase_eleven_cluster(
    surface: ConflictBatchAdmissionSurfaceIdentity,
) -> Option<WorthWorkloadOrdinaryConsumerClusterKind> {
    match surface {
        ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadAdmitLookupConsumedWorkload
        | ConflictBatchAdmissionSurfaceIdentity::CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived)
        }
        ConflictBatchAdmissionSurfaceIdentity::BooleanSplitReplayUndoBoundaryAdmission
        | ConflictBatchAdmissionSurfaceIdentity::BooleanChainIntegrationHandoff => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay)
        }
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived)
        }
        _ => None,
    }
}

fn phase_thirteen_cluster(
    surface: ConflictBatchAdmissionSurfaceIdentity,
) -> Option<WorthWorkloadOrdinaryConsumerClusterKind> {
    match surface {
        ConflictBatchAdmissionSurfaceIdentity::TopologyQueryBackedConsumerFamilyRowSelectedCompatibilityBasisIdentityDigest => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked)
        }
        ConflictBatchAdmissionSurfaceIdentity::WorthTouchedGraphConflictProofChainTopologyQuerySelectedCompatibilityBasisIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::WorthTouchedGraphConflictMilestoneFourteenSeedTopologyQuerySelectedCompatibilityBasisIdentityDigest => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout)
        }
        ConflictBatchAdmissionSurfaceIdentity::LookupConsumedReuseResolutionSelectedCompatibilityBasisIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupIndexProductSelectedCompatibilityBasisIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupIndexProductSelectedCompatibilityPosture
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupIndexReuseDecisionSelectedCompatibilityBasisIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupIndexRebuildDenialSelectedCompatibilityBasisIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::SelectedSpatialEquivalenceFamilyCompatibilityBasisIdentity
        | ConflictBatchAdmissionSurfaceIdentity::SelectedSpatialEquivalenceFamilyCompatibilityPosture
        | ConflictBatchAdmissionSurfaceIdentity::SpatialSelectedCompatibilityBasisIdentityStruct
        | ConflictBatchAdmissionSurfaceIdentity::SpatialSelectedCompatibilityBasisIdentityIdentityDigest
        | ConflictBatchAdmissionSurfaceIdentity::SpatialSelectedEquivalenceFamilyDeclarationCompatibilityPosture
        | ConflictBatchAdmissionSurfaceIdentity::SpatialCompatibilityPostureEnum => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived)
        }
        _ => None,
    }
}

fn residue_cluster(
    surface: ConflictBatchAdmissionSurfaceIdentity,
) -> Option<WorthWorkloadOrdinaryConsumerClusterKind> {
    match surface {
        ConflictBatchAdmissionSurfaceIdentity::BooleanChainResidueRows
        | ConflictBatchAdmissionSurfaceIdentity::ReplayScopeProductBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::UndoScopeProductBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::ReplayUndoSpatialBoundaryFixtureWithTestBroadReceiptScanCount => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay)
        }
        ConflictBatchAdmissionSurfaceIdentity::TraversalViewsOldAuthorityResidue
        | ConflictBatchAdmissionSurfaceIdentity::TopoEdgeSplitOverlapChainPublicContract => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived)
        }
        ConflictBatchAdmissionSurfaceIdentity::TopologyQueryBackedConsumerCutoverWithTestLoopCycleSelectedCompatibilityBasisIdentityOverride => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked)
        }
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupDiagnosticsHiddenBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupInventoryBroadScanRowCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupSourceFirewallMentionsBroadReceiptScan
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupSourceFirewallBroadReceiptScanRowCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupWorkloadCutoverBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupStageCutoverBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupStageCutoverWithTestBroadReceiptScanCount
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupPlanSelectionBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupReuseDecisionBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupReuseExecutionInputBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupConsumedWorkloadHandoffWithTestBroadReceiptScanCount
        | ConflictBatchAdmissionSurfaceIdentity::SpatialTouchBroadLedgerScanCounter => {
            Some(WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived)
        }
        _ => None,
    }
}

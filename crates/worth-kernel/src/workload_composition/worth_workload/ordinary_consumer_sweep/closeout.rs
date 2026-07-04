use topology::facade::{
    current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerFamilyRow,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;

use super::cluster_ledger::{
    row_is_non_ordinary_residue, WorthWorkloadOrdinaryConsumerClusterLedger,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
use super::current_cutover::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutoverPosture,
    WorthWorkloadOrdinaryConsumerCutoverRow,
};
use super::current_ledgers::{
    build_current_cluster_ledgers, build_workload_composition_explainer_ledger,
};
use super::error::{
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
};
use super::residue::collect_residue_rows;
use super::workload_composition_explainer_ledger::WorthWorkloadCompositionExplainerLedger;
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictPublicFacade,
};
use crate::workload_composition::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventory, ConflictBatchAdmissionSurfaceIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerSweepCloseout {
    closeout_digest: String,
    cluster_ledgers: Vec<WorthWorkloadOrdinaryConsumerClusterLedger>,
    residue_rows: Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow>,
    workload_composition_explainer_ledger: WorthWorkloadCompositionExplainerLedger,
}

pub fn current_worth_workload_ordinary_consumer_sweep_closeout() -> Result<
    WorthWorkloadOrdinaryConsumerSweepCloseout,
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
> {
    let inventory = current_conflict_batch_admission_inventory().map_err(|error| {
        WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingInventory,
            format!("ordinary sweep closeout requires current conflict inventory: {error:?}"),
        )
    })?;
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
            format!("ordinary sweep closeout requires phase-11 ordinary cutover: {error:?}"),
        )
    })?;
    let topology_cutover = current_topology_query_backed_consumer_cutover().map_err(|error| {
        WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
            format!(
                "ordinary sweep closeout requires current topology query-backed cutover: {error:?}"
            ),
        )
    })?;
    let lookup_public_closeout = current_evidence_lookup_public_closeout().map_err(|error| {
        WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
            format!("ordinary sweep closeout requires current evidence lookup public closeout: {error:?}"),
        )
    })?;
    let public_facade = current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
        WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
    )
    .map_err(|error| {
        WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
            format!("ordinary sweep closeout requires planner-owned workload-composition public facade: {error:?}"),
        )
    })?;
    build_closeout_from_artifacts(
        &cutover,
        &topology_cutover,
        &lookup_public_closeout,
        &public_facade,
        &inventory,
    )
}

fn build_closeout_from_artifacts(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    topology_cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
    lookup_public_closeout: &worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseout,
    public_facade: &WorthTouchedGraphConflictPublicFacade,
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<
    WorthWorkloadOrdinaryConsumerSweepCloseout,
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
> {
    require_no_phase_eleven_bypass_rows(cutover.rows())?;
    require_typed_query_backed_rows(topology_cutover.family_rows())?;
    let mut residue_rows = collect_residue_rows(inventory)?;
    residue_rows.sort_by(|left, right| left.surface_name().cmp(right.surface_name()));
    let workload_composition_explainer_ledger =
        build_workload_composition_explainer_ledger(public_facade);
    let cluster_ledgers = build_current_cluster_ledgers(
        cutover,
        topology_cutover,
        lookup_public_closeout,
        public_facade,
        &residue_rows,
    );
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &cluster_ledgers
            .iter()
            .map(|ledger| {
                format!(
                    "{}:{}:{}:{}:{}:{}:{}",
                    ledger.cluster_kind().as_str(),
                    ledger.blocked_follow_on_family().as_str(),
                    ledger.migrated_count(),
                    ledger.deleted_count(),
                    ledger.capped_residue_count(),
                    ledger.query_gap_count(),
                    ledger.certification_only_count()
                )
            })
            .chain(
                cluster_ledgers
                    .iter()
                    .flat_map(|ledger| ledger.proof_basis_digests().iter())
                    .map(|digest| format!("proof:{digest}")),
            )
            .chain(residue_rows.iter().map(|row| {
                format!(
                    "residue:{}:{}:{}",
                    row.surface_name(),
                    row.owner(),
                    row.disposition().as_str()
                )
            }))
            .chain(
                workload_composition_explainer_ledger
                    .proof_basis_digests()
                    .iter()
                    .map(|digest| format!("workload-composition-explainer-proof:{digest}")),
            )
            .chain(
                workload_composition_explainer_ledger
                    .rows()
                    .iter()
                    .map(|row| {
                        format!(
                            "workload-composition-explainer:{}:{}:{}",
                            row.surface_name(),
                            row.owner(),
                            row.disposition().as_str()
                        )
                    }),
            )
            .chain(std::iter::once(
                "worth-kernel:ordinary-consumer-sweep-closeout:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );
    Ok(WorthWorkloadOrdinaryConsumerSweepCloseout {
        closeout_digest,
        cluster_ledgers,
        residue_rows,
        workload_composition_explainer_ledger,
    })
}

impl WorthWorkloadOrdinaryConsumerSweepCloseout {
    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn cluster_ledgers(&self) -> &[WorthWorkloadOrdinaryConsumerClusterLedger] {
        &self.cluster_ledgers
    }

    pub fn residue_rows(&self) -> &[WorthWorkloadOrdinaryConsumerSweepResidueRow] {
        &self.residue_rows
    }

    pub fn workload_composition_explainer_ledger(
        &self,
    ) -> &WorthWorkloadCompositionExplainerLedger {
        &self.workload_composition_explainer_ledger
    }

    pub fn require_all_covered_consumers_on_compiled_product_lane(
        &self,
    ) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
        for ledger in &self.cluster_ledgers {
            if ledger.migrated_count() > 0 && ledger.proof_basis_digests().is_empty() {
                return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::CoveredOrdinaryConsumerBypass,
                    format!(
                        "cluster `{}` still exposes migrated ordinary consumers without compiled-product lane proof",
                        ledger.cluster_kind().as_str()
                    ),
                ));
            }
        }
        Ok(())
    }

    pub fn require_zero_broad_scan_fallback_on_ordinary_path(
        &self,
    ) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
        let topology_cutover = current_topology_query_backed_consumer_cutover().map_err(|error| {
            WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingCurrentProofSurface,
                format!("ordinary sweep closeout requires current topology query-backed cutover: {error:?}"),
            )
        })?;
        if let Some(row) = topology_cutover
            .family_rows()
            .iter()
            .find(|row| row.row_scan_fallback_count() > 0 || row.whole_view_fallback_count() > 0)
        {
            return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary,
                format!(
                    "query-backed family `{:?}` still reports row/whole-view fallback on the ordinary path",
                    row.request_family()
                ),
            ));
        }
        let inventory = current_conflict_batch_admission_inventory().map_err(|error| {
            WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingInventory,
                format!("ordinary sweep broad-scan gate requires current inventory: {error:?}"),
            )
        })?;
        require_nonordinary_inventory_and_query_path_for_artifacts(&inventory, &topology_cutover)?;
        Ok(())
    }
}

pub(crate) fn require_no_phase_eleven_bypass_rows(
    rows: &[WorthWorkloadOrdinaryConsumerCutoverRow],
) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
    if let Some(row) = rows.iter().find(|row| {
        row.posture()
            == WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency
    }) {
        return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::CoveredOrdinaryConsumerBypass,
            format!(
                "phase-11 ordinary consumer `{}` remains a covered dependency instead of a compiled-product lane consumer",
                row.surface_name()
            ),
        ));
    }
    Ok(())
}

fn require_typed_query_backed_rows(
    rows: &[TopologyQueryBackedConsumerFamilyRow],
) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
    if let Some(row) = rows.iter().find(|row| {
        row.selected_equivalence_family_identity().is_none()
            || row.selected_equivalence_basis_identity_digest().is_none()
            || row.selected_compatibility_basis_identity_digest().is_none()
            || row.selected_reuse_basis_identity_digest().is_none()
            || (row.reuse_decision_identity_digest().is_none()
                && row.rebuild_denial_identity_digest().is_none())
    }) {
        return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::MissingTypedQueryBackedProof,
            format!(
                "query-backed family `{:?}` is missing typed equivalence/reuse proof on the ordinary path",
                row.request_family()
            ),
        ));
    }
    Ok(())
}

fn require_explicit_surface_denials_for_ordinary_path(
    inventory: &ConflictBatchAdmissionInventory,
) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
    for surface in expected_nonordinary_surface_identities() {
        let Some(row) = inventory
            .rows()
            .iter()
            .find(|row| row.surface_identity() == *surface)
        else {
            return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary,
                format!("broad-scan surface `{surface:?}` is missing from current inventory"),
            ));
        };
        if row.disposition() != ConflictBatchAdmissionDisposition::Cap
            || row.certification_posture()
                != ConflictBatchAdmissionCertificationPosture::NonOrdinaryResidueDeniedAsOrdinaryProof
            || row.cost_posture() != ConflictBatchAdmissionCostPosture::CappedResidue
            || !row_is_non_ordinary_residue(row)
        {
            return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
                WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary,
                format!(
                    "broad-scan surface `{surface:?}` is not explicitly denied as non-ordinary residue"
                ),
            ));
        }
    }
    Ok(())
}

fn require_nonordinary_inventory_and_query_path_for_artifacts(
    inventory: &ConflictBatchAdmissionInventory,
    topology_cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
) -> Result<(), WorthWorkloadOrdinaryConsumerSweepCloseoutError> {
    if let Some(row) = topology_cutover
        .family_rows()
        .iter()
        .find(|row| row.row_scan_fallback_count() > 0 || row.whole_view_fallback_count() > 0)
    {
        return Err(WorthWorkloadOrdinaryConsumerSweepCloseoutError::new(
            WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary,
            format!(
                "query-backed family `{:?}` still reports row/whole-view fallback on the ordinary path",
                row.request_family()
            ),
        ));
    }
    require_explicit_surface_denials_for_ordinary_path(inventory)
}

#[cfg(test)]
pub(crate) fn validate_assembled_ordinary_sweep_closeout_for_test(
    inventory: &ConflictBatchAdmissionInventory,
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    topology_cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
    lookup_public_closeout: &worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseout,
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Result<
    WorthWorkloadOrdinaryConsumerSweepCloseout,
    WorthWorkloadOrdinaryConsumerSweepCloseoutError,
> {
    let closeout = build_closeout_from_artifacts(
        cutover,
        topology_cutover,
        lookup_public_closeout,
        public_facade,
        inventory,
    )?;
    require_nonordinary_inventory_and_query_path_for_artifacts(inventory, topology_cutover)?;
    Ok(closeout)
}

fn expected_nonordinary_surface_identities() -> &'static [ConflictBatchAdmissionSurfaceIdentity] {
    &[
        ConflictBatchAdmissionSurfaceIdentity::ReplayScopeProductBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::UndoScopeProductBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupDiagnosticsHiddenBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupInventoryBroadScanRowCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupSourceFirewallMentionsBroadReceiptScan,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupSourceFirewallBroadReceiptScanRowCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupWorkloadCutoverBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupStageCutoverBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupPlanSelectionBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupReuseDecisionBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupReuseExecutionInputBroadReceiptScanCounter,
        ConflictBatchAdmissionSurfaceIdentity::SpatialTouchBroadLedgerScanCounter,
    ]
}

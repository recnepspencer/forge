use super::closeout::current_worth_workload_ordinary_consumer_sweep_closeout;
use super::closeout_test_support::{
    disposition_label, evidence_lookup_public_owner_label, inventory_owner_label as owner_label,
    public_closeout_owner_label, query_backed_owner_label, spatial_owner_label,
    topology_owner_label,
};
use super::cluster_ledger::{
    WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily, WorthWorkloadOrdinaryConsumerClusterKind,
    WorthWorkloadOrdinaryConsumerClusterLedger, WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
use super::current_cutover::current_worth_workload_ordinary_consumer_cutover;
use crate::workload_composition::public_closeout::{
    current_public_closeout_consumer_residue_manifest,
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_closeout,
};
use crate::workload_composition::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventory, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionSurfaceIdentity,
};
use std::collections::BTreeSet;
use topology::facade::{
    current_query_backed_consumer_residue_manifest, current_topology_consumer_residue_manifest,
    current_topology_query_backed_consumer_cutover, QueryBackedConsumerResidueDisposition,
    TopologyConsumerResidueDisposition,
};
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout,
    current_evidence_lookup_public_closeout_residue_manifest,
    EvidenceLookupPublicCloseoutDisposition, EvidenceLookupPublicCloseoutResidueDisposition,
};
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::{
    current_spatial_consumer_residue_manifest, SpatialConsumerResidueDisposition,
};
type LedgerRowKey = (String, String, String, String, String);
#[test]
fn current_closeout_matches_exact_live_cluster_ledgers() {
    let closeout = current_worth_workload_ordinary_consumer_sweep_closeout()
        .expect("current ordinary-consumer sweep closeout should build");
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");
    let topology_cutover =
        current_topology_query_backed_consumer_cutover().expect("query-backed cutover");
    let lookup_public_closeout =
        current_evidence_lookup_public_closeout().expect("lookup public closeout");
    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().expect("kernel public closeout");
    let phase_fifteen_seed =
        current_worth_touched_graph_conflict_milestone_fifteen_seed().expect("seed");
    let expected_residue_rows = expected_residue_rows(
        &current_conflict_batch_admission_inventory().expect("current inventory should build"),
    );
    assert_eq!(
        closeout
            .cluster_ledgers()
            .iter()
            .map(|ledger| ledger.cluster_kind())
            .collect::<Vec<_>>(),
        vec![
            WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
            WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
            WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
            WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
        ]
    );
    assert_cluster_ledger(
        ledger_for(
            &closeout,
            WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
        ),
        WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::TopologyDerivedMaterializationConsumers,
        BTreeSet::new(),
        expected_rows_for_cluster(
            &expected_residue_rows,
            WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
        ),
    );
    assert_cluster_ledger(
        ledger_for(
            &closeout,
            WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
        ),
        WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::EvidenceLookupIndexProductConsumers,
        expected_spatial_proof_digests(&cutover, lookup_public_closeout.closeout_digest()),
        expected_spatial_rows(&cutover, &expected_residue_rows),
    );
    assert_cluster_ledger(
        ledger_for(&closeout, WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked),
        WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::QueryBackedProjectionAndLowerRuntimeConsumers,
        BTreeSet::from([topology_cutover.closeout_digest().to_string()]),
        expected_query_backed_rows(&topology_cutover, &expected_residue_rows),
    );
    assert_cluster_ledger(
        ledger_for(
            &closeout,
            WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
        ),
        WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::RetainedReplayProductConsumers,
        expected_retained_replay_proof_digests(&cutover),
        expected_retained_replay_rows(&cutover, &expected_residue_rows),
    );
    assert_cluster_ledger(
        ledger_for(
            &closeout,
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
        ),
        WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::PublicCloseoutAndReadModelConsumers,
        BTreeSet::from([
            public_closeout.closeout_digest().to_string(),
            public_closeout
                .proof_chain()
                .proof_chain_digest()
                .to_string(),
            phase_fifteen_seed.seed_digest().to_string(),
        ]),
        expected_public_closeout_rows(
            &lookup_public_closeout,
            &public_closeout,
            &expected_residue_rows,
        ),
    );

    assert_exact_row_set(
        "ordinary sweep residue rows",
        &actual_row_set(closeout.residue_rows()),
        &expected_residue_rows
            .iter()
            .map(|(_, row)| row.clone())
            .collect::<BTreeSet<_>>(),
    );
}
fn assert_cluster_ledger(
    ledger: &WorthWorkloadOrdinaryConsumerClusterLedger,
    blocked_follow_on_family: WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily,
    expected_proof_digests: BTreeSet<String>,
    expected_rows: BTreeSet<LedgerRowKey>,
) {
    assert_eq!(ledger.blocked_follow_on_family(), blocked_follow_on_family);
    assert_eq!(
        ledger
            .proof_basis_digests()
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected_proof_digests
    );
    let label = format!("cluster ledger {}", ledger.cluster_kind().as_str());
    let actual_rows = actual_row_set(ledger.rows());
    assert_exact_row_set(&label, &actual_rows, &expected_rows);
}

fn assert_exact_row_set(
    label: &str,
    actual: &BTreeSet<LedgerRowKey>,
    expected: &BTreeSet<LedgerRowKey>,
) {
    let missing = expected.difference(actual).cloned().collect::<Vec<_>>();
    let unexpected = actual.difference(expected).cloned().collect::<Vec<_>>();
    assert!(
        missing.is_empty() && unexpected.is_empty(),
        "{label} mismatch; missing={missing:?}; unexpected={unexpected:?}"
    );
}
fn expected_spatial_rows(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    residue_rows: &[(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)],
) -> BTreeSet<LedgerRowKey> {
    let migrated = current_cutover_rows(
        cutover,
        [
            "WorthWorkload::admit_lookup_consumed_workload",
            "CompletedBooleanSplitHandoff::admit_downstream_split_consumption",
        ],
    );
    migrated
        .into_iter()
        .chain(expected_rows_for_cluster(
            residue_rows,
            WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
        ))
        .collect()
}
fn expected_retained_replay_rows(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    residue_rows: &[(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)],
) -> BTreeSet<LedgerRowKey> {
    let migrated = current_cutover_rows(cutover, ["admit_boolean_split_replay_undo_boundary"]);
    migrated
        .into_iter()
        .chain(expected_rows_for_cluster(
            residue_rows,
            WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
        ))
        .collect()
}
fn expected_query_backed_rows(
    topology_cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
    residue_rows: &[(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)],
) -> BTreeSet<LedgerRowKey> {
    topology_cutover
        .family_rows()
        .iter()
        .map(|row| query_backed_row_tuple(row.request_family()))
        .chain(expected_rows_for_cluster(
            residue_rows,
            WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
        ))
        .collect()
}
fn expected_public_closeout_rows(
    lookup_public_closeout: &worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseout,
    public_closeout: &crate::workload_composition::public_closeout::WorthTouchedGraphConflictPublicCloseout,
    residue_rows: &[(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)],
) -> BTreeSet<LedgerRowKey> {
    lookup_public_closeout
        .family_stage_rows()
        .iter()
        .filter(|row| matches!(row.disposition(), EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }))
        .map(evidence_lookup_public_row_tuple)
        .chain(std::iter::once((
            "current_worth_touched_graph_conflict_public_closeout".to_string(),
            "worth-kernel".to_string(),
            WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer.as_str().to_string(),
            "public closeout must keep consuming typed topology, evidence lookup, and replay/undo proof products instead of local cache folklore".to_string(),
            format!(
                "ordinary public/read-model consumers remain on the live public closeout proof chain digest {}",
                public_closeout.proof_chain().proof_chain_digest()
            ),
        )))
        .chain(expected_rows_for_cluster(residue_rows, WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout))
        .collect()
}

fn expected_spatial_proof_digests(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    lookup_public_closeout_digest: &str,
) -> BTreeSet<String> {
    route_digests_for_surfaces(
        cutover,
        [
            "WorthWorkload::admit_lookup_consumed_workload",
            "CompletedBooleanSplitHandoff::admit_downstream_split_consumption",
        ],
    )
    .into_iter()
    .chain([
        cutover
            .batch_execution_receipt()
            .execution_receipt_digest()
            .to_string(),
        lookup_public_closeout_digest.to_string(),
    ])
    .collect()
}

fn expected_retained_replay_proof_digests(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
) -> BTreeSet<String> {
    route_digests_for_surfaces(cutover, ["admit_boolean_split_replay_undo_boundary"])
        .into_iter()
        .chain(cutover.replay_undo_boundary_proof_digests())
        .collect()
}

fn current_cutover_rows<const N: usize>(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    surfaces: [&str; N],
) -> BTreeSet<LedgerRowKey> {
    cutover
        .rows()
        .iter()
        .filter(|row| surfaces.contains(&row.surface_name()))
        .map(|row| {
            (
                row.surface_name().to_string(),
                row.owner().to_string(),
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer
                    .as_str()
                    .to_string(),
                row.blocker().to_string(),
                row.removal_trigger().to_string(),
            )
        })
        .collect()
}

fn route_digests_for_surfaces<const N: usize>(
    cutover: &super::current_cutover::WorthWorkloadOrdinaryConsumerCutover,
    surfaces: [&str; N],
) -> Vec<String> {
    cutover
        .rows()
        .iter()
        .filter(|row| surfaces.contains(&row.surface_name()))
        .filter_map(|row| {
            row.selected_plan_witness()
                .map(|witness| witness.route_authority_digest().to_string())
        })
        .collect()
}

fn query_backed_row_tuple(family: TopologyReadRequestFamily) -> LedgerRowKey {
    (
        format!("TopologyQueryBackedConsumerFamilyRow::{family:?}"),
        "worth-topo".to_string(),
        WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer.as_str().to_string(),
        format!("query-backed ordinary family `{family:?}` must keep consuming typed compiled-product and equivalence proof instead of local support folklore"),
        format!("ordinary read-model consumers for `{family:?}` must keep lowering through the query-backed cutover row rather than reopening historical stability folklore"),
    )
}

fn evidence_lookup_public_row_tuple(
    row: &worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutFamilyStageRow,
) -> LedgerRowKey {
    (
        format!("EvidenceLookupPublicCloseoutFamilyStageRow::{}::{:?}", row.family_identity(), row.stage()),
        "worth-spatial".to_string(),
        WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer.as_str().to_string(),
        "evidence lookup public closeout must stay on receipt-backed typed lookup proof instead of local comparison or broad-scan fallback".to_string(),
        format!(
            "public lookup stage remains on selected lookup plan {} and query surface digest {}",
            row.selected_lookup_plan_digest().unwrap_or("missing-selected-lookup-plan"),
            row.query_surface_row_digest()
        ),
    )
}

fn actual_row_set(rows: &[WorthWorkloadOrdinaryConsumerSweepResidueRow]) -> BTreeSet<LedgerRowKey> {
    rows.iter()
        .map(|row| {
            (
                row.surface_name().to_string(),
                row.owner().to_string(),
                row.disposition().as_str().to_string(),
                row.blocker().to_string(),
                row.removal_trigger().to_string(),
            )
        })
        .collect()
}

fn expected_rows_for_cluster(
    rows: &[(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)],
    cluster: WorthWorkloadOrdinaryConsumerClusterKind,
) -> BTreeSet<LedgerRowKey> {
    rows.iter()
        .filter(|(kind, _)| *kind == cluster)
        .map(|(_, row)| row.clone())
        .collect()
}

fn expected_residue_rows(
    inventory: &ConflictBatchAdmissionInventory,
) -> Vec<(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)> {
    let mut rows = inventory
        .rows()
        .iter()
        .filter(|row| row.disposition() != ConflictBatchAdmissionDisposition::Migrate)
        .filter_map(expected_inventory_residue_row)
        .collect::<Vec<_>>();
    rows.extend(
        current_topology_consumer_residue_manifest()
            .iter()
            .map(|row| {
                (
                    WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
                    topology_residue_tuple(row),
                )
            }),
    );
    rows.extend(
        current_query_backed_consumer_residue_manifest()
            .iter()
            .map(|row| {
                (
                    WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
                    query_backed_residue_tuple(row),
                )
            }),
    );
    rows.extend(
        current_public_closeout_consumer_residue_manifest()
            .expect("public closeout residue")
            .iter()
            .map(|row| {
                (
                    WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
                    public_closeout_residue_tuple(row),
                )
            }),
    );
    rows.extend(
        current_evidence_lookup_public_closeout_residue_manifest()
            .iter()
            .map(|row| {
                (
                    WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
                    evidence_lookup_public_closeout_residue_tuple(row),
                )
            }),
    );
    rows.extend(
        current_spatial_consumer_residue_manifest()
            .iter()
            .map(|row| {
                (
                    WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
                    spatial_residue_tuple(row),
                )
            }),
    );
    rows
}

fn expected_inventory_residue_row(
    row: &ConflictBatchAdmissionInventoryRow,
) -> Option<(WorthWorkloadOrdinaryConsumerClusterKind, LedgerRowKey)> {
    let cluster = match row.surface_identity() {
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof => WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
        ConflictBatchAdmissionSurfaceIdentity::BooleanChainIntegrationHandoff
        | ConflictBatchAdmissionSurfaceIdentity::BooleanChainResidueRows
        | ConflictBatchAdmissionSurfaceIdentity::ReplayScopeProductBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::UndoScopeProductBroadReceiptScanCounter
        | ConflictBatchAdmissionSurfaceIdentity::ReplayUndoSpatialBoundaryFixtureWithTestBroadReceiptScanCount => WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
        ConflictBatchAdmissionSurfaceIdentity::TraversalViewsOldAuthorityResidue
        | ConflictBatchAdmissionSurfaceIdentity::TopoEdgeSplitOverlapChainPublicContract => WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
        ConflictBatchAdmissionSurfaceIdentity::TopologyQueryBackedConsumerCutoverWithTestLoopCycleSelectedCompatibilityBasisIdentityOverride => WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
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
        | ConflictBatchAdmissionSurfaceIdentity::SpatialTouchBroadLedgerScanCounter => WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
        _ => return None,
    };
    Some((
        cluster,
        (
            row.surface_name().to_string(),
            owner_label(row.owner()).to_string(),
            disposition_label(row.disposition()).to_string(),
            row.blocker().to_string(),
            row.removal_trigger().to_string(),
        ),
    ))
}

fn topology_residue_tuple(row: &topology::facade::TopologyConsumerResidueRow) -> LedgerRowKey {
    (
        row.current_surface().to_string(),
        topology_owner_label(row.owner()).to_string(),
        match row.disposition() {
            TopologyConsumerResidueDisposition::ExplicitResidue => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
            }
            TopologyConsumerResidueDisposition::QueryGap => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
            }
            TopologyConsumerResidueDisposition::AuthoritativeOrdinaryConsumer => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer
            }
        }
        .as_str()
        .to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}
fn query_backed_residue_tuple(
    row: &topology::facade::QueryBackedConsumerResidueRow,
) -> LedgerRowKey {
    (
        row.current_surface().to_string(),
        query_backed_owner_label(row.owner()).to_string(),
        match row.disposition() {
            QueryBackedConsumerResidueDisposition::ExplicitResidue => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
            }
            QueryBackedConsumerResidueDisposition::QueryGap => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
            }
        }
        .as_str()
        .to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}
fn public_closeout_residue_tuple(
    row: &crate::workload_composition::public_closeout::PublicCloseoutConsumerResidueRow,
) -> LedgerRowKey {
    (
        row.current_surface().to_string(),
        public_closeout_owner_label(row.owner()).to_string(),
        WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
            .as_str()
            .to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}
fn evidence_lookup_public_closeout_residue_tuple(
    row: &worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueRow,
) -> LedgerRowKey {
    (
        row.current_surface().to_string(),
        evidence_lookup_public_owner_label(row.owner()).to_string(),
        match row.disposition() {
            EvidenceLookupPublicCloseoutResidueDisposition::ExplicitResidue => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
            }
            EvidenceLookupPublicCloseoutResidueDisposition::QueryGap => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
            }
        }
        .as_str()
        .to_string(),
        row.blocker().to_string(),
        row.removal_trigger().to_string(),
    )
}
fn spatial_residue_tuple(
    row: &worth_spatial::facade::spatial_compiled_product_consumer_cutover::SpatialConsumerResidueRow,
) -> LedgerRowKey {
    (row.current_surface().to_string(), spatial_owner_label(row.owner()).to_string(), match row.disposition() { SpatialConsumerResidueDisposition::ExplicitResidue => WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue, SpatialConsumerResidueDisposition::CertificationOnly => WorthWorkloadOrdinaryConsumerClusterRowDisposition::CertificationOnlyDeniedAsOrdinaryProof, SpatialConsumerResidueDisposition::AuthoritativeOrdinaryConsumer => WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer }.as_str().to_string(), row.blocker().to_string(), row.removal_trigger().to_string())
}

fn ledger_for(
    closeout: &super::closeout::WorthWorkloadOrdinaryConsumerSweepCloseout,
    kind: WorthWorkloadOrdinaryConsumerClusterKind,
) -> &WorthWorkloadOrdinaryConsumerClusterLedger {
    closeout
        .cluster_ledgers()
        .iter()
        .find(|ledger| ledger.cluster_kind() == kind)
        .expect("cluster ledger should exist")
}

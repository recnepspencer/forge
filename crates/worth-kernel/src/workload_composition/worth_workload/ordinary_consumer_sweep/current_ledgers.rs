use topology::facade::{TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerFamilyRow};
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_public_closeout::{
    EvidenceLookupPublicCloseout, EvidenceLookupPublicCloseoutDisposition,
    EvidenceLookupPublicCloseoutFamilyStageRow,
};

use super::cluster_ledger::{
    WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily, WorthWorkloadOrdinaryConsumerClusterKind,
    WorthWorkloadOrdinaryConsumerClusterLedger, WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
use super::current_cutover::{
    WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverPosture,
};
use super::workload_composition_explainer_ledger::WorthWorkloadCompositionExplainerLedger;
use crate::workload_composition::WorthTouchedGraphConflictPublicFacade;

pub(super) fn build_current_cluster_ledgers(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    topology_cutover: &TopologyQueryBackedConsumerCutover,
    lookup_public_closeout: &EvidenceLookupPublicCloseout,
    public_facade: &WorthTouchedGraphConflictPublicFacade,
    residue_rows: &[WorthWorkloadOrdinaryConsumerSweepResidueRow],
) -> Vec<WorthWorkloadOrdinaryConsumerClusterLedger> {
    vec![
        WorthWorkloadOrdinaryConsumerClusterLedger::new(
            WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
            blocked_follow_on_family(WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived),
            Vec::new(),
            cluster_rows(
                WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
                Vec::new(),
                residue_rows,
            ),
        ),
        WorthWorkloadOrdinaryConsumerClusterLedger::new(
            WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
            blocked_follow_on_family(WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived),
            spatial_proof_basis_digests(cutover, lookup_public_closeout.closeout_digest()),
            cluster_rows(
                WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
                current_spatial_rows(cutover),
                residue_rows,
            ),
        ),
        WorthWorkloadOrdinaryConsumerClusterLedger::new(
            WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
            blocked_follow_on_family(WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked),
            vec![topology_cutover.closeout_digest().to_string()],
            cluster_rows(
                WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
                current_query_backed_rows(topology_cutover),
                residue_rows,
            ),
        ),
        WorthWorkloadOrdinaryConsumerClusterLedger::new(
            WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
            blocked_follow_on_family(WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay),
            retained_replay_proof_basis_digests(cutover),
            cluster_rows(
                WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
                current_retained_replay_rows(cutover),
                residue_rows,
            ),
        ),
        WorthWorkloadOrdinaryConsumerClusterLedger::new(
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
            blocked_follow_on_family(WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout),
            public_closeout_proof_basis_digests(public_facade),
            cluster_rows(
                WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
                current_public_closeout_rows(lookup_public_closeout, public_facade.public_proof()),
                residue_rows,
            ),
        ),
    ]
}

pub(super) fn build_workload_composition_explainer_ledger(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> WorthWorkloadCompositionExplainerLedger {
    WorthWorkloadCompositionExplainerLedger::current_from_public_facade(public_facade)
}

fn cluster_rows(
    cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
    mut current_rows: Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow>,
    residue_rows: &[WorthWorkloadOrdinaryConsumerSweepResidueRow],
) -> Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow> {
    current_rows.extend(
        residue_rows
            .iter()
            .filter(|row| row.cluster_kind() == cluster_kind)
            .cloned(),
    );
    current_rows
}

fn blocked_follow_on_family(
    cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
) -> WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily {
    match cluster_kind {
        WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived => {
            WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::TopologyDerivedMaterializationConsumers
        }
        WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived => {
            WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::EvidenceLookupIndexProductConsumers
        }
        WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked => {
            WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::QueryBackedProjectionAndLowerRuntimeConsumers
        }
        WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay => {
            WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::RetainedReplayProductConsumers
        }
        WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout => {
            WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily::PublicCloseoutAndReadModelConsumers
        }
    }
}

fn current_spatial_rows(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow> {
    cutover
        .rows()
        .iter()
        .filter(|row| {
            row.posture() == WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
                && matches!(
                    row.surface_name(),
                    "WorthWorkload::admit_lookup_consumed_workload"
                        | "CompletedBooleanSplitHandoff::admit_downstream_split_consumption"
                )
        })
        .map(|row| {
            WorthWorkloadOrdinaryConsumerSweepResidueRow::ordinary_migrated(
                WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
                "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_cutover.rs",
                row.surface_name(),
                row.owner(),
                row.blocker(),
                row.removal_trigger(),
            )
        })
        .collect()
}

fn current_retained_replay_rows(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow> {
    cutover
        .rows()
        .iter()
        .filter(|row| {
            row.posture() == WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
                && row.surface_name() == "admit_boolean_split_replay_undo_boundary"
        })
        .map(|row| {
            WorthWorkloadOrdinaryConsumerSweepResidueRow::ordinary_migrated(
                WorthWorkloadOrdinaryConsumerClusterKind::RetainedReplay,
                "crates/worth-kernel/src/workload_composition/worth_workload/ordinary_consumer_sweep/current_cutover.rs",
                row.surface_name(),
                row.owner(),
                row.blocker(),
                row.removal_trigger(),
            )
        })
        .collect()
}

fn current_query_backed_rows(
    topology_cutover: &TopologyQueryBackedConsumerCutover,
) -> Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow> {
    topology_cutover
        .family_rows()
        .iter()
        .map(query_backed_family_row_to_current_row)
        .collect()
}

fn query_backed_family_row_to_current_row(
    row: &TopologyQueryBackedConsumerFamilyRow,
) -> WorthWorkloadOrdinaryConsumerSweepResidueRow {
    WorthWorkloadOrdinaryConsumerSweepResidueRow::with_disposition(
        WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
        "crates/worth-topo/src/projection/query_backed_consumer_cutover/closeout.rs",
        format!(
            "TopologyQueryBackedConsumerFamilyRow::{:?}",
            row.request_family()
        ),
        "worth-topo",
        WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer,
        format!(
            "query-backed ordinary family `{:?}` must keep consuming typed compiled-product and equivalence proof instead of local support folklore",
            row.request_family()
        ),
        query_backed_removal_trigger(row.request_family()),
    )
}

fn current_public_closeout_rows(
    lookup_public_closeout: &EvidenceLookupPublicCloseout,
    public_proof: &crate::workload_composition::WorthTouchedGraphConflictPublicProofInspection,
) -> Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow> {
    let mut rows = lookup_public_closeout
        .family_stage_rows()
        .iter()
        .filter(|row| {
            matches!(
                row.disposition(),
                EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. }
            )
        })
        .map(evidence_lookup_family_row_to_current_row)
        .collect::<Vec<_>>();
    rows.push(
        WorthWorkloadOrdinaryConsumerSweepResidueRow::ordinary_migrated(
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
            "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_proof/current.rs",
            "current_worth_touched_graph_conflict_public_closeout",
            "worth-kernel",
            "public closeout must keep consuming typed topology, evidence lookup, and replay/undo proof products instead of local cache folklore",
            format!(
                "ordinary public/read-model consumers remain on the live public closeout proof chain digest {}",
                public_proof.proof_chain_digest()
            ),
        ),
    );
    rows
}

fn evidence_lookup_family_row_to_current_row(
    row: &EvidenceLookupPublicCloseoutFamilyStageRow,
) -> WorthWorkloadOrdinaryConsumerSweepResidueRow {
    WorthWorkloadOrdinaryConsumerSweepResidueRow::ordinary_migrated(
        WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
        "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/closeout_artifacts.rs",
        format!(
            "EvidenceLookupPublicCloseoutFamilyStageRow::{}::{:?}",
            row.family_identity(),
            row.stage()
        ),
        "worth-spatial",
        "evidence lookup public closeout must stay on receipt-backed typed lookup proof instead of local comparison or broad-scan fallback",
        format!(
            "public lookup stage remains on selected lookup plan {} and query surface digest {}",
            row.selected_lookup_plan_digest().unwrap_or("missing-selected-lookup-plan"),
            row.query_surface_row_digest()
        ),
    )
}

fn query_backed_removal_trigger(family: TopologyReadRequestFamily) -> String {
    format!(
        "ordinary read-model consumers for `{:?}` must keep lowering through the query-backed cutover row rather than reopening historical stability folklore",
        family
    )
}

fn spatial_proof_basis_digests(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    lookup_public_closeout_digest: &str,
) -> Vec<String> {
    let mut digests = cutover
        .rows()
        .iter()
        .filter(|row| {
            matches!(
                row.surface_name(),
                "WorthWorkload::admit_lookup_consumed_workload"
                    | "CompletedBooleanSplitHandoff::admit_downstream_split_consumption"
            )
        })
        .filter_map(|row| {
            row.selected_plan_witness()
                .map(|witness| witness.route_authority_digest().to_string())
        })
        .collect::<Vec<_>>();
    digests.push(
        cutover
            .batch_execution_receipt()
            .execution_receipt_digest()
            .to_string(),
    );
    digests.push(lookup_public_closeout_digest.to_string());
    digests
}

fn retained_replay_proof_basis_digests(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Vec<String> {
    let mut digests = cutover
        .rows()
        .iter()
        .filter(|row| row.surface_name() == "admit_boolean_split_replay_undo_boundary")
        .filter_map(|row| {
            row.selected_plan_witness()
                .map(|witness| witness.route_authority_digest().to_string())
        })
        .collect::<Vec<_>>();
    digests.extend(cutover.replay_undo_boundary_proof_digests());
    digests
}

fn public_closeout_proof_basis_digests(
    public_facade: &WorthTouchedGraphConflictPublicFacade,
) -> Vec<String> {
    let public_proof = public_facade.public_proof();
    vec![
        public_proof.closeout_digest().to_string(),
        public_proof.proof_chain_digest().to_string(),
        public_proof
            .milestone_fifteen_seed()
            .seed_digest()
            .to_string(),
    ]
}

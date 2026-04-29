use super::support::{default_query_mutation_evidence, seed_sheet_disk_topology};
use super::*;

#[test]
fn query_native_derived_chain_materializes_interpretation_and_validation() {
    let mut workspace = worth_topology_query_workspace("worth-query-derived-chain")
        .expect("query workspace should build");
    let entities =
        declare_worth_topology_entity_live_view::<Value>(&mut workspace, "worth.topology.entities")
            .expect("entity live view should declare");
    let relations = declare_worth_topology_relation_live_view::<Value>(
        &mut workspace,
        "worth.topology.relations",
    )
    .expect("relation live view should declare");
    let materialized = declare_worth_topology_materialized_surface::<Value, _, _>(
        &mut workspace,
        MATERIALIZED_TOPOLOGY_SURFACE,
        &entities,
        &relations,
    )
    .expect("materialized topology computed surface should declare");
    let interpreted = declare_worth_topology_interpreted_surface::<Value, _>(
        &mut workspace,
        INTERPRETED_TOPOLOGY_SURFACE,
        &materialized,
    )
    .expect("interpreted topology computed surface should declare");
    let validation = declare_worth_topology_validation_surface::<Value, _, _>(
        &mut workspace,
        VALIDATION_TOPOLOGY_SURFACE,
        &materialized,
        &interpreted,
    )
    .expect("validation topology computed surface should declare");

    let last_receipt = seed_sheet_disk_topology(&mut workspace);
    let materialized_rows = workspace.materialize(&materialized);
    let interpreted_rows = workspace.materialize(&interpreted);
    let validation_rows = workspace.materialize(&validation);

    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&MATERIALIZED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&INTERPRETED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&VALIDATION_TOPOLOGY_SURFACE.to_string()));
    assert_eq!(last_receipt.considered_computed_view_count(), 3);

    let interpreted_view: InterpretedTopologyView =
        serde_json::from_value(interpreted_rows[0].clone()).expect("interpreted topology row");
    let validation_report: DerivedTopologyValidationReport =
        serde_json::from_value(validation_rows[0].clone()).expect("validation topology row");

    assert_eq!(materialized_rows.len(), 1);
    assert_eq!(interpreted_view.report().interpreted_wire_count, 1);
    assert_eq!(interpreted_view.report().interpreted_shell_count, 1);
    assert!(validation_report
        .rows
        .iter()
        .any(|row| row.validator == "ownership"));
    assert!(validation_report
        .rows
        .iter()
        .any(|row| row.validator == "radial"));
}

#[test]
fn query_native_derived_chain_exposes_query_state_and_inspection() {
    let mut workspace = worth_topology_query_workspace("worth-query-derived-inspection")
        .expect("query workspace should build");
    let assembly =
        WorthTopologyQueryAssembly::declare(&mut workspace).expect("query assembly should declare");

    let receipt = seed_sheet_disk_topology(&mut workspace);
    let validation_state = workspace
        .state(assembly.validation())
        .expect("validation state should reflect retained derived posture");
    let equivalence_state = workspace
        .state(assembly.equivalence_contract())
        .expect("equivalence state should reflect retained derived posture");
    let validation_inspection = workspace
        .inspect(assembly.validation())
        .expect("validation surface should inspect");
    let equivalence_inspection = workspace
        .inspect(assembly.equivalence_contract())
        .expect("equivalence surface should inspect");
    let receipt_inspection = workspace
        .inspect(&receipt)
        .expect("topology write receipt should inspect");

    assert_eq!(validation_state.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(
        validation_state.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert!(validation_state
        .explanation()
        .contains("retained materialization"));
    assert_eq!(equivalence_state.kind(), ForgeQueryRuntimeStateKind::Ready);

    match validation_inspection {
        ForgeQueryInspection::DerivedView(inspection) => {
            assert_eq!(inspection.name(), VALIDATION_TOPOLOGY_SURFACE);
            assert_eq!(
                inspection.upstream_derived_views(),
                &[
                    MATERIALIZED_TOPOLOGY_SURFACE.to_string(),
                    INTERPRETED_TOPOLOGY_SURFACE.to_string(),
                ]
            );
            assert!(inspection.upstream_live_views().is_empty());
            assert_eq!(
                inspection.dependency_aspects(),
                &[
                    "topology.structure".to_string(),
                    "topology.ownership".to_string(),
                    "topology.boundary".to_string(),
                    "topology.radial".to_string(),
                    "diagnostics.interpretations".to_string(),
                ]
            );
            assert_eq!(
                inspection.produced_aspects(),
                &["diagnostics.decisions".to_string()]
            );
            assert!(!inspection.incremental_delivery());
            assert!(inspection.materialized_row_count() > 0);
            assert!(inspection.pending_patch_count() > 0);
            assert_eq!(
                inspection.pending_refresh_fallback_count(),
                inspection.pending_patch_count()
            );
        }
        other => panic!("expected derived inspection, got {other:?}"),
    }

    match equivalence_inspection {
        ForgeQueryInspection::DerivedView(inspection) => {
            assert_eq!(inspection.name(), EQUIVALENCE_TOPOLOGY_SURFACE);
            assert_eq!(
                inspection.upstream_derived_views(),
                &[DIAGNOSTICS_TOPOLOGY_SURFACE.to_string()]
            );
            assert_eq!(
                inspection.dependency_aspects(),
                &[
                    "diagnostics.interpretations".to_string(),
                    "diagnostics.decisions".to_string(),
                ]
            );
            assert!(inspection.pending_refresh_fallback_count() >= 1);
        }
        other => panic!("expected derived inspection, got {other:?}"),
    }

    match receipt_inspection {
        ForgeQueryInspection::WriteReceipt(inspection) => {
            assert_eq!(inspection.commit_identity(), receipt.commit_identity());
            assert_eq!(
                inspection.declared_collection(),
                Some("WorthTopologyRelation")
            );
            assert!(!inspection.live_patch_artifacts().is_empty());
            assert!(!inspection.runtime_evidence().evidence().is_empty());
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get(WorthTopologyQueryMutationEvidence::metadata_key())
                    .expect("worth topology mutation metadata should retain"),
                &serde_json::to_value(default_query_mutation_evidence([
                    "topology.boundary".to_string()
                ]))
                .expect("query mutation evidence should serialize")
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }

    assert_eq!(
        receipt.affected_live_view_ids(),
        &["worth.topology.relations".to_string()]
    );
    assert_eq!(
        receipt.affected_derived_view_ids(),
        &[
            DIAGNOSTICS_TOPOLOGY_SURFACE.to_string(),
            EQUIVALENCE_TOPOLOGY_SURFACE.to_string(),
            INTERPRETED_TOPOLOGY_SURFACE.to_string(),
            MATERIALIZED_TOPOLOGY_SURFACE.to_string(),
            VALIDATION_TOPOLOGY_SURFACE.to_string(),
        ]
    );

    let diagnostics_rows = workspace.materialize(assembly.diagnostics());
    let equivalence_rows = workspace.materialize(assembly.equivalence_contract());
    let diagnostics_report: WorthDerivedReadDiagnostics =
        serde_json::from_value(diagnostics_rows[0].clone()).expect("diagnostics topology row");
    let equivalence_report = equivalence_contract_from_diagnostics_rows(&diagnostics_rows)
        .expect("equivalence contract should decode from diagnostics surface");

    assert!(
        diagnostics_report
            .invalidation_report
            .triggered_target_count
            > 0
    );
    assert_eq!(
        serde_json::from_value::<crate::facade::WorthDerivedEquivalenceContractReport>(
            equivalence_rows[0].clone()
        )
        .expect("equivalence row should decode"),
        equivalence_report
    );
}

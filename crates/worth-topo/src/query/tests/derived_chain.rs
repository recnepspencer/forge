use super::domain_query::support::{default_query_mutation_evidence, seeded_sheet_disk_workspace};
use super::*;

#[test]
fn query_native_derived_chain_materializes_interpretation_and_validation() {
    let (mut workspace, assembly, _) = seeded_sheet_disk_workspace("query-derived-chain");

    let last_receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .metadata(
                    TopologyQueryMutationEvidence::metadata_key(),
                    default_query_mutation_evidence([
                        "topology.structure".to_string(),
                        "naming.persistent_name".to_string(),
                    ]),
                )
                .aspect("topology.kind", TopologyEntityKind::Vertex.kind_name())
                .aspect("topology.structure", "query-derived-chain.extra-vertex")
                .aspect("naming.persistent_name", "query-derived-chain.extra-vertex")
        })
        .expect("entity insert should succeed");
    let materialized_rows = workspace.materialize(assembly.materialized());
    let interpreted_rows = workspace.materialize(assembly.interpreted());
    let validation_rows = workspace.materialize(assembly.validation());

    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&MATERIALIZED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&INTERPRETED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .affected_derived_view_ids()
        .contains(&VALIDATION_TOPOLOGY_SURFACE.to_string()));
    assert_eq!(last_receipt.considered_computed_view_count(), 5);

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
    let (mut workspace, assembly, _) = seeded_sheet_disk_workspace("query-derived-inspection");
    let receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .metadata(
                    TopologyQueryMutationEvidence::metadata_key(),
                    default_query_mutation_evidence([
                        "topology.structure".to_string(),
                        "naming.persistent_name".to_string(),
                    ]),
                )
                .aspect("topology.kind", TopologyEntityKind::Vertex.kind_name())
                .aspect(
                    "topology.structure",
                    "query-derived-inspection.extra-vertex",
                )
                .aspect(
                    "naming.persistent_name",
                    "query-derived-inspection.extra-vertex",
                )
        })
        .expect("entity insert should succeed");
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
            assert_eq!(inspection.declared_collection(), Some("TopologyEntity"));
            assert!(!inspection.live_patch_artifacts().is_empty());
            assert!(!inspection.runtime_evidence().evidence().is_empty());
            assert_eq!(
                inspection
                    .mutation_metadata()
                    .get(TopologyQueryMutationEvidence::metadata_key())
                    .expect(" topology mutation metadata should retain"),
                &serde_json::to_value(default_query_mutation_evidence([
                    "topology.structure".to_string(),
                    "naming.persistent_name".to_string(),
                ]))
                .expect("query mutation evidence should serialize")
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }

    assert_eq!(
        receipt.affected_live_view_ids(),
        &[
            ".naming.persistent_names".to_string(),
            ".topology.entities".to_string(),
        ]
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
    let diagnostics_report: DerivedReadDiagnostics =
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
        serde_json::from_value::<crate::facade::DerivedEquivalenceContractReport>(
            equivalence_rows[0].clone()
        )
        .expect("equivalence row should decode"),
        equivalence_report
    );
}

use super::topology_reads::support::{
    default_query_mutation_evidence, seeded_sheet_disk_workspace,
};
use super::*;
use crate::projection::runtime_boundary::declared_query_surfaces::materialize_declared_query_surface_row;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;

#[test]
fn query_native_derived_chain_materializes_interpretation_and_validation() {
    let (mut workspace, surfaces, _) = seeded_sheet_disk_workspace("query-derived-chain");

    let last_receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .metadata(
                    TopologyQueryMutationEvidence::metadata_key(),
                    serde_json::to_string(&default_query_mutation_evidence([
                        "topology.structure".to_string(),
                        "naming.persistent_name".to_string(),
                    ]))
                    .expect("query mutation evidence should serialize"),
                )
                .pipe(|builder| {
                    TopologyNativeQueryRowField::NamingPersistentName.set_on(
                        TopologyNativeQueryRowField::TopologyStructure.set_on(
                            TopologyNativeQueryRowField::TopologyKind
                                .set_on(builder, TopologyEntityKind::Vertex.kind_name()),
                            "query-derived-chain.extra-vertex",
                        ),
                        "query-derived-chain.extra-vertex",
                    )
                })
        })
        .expect("entity insert should succeed");
    let materialized: MaterializedTopologyView =
        materialize_declared_query_surface_row(&mut workspace, surfaces.materialized())
            .expect("materialized topology row");
    let interpreted_view: InterpretedTopologyView =
        materialize_declared_query_surface_row(&mut workspace, surfaces.interpreted())
            .expect("interpreted topology row");
    let validation_report: DerivedTopologyValidationReport =
        materialize_declared_query_surface_row(&mut workspace, surfaces.validation())
            .expect("validation topology row");

    assert!(last_receipt
        .terminal_affected_derived_view_ids_projection()
        .contains(&MATERIALIZED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .terminal_affected_derived_view_ids_projection()
        .contains(&INTERPRETED_TOPOLOGY_SURFACE.to_string()));
    assert!(last_receipt
        .terminal_affected_derived_view_ids_projection()
        .contains(&VALIDATION_TOPOLOGY_SURFACE.to_string()));
    assert_eq!(last_receipt.considered_computed_view_count(), 5);

    assert!(materialized.report().breadth.topology_entity_count > 0);
    assert_eq!(interpreted_view.report().interpreted_wire_count, 1);
    assert_eq!(interpreted_view.report().interpreted_shell_count, 1);
    assert!(validation_report
        .rows
        .iter()
        .any(|row| row.validator == "ownership"));
    assert!(validation_report
        .rows
        .iter()
        .any(|row| row.validator == "radial_rings"));
}

#[test]
fn query_native_derived_chain_exposes_query_state_and_inspection() {
    let (mut workspace, surfaces, _) = seeded_sheet_disk_workspace("query-derived-inspection");
    let receipt = workspace
        .insert("TopologyEntity", |builder| {
            builder
                .metadata(
                    TopologyQueryMutationEvidence::metadata_key(),
                    serde_json::to_string(&default_query_mutation_evidence([
                        "topology.structure".to_string(),
                        "naming.persistent_name".to_string(),
                    ]))
                    .expect("query mutation evidence should serialize"),
                )
                .pipe(|builder| {
                    TopologyNativeQueryRowField::NamingPersistentName.set_on(
                        TopologyNativeQueryRowField::TopologyStructure.set_on(
                            TopologyNativeQueryRowField::TopologyKind
                                .set_on(builder, TopologyEntityKind::Vertex.kind_name()),
                            "query-derived-inspection.extra-vertex",
                        ),
                        "query-derived-inspection.extra-vertex",
                    )
                })
        })
        .expect("entity insert should succeed");
    let validation_state = workspace
        .state(surfaces.validation())
        .expect("validation state should reflect retained derived posture");
    let equivalence_state = workspace
        .state(surfaces.equivalence_contract())
        .expect("equivalence state should reflect retained derived posture");
    let validation_inspection = workspace
        .inspect(surfaces.validation())
        .expect("validation surface should inspect");
    let equivalence_inspection = workspace
        .inspect(surfaces.equivalence_contract())
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
            assert_eq!(inspection.dependency_aspect_touches().len(), 5);
            assert_eq!(inspection.produced_aspect_touches().len(), 1);
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
            assert_eq!(inspection.dependency_aspect_touches().len(), 2);
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
                    .get(
                        &forge_query::facade::ForgeQueryMutationMetadataKey::new(
                            TopologyQueryMutationEvidence::metadata_key(),
                        )
                        .expect("metadata key should admit"),
                    )
                    .expect(" topology mutation metadata should retain")
                    .terminal_digest_text(),
                serde_json::to_string(&default_query_mutation_evidence([
                    "topology.structure".to_string(),
                    "naming.persistent_name".to_string(),
                ]))
                .expect("query mutation evidence should serialize")
                .as_str()
            );
        }
        other => panic!("expected write receipt inspection, got {other:?}"),
    }

    assert_eq!(
        receipt.terminal_affected_live_view_ids_projection(),
        vec![
            ".naming.persistent_names".to_string(),
            ".topology.entities".to_string()
        ]
    );
    assert_eq!(
        receipt.terminal_affected_derived_view_ids_projection(),
        vec![
            DIAGNOSTICS_TOPOLOGY_SURFACE.to_string(),
            EQUIVALENCE_TOPOLOGY_SURFACE.to_string(),
            INTERPRETED_TOPOLOGY_SURFACE.to_string(),
            MATERIALIZED_TOPOLOGY_SURFACE.to_string(),
            VALIDATION_TOPOLOGY_SURFACE.to_string(),
        ]
    );

    let diagnostics_report: DerivedReadDiagnostics =
        materialize_declared_query_surface_row(&mut workspace, surfaces.diagnostics())
            .expect("diagnostics topology row");
    let equivalence_report: crate::certification::DerivedEquivalenceContractReport =
        materialize_declared_query_surface_row(&mut workspace, surfaces.equivalence_contract())
            .expect("equivalence row should decode");

    assert!(
        diagnostics_report
            .invalidation_report
            .triggered_target_count
            > 0
    );
    assert_eq!(
        diagnostics_report
            .validation_execution_report
            .execution_count,
        1
    );
    assert_eq!(
        diagnostics_report
            .validation_execution_report
            .registered_rule_count,
        diagnostics_report.validation_report.rows.len()
    );
    assert_eq!(
        equivalence_report,
        diagnostics_report.equivalence_contract_report
    );
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

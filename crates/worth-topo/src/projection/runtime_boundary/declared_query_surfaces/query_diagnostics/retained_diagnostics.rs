use forge_query::facade::{
    ForgeQueryDerivedView, ForgeQueryRetainedRefreshContext, ForgeQueryRetainedUpstreamInputs,
};
#[cfg(test)]
use serde_json::Value;

use crate::compiled_product_family::{
    current_topology_compiled_product_family_catalog, select_topology_compiled_product_family,
    TopologyCompiledProductConsumer,
};
use crate::derived_invalidation_compiled_product_admission::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionRequest,
};
use crate::derived_topology::compiled_product_consumer_cutover::build_derived_equivalence_contract_report;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::DerivedReadDiagnostics;
use crate::projection::planner_owned_routing::diagnostic_projection_input::{
    build_derived_fallback_report_from_counts, build_derived_invalidation_report_from_aspects,
    build_derived_rebuild_report, derived_validation_execution_report,
    topology_derived_diagnostic_projection_source,
};
use crate::selected_equivalence_family::{
    current_topology_selected_equivalence_family_catalog, select_topology_equivalence_family,
};
use crate::validation::{DerivedTopologyValidationReport, RegisteredTopologyValidationReport};

#[cfg(test)]
use super::super::derived_surfaces::decode_query_surface_row;
use super::super::retained_payload::decode_single_retained_payload_row;
use super::super::{TopologyQuerySurfaceError, TopologyQuerySurfaceErrorKind};
use super::{TopologyHistoricalReadBasisMetadata, TopologyQueryMutationEvidence};

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn derived_read_diagnostics_from_query_rows(
    refresh: &ForgeQueryRetainedRefreshContext,
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
    validation_rows: &[Value],
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let read_basis_metadata = TopologyHistoricalReadBasisMetadata::from_refresh(refresh)?;
    let evidence = TopologyQueryMutationEvidence::from_read_basis(read_basis_metadata.read_basis());
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_query_surface_row(interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_query_surface_row(validation_rows, "topology validation")?;

    build_diagnostics_from_decoded_surfaces(
        read_basis_metadata,
        evidence,
        materialized,
        interpreted,
        validation,
    )
}

pub(super) fn derived_read_diagnostics_from_upstreams(
    refresh: &ForgeQueryRetainedRefreshContext,
    view: &ForgeQueryDerivedView,
    upstreams: &ForgeQueryRetainedUpstreamInputs,
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let read_basis_metadata = TopologyHistoricalReadBasisMetadata::from_refresh(refresh)?;
    let evidence = TopologyQueryMutationEvidence::from_read_basis(read_basis_metadata.read_basis());
    let mut upstream_rows = upstreams.declared_retained_computed_row_sets(view);
    let materialized: MaterializedTopologyView = decode_single_retained_payload_row(
        upstream_rows.next().unwrap_or_default(),
        "materialized topology",
    )?;
    let interpreted: InterpretedTopologyView = decode_single_retained_payload_row(
        upstream_rows.next().unwrap_or_default(),
        "interpreted topology",
    )?;
    let validation: DerivedTopologyValidationReport = decode_single_retained_payload_row(
        upstream_rows.next().unwrap_or_default(),
        "topology validation",
    )?;

    build_diagnostics_from_decoded_surfaces(
        read_basis_metadata,
        evidence,
        materialized,
        interpreted,
        validation,
    )
}

fn build_diagnostics_from_decoded_surfaces(
    read_basis_metadata: TopologyHistoricalReadBasisMetadata,
    evidence: TopologyQueryMutationEvidence,
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let touched_aspects = evidence.touched_aspects()?;
    let registered_validation = registered_validation_report(validation)?;
    let validation = registered_validation.report();
    let catalog = current_topology_compiled_product_family_catalog();
    let admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            read_basis_metadata.read_basis(),
        ),
    )
    .map_err(|error| {
        TopologyQuerySurfaceError::with_kind(
            TopologyQuerySurfaceErrorKind::CompiledProductAdmissionDenied,
            format!("query-derived diagnostics failed to admit topology compiled-product input: {error}"),
        )
    })?;
    let selected = select_topology_compiled_product_family(
        &catalog,
        admitted.clone().into_family_admitted_input(),
    )
    .map_err(|error| {
        TopologyQuerySurfaceError::with_kind(
            TopologyQuerySurfaceErrorKind::CompiledProductFamilySelectionFailed,
            format!("query-derived diagnostics failed to select topology compiled-product family: {error:?}"),
        )
    })?;
    let lowered = selected
        .compile_product_identity(&materialized, &interpreted, validation)
        .map_err(|error| {
            TopologyQuerySurfaceError::with_kind(
                TopologyQuerySurfaceErrorKind::CompiledProductIdentityLoweringFailed,
                format!(
                    "query-derived diagnostics failed to lower topology compiled-product identity: {error:?}"
                ),
            )
        })?;
    let selected_equivalence_family = select_topology_equivalence_family(
        &current_topology_selected_equivalence_family_catalog(),
        &admitted,
    )
    .map_err(|error| {
        TopologyQuerySurfaceError::with_kind(
            TopologyQuerySurfaceErrorKind::CompiledProductFamilySelectionFailed,
            format!(
                "query-derived diagnostics failed to select topology equivalence family: {error:?}"
            ),
        )
    })?;
    let equivalence_contract_report = build_derived_equivalence_contract_report(
        admitted.source_authority_basis().authority_snapshot_id(),
        admitted
            .source_authority_basis()
            .authority_branch_id()
            .to_string(),
        evidence.authoritative_mutation_origin,
        evidence.derivation_origin,
        admitted
            .source_authority_basis()
            .truth_basis_digest_hex()
            .to_string(),
        admitted.source_authority_basis().touched_aspect_count(),
        admitted
            .locality_basis()
            .triggered_invalidation_targets()
            .to_vec(),
        admitted.source_authority_basis().precision_fallback_count(),
        admitted
            .source_authority_basis()
            .precision_budget_fallback_count(),
        Some(&selected_equivalence_family),
        Some(selected.declaration().identity()),
        Some(&lowered),
        &materialized,
        &interpreted,
        validation,
    );
    Ok(DerivedReadDiagnostics {
        diagnostic_projection_source: topology_derived_diagnostic_projection_source(
            read_basis_metadata.read_basis(),
            &equivalence_contract_report,
        ),
        compiled_product_reuse_route_packet_identity: None,
        topology_reuse_posture: None,
        spatial_reuse_posture: None,
        spatial_reuse_decision_identity_digest: None,
        spatial_rebuild_denial_identity_digest: None,
        batch_admission_route_packet_identity: None,
        batch_admission_denial_witness_identity: None,
        batch_admission_denial_witness_kind: None,
        conflict_independence_route_packet_identity: None,
        conflict_independence_denial_witness_identity: None,
        conflict_independence_denial_witness_kind: None,
        invalidation_report: build_derived_invalidation_report_from_aspects(
            touched_aspects.iter().copied(),
        ),
        rebuild_report: build_derived_rebuild_report(&materialized, &interpreted, validation),
        fallback_report: build_derived_fallback_report_from_counts(
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
        ),
        validation_report: validation.clone(),
        validation_execution_report: derived_validation_execution_report(
            registered_validation.registered_rule_count(),
        ),
        equivalence_contract_report,
    })
}

fn registered_validation_report(
    validation: DerivedTopologyValidationReport,
) -> Result<RegisteredTopologyValidationReport, TopologyQuerySurfaceError> {
    RegisteredTopologyValidationReport::from_report(validation).map_err(|error| {
        TopologyQuerySurfaceError::with_kind(
            TopologyQuerySurfaceErrorKind::UnregisteredValidationReport,
            format!(
                "query-derived validation diagnostics rejected unregistered validation report: {error}"
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
    use crate::projection::runtime_boundary::declared_query_surfaces::materialize_declared_query_surface_row;
    use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
    use crate::validation::reference_integrity::milestone_one_runtime_builder;
    use crate::validation::DerivedTopologyValidationReport;
    use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

    use super::registered_validation_report;

    #[test]
    fn decoded_diagnostics_reject_unregistered_validation_rule_identity() {
        let forged_report: DerivedTopologyValidationReport = serde_json::from_value(json!({
            "rows": [{
                "validator": "fake_validator",
                "rule_identity": {
                    "namespace": "worth.topo.validation",
                    "name": "fake_validator",
                    "version": 1
                },
                "phase": "DerivedMaterialization",
                "input_class": "MaterializedTopologyView",
                "status": "passed"
            }]
        }))
        .expect("forged report should deserialize like retained query rows can");

        let error = registered_validation_report(forged_report)
            .expect_err("unregistered retained validation identity must fail closed");

        assert!(error
            .to_string()
            .contains("rejected unregistered validation report"));
    }

    #[test]
    fn decoded_diagnostics_reject_registered_identity_with_wrong_phase() {
        let forged_report: DerivedTopologyValidationReport = serde_json::from_value(json!({
            "rows": [
                registered_row("ownership", "DerivedInterpretation", "MaterializedTopologyView"),
                registered_row("loop_wiring", "DerivedMaterialization", "MaterializedTopologyView"),
                registered_row("radial_rings", "DerivedInterpretation", "InterpretedTopologyView"),
                registered_row("shell_closure", "DerivedInterpretation", "InterpretedTopologyView"),
                registered_row("vertex_disks", "DerivedInterpretation", "InterpretedTopologyView")
            ]
        }))
        .expect("semantically forged report should deserialize");

        let error = registered_validation_report(forged_report)
            .expect_err("wrong phase must not pass registered validation");

        assert!(error.to_string().contains("wrong phase"));
    }

    #[test]
    fn decoded_diagnostics_reject_missing_registered_rule() {
        let forged_report: DerivedTopologyValidationReport = serde_json::from_value(json!({
            "rows": [
                registered_row("ownership", "DerivedMaterialization", "MaterializedTopologyView"),
                registered_row("loop_wiring", "DerivedMaterialization", "MaterializedTopologyView"),
                registered_row("radial_rings", "DerivedInterpretation", "InterpretedTopologyView"),
                registered_row("shell_closure", "DerivedInterpretation", "InterpretedTopologyView")
            ]
        }))
        .expect("short report should deserialize");

        let error = registered_validation_report(forged_report)
            .expect_err("missing registered rule must fail closed");

        assert!(error
            .to_string()
            .contains("expected 5 registered validation rows"));
    }

    #[test]
    fn retained_diagnostics_surface_fails_closed_through_real_retained_refresh_boundary() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive_through_schema_execution(
            &mut runtime,
            "phase-two-retained-diagnostics",
            &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        )
        .expect("verified primitive");
        let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
            &runtime,
            verified.read_basis().clone(),
            "retained-diagnostics-truth-basis-proof",
        )
        .expect("read-basis query runtime should open");
        let materialized_surface = query_runtime.surfaces().materialized().clone();
        let diagnostics_surface = query_runtime.surfaces().diagnostics().clone();
        let materialized_row: serde_json::Value = materialize_declared_query_surface_row(
            query_runtime.workspace(),
            &materialized_surface,
        )
        .expect("materialized surface should return retained failure payload");
        let diagnostics_row: serde_json::Value =
            materialize_declared_query_surface_row(query_runtime.workspace(), &diagnostics_surface)
                .expect("diagnostics surface should return retained failure payload");

        assert_eq!(materialized_row["query_surface_error_kind"].as_str(), None);
        assert_eq!(
            diagnostics_row["query_surface_error_kind"].as_str(),
            Some("retained_payload_decode_failed")
        );
        assert!(
            materialized_row["query_surface_error"]
                .as_str()
                .is_some_and(|message| message.contains("Query-native retained materialization support")),
            "materialized surface should fail closed on the real retained runtime seam instead of silently rebuilding local truth: {materialized_row:?}",
        );
        assert!(
            diagnostics_row["query_surface_error"].as_str().is_some_and(|message| {
                message.contains("materialized topology")
                    && message.contains("payload failed to decode")
            }),
            "diagnostics surface should expose the real retained upstream failure instead of bypassing the refresh seam with a helper path: {diagnostics_row:?}",
        );
    }

    fn registered_row(name: &str, phase: &str, input_class: &str) -> serde_json::Value {
        json!({
            "validator": name,
            "rule_identity": {
                "namespace": "worth.topo.validation",
                "name": name,
                "version": 1
            },
            "phase": phase,
            "input_class": input_class,
            "status": "passed"
        })
    }
}

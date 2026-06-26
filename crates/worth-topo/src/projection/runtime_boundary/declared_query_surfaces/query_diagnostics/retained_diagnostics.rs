use forge_query::facade::{
    ForgeQueryDerivedView, ForgeQueryRetainedRefreshContext, ForgeQueryRetainedUpstreamInputs,
};
#[cfg(test)]
use serde_json::Value;

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::projection::diagnostic_surfaces::{
    build_derived_equivalence_contract_report,
    derived_read_diagnostics::{
        build_derived_fallback_report_from_counts, build_derived_invalidation_report_from_aspects,
        derived_validation_execution_report, triggered_invalidation_targets_from_aspects,
        DerivedReadDiagnostics,
    },
};
use crate::validation::{DerivedTopologyValidationReport, RegisteredTopologyValidationReport};

#[cfg(test)]
use super::super::derived_surfaces::decode_query_surface_row;
use super::super::retained_payload::decode_single_retained_payload_row;
use super::super::TopologyQuerySurfaceError;
use super::TopologyQueryMutationEvidence;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn derived_read_diagnostics_from_query_rows(
    refresh: &ForgeQueryRetainedRefreshContext,
    materialized_rows: &[Value],
    interpreted_rows: &[Value],
    validation_rows: &[Value],
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let evidence = TopologyQueryMutationEvidence::from_refresh(refresh)?;
    let materialized: MaterializedTopologyView =
        decode_query_surface_row(materialized_rows, "materialized topology")?;
    let interpreted: InterpretedTopologyView =
        decode_query_surface_row(interpreted_rows, "interpreted topology")?;
    let validation: DerivedTopologyValidationReport =
        decode_query_surface_row(validation_rows, "topology validation")?;

    build_diagnostics_from_decoded_surfaces(evidence, materialized, interpreted, validation)
}

pub(super) fn derived_read_diagnostics_from_upstreams(
    refresh: &ForgeQueryRetainedRefreshContext,
    view: &ForgeQueryDerivedView,
    upstreams: &ForgeQueryRetainedUpstreamInputs,
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let evidence = TopologyQueryMutationEvidence::from_refresh(refresh)?;
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

    build_diagnostics_from_decoded_surfaces(evidence, materialized, interpreted, validation)
}

fn build_diagnostics_from_decoded_surfaces(
    evidence: TopologyQueryMutationEvidence,
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
) -> Result<DerivedReadDiagnostics, TopologyQuerySurfaceError> {
    let touched_aspects = evidence.touched_aspects()?;
    let registered_validation = registered_validation_report(validation)?;
    let validation = registered_validation.report();

    Ok(DerivedReadDiagnostics {
        invalidation_report: build_derived_invalidation_report_from_aspects(
            touched_aspects.iter().copied(),
        ),
        rebuild_report: crate::projection::diagnostic_surfaces::derived_read_diagnostics::build_derived_rebuild_report(
            &materialized,
            &interpreted,
            validation,
        ),
        fallback_report: build_derived_fallback_report_from_counts(
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
        ),
        validation_report: validation.clone(),
        validation_execution_report: derived_validation_execution_report(
            registered_validation.registered_rule_count(),
        ),
        equivalence_contract_report: build_derived_equivalence_contract_report(
            evidence.authority_snapshot_id,
            evidence.authority_branch_id,
            evidence.authoritative_mutation_origin,
            evidence.derivation_origin,
            evidence.truth_basis_digest_hex,
            touched_aspects.len(),
            triggered_invalidation_targets_from_aspects(touched_aspects.iter().copied()),
            evidence.precision_fallback_count,
            evidence.precision_budget_fallback_count,
            &materialized,
            &interpreted,
            validation,
        ),
    })
}

fn registered_validation_report(
    validation: DerivedTopologyValidationReport,
) -> Result<RegisteredTopologyValidationReport, TopologyQuerySurfaceError> {
    RegisteredTopologyValidationReport::from_report(validation).map_err(|error| {
        TopologyQuerySurfaceError::new(format!(
            "query-derived validation diagnostics rejected unregistered validation report: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::validation::DerivedTopologyValidationReport;

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

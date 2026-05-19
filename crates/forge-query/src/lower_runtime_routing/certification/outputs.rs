use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    forge_query_lower_runtime_gap_registry, forge_query_lower_runtime_support_matrix,
};

use super::model::ForgeQueryLowerRuntimeCertificationOutputDigest;
use super::performance::ForgeQueryLowerRuntimePerformanceSlopeReport;
use super::surface::{
    ForgeQueryLowerRuntimeAcceptanceLane, ForgeQueryLowerRuntimeAcceptanceSuite,
    ForgeQueryLowerRuntimeRepresentativeSurface,
};
use super::{
    forge_query_lower_runtime_phase_progression_digest,
    forge_query_lower_runtime_proof_shape_digest, ForgeQueryLowerRuntimeNonBypassAudit,
};

pub(super) fn certification_output_digests(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
    acceptance: &ForgeQueryLowerRuntimeAcceptanceSuite,
    non_bypass: &ForgeQueryLowerRuntimeNonBypassAudit,
    slopes: &ForgeQueryLowerRuntimePerformanceSlopeReport,
) -> Vec<ForgeQueryLowerRuntimeCertificationOutputDigest> {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let gaps = forge_query_lower_runtime_gap_registry();
    let closeout = forge_query_lower_runtime_closeout_registry();
    let support = forge_query_lower_runtime_support_matrix();

    vec![
        output("query_digest", surface.query_digest().to_string()),
        output("capability_request_digest", digest_requests(surface)),
        output("capability_family_digest", digest_capability_families()),
        output(
            "capability_eligibility_digest",
            digest_eligibilities(surface),
        ),
        output(
            "lower_runtime_route_plan_digest",
            digest_route_plans(surface),
        ),
        output(
            "boundary_execution_receipt_digest",
            digest_boundary_receipts(surface),
        ),
        output(
            "lower_runtime_boundary_envelope_digest",
            digest_envelopes(surface),
        ),
        output("crossing_inventory_digest", crossings.inventory_digest()),
        output(
            "crossing_classification_digest",
            crossings.classification_digest(),
        ),
        output("compatibility_debt_registry_digest", gaps.registry_digest()),
        output(
            "debt_exit_criteria_digest",
            hash_parts(&[
                gaps.debt_exit_criteria_digest(),
                closeout.required_closeout_digest(),
            ]),
        ),
        output(
            "route_authority_digest",
            digest_envelope_field(surface, "authority"),
        ),
        output(
            "route_evidence_digest",
            digest_envelope_field(surface, "evidence"),
        ),
        output(
            "route_cost_posture_digest",
            digest_envelope_field(surface, "cost"),
        ),
        output(
            "route_failure_topology_digest",
            digest_envelope_field(surface, "failure"),
        ),
        output("route_support_matrix_digest", support.matrix_digest()),
        output(
            "route_public_surface_digest",
            non_bypass.route_public_surface_digest().to_string(),
        ),
        output("route_target_dx_digest", surface.dx_digest().to_string()),
        output(
            "route_golden_transcript_digest",
            surface.golden_transcript_digest().to_string(),
        ),
        output(
            "route_concrete_surface_digest",
            surface.concrete_surface_digest(),
        ),
        output(
            "route_synthetic_surface_digest",
            surface.synthetic_surface_digest(),
        ),
        output(
            "route_proof_shape_digest",
            forge_query_lower_runtime_proof_shape_digest(),
        ),
        output(
            "route_phase_progression_digest",
            forge_query_lower_runtime_phase_progression_digest(),
        ),
        output(
            "route_parity_digest",
            surface.route_parity_digest().to_string(),
        ),
        output(
            "route_non_bypass_digest",
            non_bypass.route_non_bypass_digest().to_string(),
        ),
        output("lower_runtime_gap_registry_digest", gaps.registry_digest()),
        output(
            "compile_fail_boundary_digest",
            non_bypass.compile_fail_boundary_digest().to_string(),
        ),
        output(
            "failure_digest",
            hash_parts(&[
                acceptance
                    .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
                    .digest()
                    .to_string(),
                deferred_neighbor_failure_digest(),
                downstream_boundary_failure_digest(non_bypass),
            ]),
        ),
        output(
            "counter_snapshot",
            hash_parts(&[
                format!("crossings:{}", crossings.rows().len()),
                format!("gaps:{}", gaps.rows().len()),
                format!("closeout:{}", closeout.rows().len()),
                format!("support:{}", support.rows().len()),
                format!("routes:{}", surface.route_plans().len()),
                format!("evidence:{}", surface.envelopes().len()),
                format!("checked_files:{}", non_bypass.checked_file_count()),
            ]),
        ),
        output(
            "crossing_inventory_width",
            crossings.rows().len().to_string(),
        ),
        output("compatibility_debt_width", gaps.rows().len().to_string()),
        output("route_plan_width", surface.route_plans().len().to_string()),
        output(
            "boundary_evidence_width",
            surface.envelopes().len().to_string(),
        ),
        output(
            "route_concrete_surface_width",
            surface.concrete_surface_width().to_string(),
        ),
        output(
            "route_synthetic_surface_width",
            surface.synthetic_surface_width().to_string(),
        ),
        output(
            "capability_eligibility_slope_digest",
            slope_digest(slopes, "capability_eligibility_slope_digest"),
        ),
        output(
            "route_plan_assembly_slope_digest",
            slope_digest(slopes, "route_plan_assembly_slope_digest"),
        ),
        output(
            "boundary_receipt_assembly_slope_digest",
            slope_digest(slopes, "boundary_receipt_assembly_slope_digest"),
        ),
        output(
            "boundary_envelope_assembly_slope_digest",
            slope_digest(slopes, "boundary_envelope_assembly_slope_digest"),
        ),
        output(
            "support_lookup_slope_digest",
            slope_digest(slopes, "support_lookup_slope_digest"),
        ),
        output(
            "debt_registry_lookup_slope_digest",
            slope_digest(slopes, "debt_registry_lookup_slope_digest"),
        ),
    ]
}

fn output(name: &'static str, digest: String) -> ForgeQueryLowerRuntimeCertificationOutputDigest {
    ForgeQueryLowerRuntimeCertificationOutputDigest::new(name, digest)
}

fn digest_requests(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(
        &surface
            .requests()
            .iter()
            .map(|request| request.request_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_eligibilities(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(
        &surface
            .eligibilities()
            .iter()
            .map(|eligibility| eligibility.eligibility_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_route_plans(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(
        &surface
            .route_plans()
            .iter()
            .map(|plan| plan.route_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_boundary_receipts(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(
        &surface
            .boundary_receipts()
            .iter()
            .map(|receipt| receipt.boundary_execution_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_envelopes(surface: &ForgeQueryLowerRuntimeRepresentativeSurface) -> String {
    hash_parts(
        &surface
            .envelopes()
            .iter()
            .map(|envelope| envelope.envelope_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_capability_families() -> String {
    hash_parts(
        &forge_query_lower_runtime_crossing_inventory()
            .rows()
            .iter()
            .map(|row| row.capability_label().to_string())
            .collect::<Vec<_>>(),
    )
}

fn digest_envelope_field(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
    field: &str,
) -> String {
    hash_parts(
        &surface
            .envelopes()
            .iter()
            .map(|envelope| match field {
                "authority" => envelope.route_authority_digest().to_string(),
                "evidence" => envelope.route_evidence_digest().to_string(),
                "cost" => envelope.route_cost_posture().as_str().to_string(),
                _ => envelope.route_failure_topology().as_str().to_string(),
            })
            .collect::<Vec<_>>(),
    )
}

fn slope_digest(
    report: &ForgeQueryLowerRuntimePerformanceSlopeReport,
    output_name: &str,
) -> String {
    report
        .digest_for_output(output_name)
        .unwrap_or_else(|| panic!("missing slope digest {output_name}"))
        .to_string()
}

fn deferred_neighbor_failure_digest() -> String {
    hash_parts(
        &forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .filter(|row| row.posture().as_str() == "deferred-neighbor")
            .map(|row| {
                format!(
                    "{}|{}|{}",
                    row.seam_key().as_str(),
                    row.closeout_target(),
                    row.required_closeout()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn downstream_boundary_failure_digest(non_bypass: &ForgeQueryLowerRuntimeNonBypassAudit) -> String {
    hash_parts(&[
        "hostile_projection_file_outside_runtime_boundary_is_rejected".to_string(),
        non_bypass.route_public_surface_digest().to_string(),
        non_bypass.compile_fail_boundary_digest().to_string(),
    ])
}

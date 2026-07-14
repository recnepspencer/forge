use super::model::{
    WorthQueryDeclarativeCapabilityFamily as Family,
    WorthQueryDeclarativePhaseResponsibility as Phase, WorthQueryDeclarativeSurfaceClass as Class,
    WorthQueryDeclarativeSurfaceRow as Row,
};

pub(super) fn public_phase_exposure_rows() -> &'static [Row] {
    static ROWS: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        EXPOSURES
            .iter()
            .map(|exposure| exposure_row(exposure.source, exposure.function))
            .collect()
    })
}

struct Exposure {
    source: &'static str,
    function: &'static str,
}

#[rustfmt::skip]
const EXPOSURES: &[Exposure] = &[
    exposure("src/ordinary/count/mod.rs", "declare_count"),
    exposure("src/ordinary/live/mod.rs", "declare_live"),
    exposure("src/ordinary/read/context/mod.rs", "current"),
    exposure("src/ordinary/read/mod.rs", "*"),
    exposure("src/ordinary/read/mod.rs", "declare"),
    exposure("src/facade/exports_certification.rs", "inspect_lower_runtime_closeout"),
    exposure("src/facade/exports_foundation.rs", "admit_runtime_current_snapshot_basis"),
    exposure("src/facade/exports_foundation.rs", "resolve_runtime_current_snapshot_basis"),
    exposure("src/facade/exports_foundation.rs", "resolve_snapshot_basis"),
    exposure("src/facade/exports_foundation.rs", "resolve_bindings"),
    exposure("src/facade/exports_foundation.rs", "canonicalize_request"),
    exposure("src/facade/exports_foundation.rs", "resolve_correspondence_evidence"),
    exposure("src/facade/exports_foundation.rs", "compose_correspondence_historical_envelope"),
    exposure("src/facade/exports_foundation.rs", "compose_historical_admission_denied_envelope"),
    exposure("src/facade/exports_foundation.rs", "compose_historical_path_denied_envelope"),
    exposure("src/facade/exports_foundation.rs", "declare_branch_compare_from_live_sessions"),
    exposure("src/facade/exports_foundation.rs", "declare_live_query_session"),
    exposure("src/facade/exports_foundation.rs", "declare_runtime_live_query_session"),
    exposure("src/facade/exports_foundation.rs", "declare_writeback_from_live_session"),
    exposure("src/facade/exports_foundation.rs", "admit_effect_batch_components"),
    exposure("src/facade/exports_foundation.rs", "admit_effect_intent"),
    exposure("src/facade/exports_foundation.rs", "execute_lowered_effect_plan"),
    exposure("src/facade/exports_foundation.rs", "lower_authority_scoped_effect_plan"),
    exposure("src/facade/exports_foundation.rs", "execute_parallel_admission_route"),
    exposure("src/facade/exports_foundation.rs", "execute_serial_fallback_route"),
    exposure("src/facade/exports_foundation.rs", "execute_preflight_bundle"),
    exposure("src/facade/exports_foundation.rs", "admit_historical_evaluation_path"),
    exposure("src/facade/exports_foundation.rs", "materialization_metadata_from_resolved"),
    exposure("src/facade/exports_foundation.rs", "resolve_historical_materialization_path"),
    exposure("src/facade/exports_foundation.rs", "admit_identity_evolution_query"),
    exposure("src/facade/exports_foundation.rs", "execute_admitted_identity_evolution_query"),
    exposure("src/facade/exports_foundation.rs", "admit_region_scoped_live_plan"),
    exposure("src/facade/exports_foundation.rs", "execute_live_change"),
    exposure("src/facade/exports_foundation.rs", "execute_region_scoped_live_change"),
    exposure("src/facade/exports_foundation.rs", "lower_region_scoped_execution_to_stream_contract"),
    exposure("src/facade/exports_foundation.rs", "admit_authored_entity_token"),
    exposure("src/facade/exports_foundation.rs", "admit_external_commit_token"),
    exposure("src/facade/exports_foundation.rs", "admit_external_snapshot_token"),
    exposure("src/facade/exports_policy.rs", "plan_validated_bundle"),
    exposure("src/facade/exports_policy.rs", "plan_validated_bundle_for_collection_family"),
    exposure("src/facade/exports_policy.rs", "admit_policy_tenant_context"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_delivery_shape"),
    exposure("src/facade/exports_policy.rs", "admit_policy_aware_live_plan"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_branch_plan"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_current_plan"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_diff_plan"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_historical_plan"),
    exposure("src/facade/exports_policy.rs", "lower_policy_aware_optimizer_input"),
    exposure("src/facade/exports_policy.rs", "admit_authoritative_preview_comparison_candidate"),
    exposure("src/facade/exports_policy.rs", "admit_preview_promotion_parity_comparison"),
    exposure("src/facade/exports_policy.rs", "admit_preview_workflow_foundation"),
    exposure("src/facade/exports_policy.rs", "admit_preview_workflow_foundation_request"),
    exposure("src/facade/exports_policy.rs", "admit_promotion_eligible_preview_session_plan_binding"),
    exposure("src/facade/exports_policy.rs", "admit_read_only_preview_session_plan_binding"),
    exposure("src/facade/exports_policy.rs", "admit_scoped_preview_live_session_plan"),
    exposure("src/facade/exports_policy.rs", "admit_scoped_preview_session_plan_binding"),
    exposure("src/facade/exports_policy.rs", "admit_scoped_preview_session_plan_binding_from_preview_binding"),
    exposure("src/facade/exports_policy.rs", "bind_preflight_to_preview_session"),
    exposure("src/facade/exports_policy.rs", "execute_promotion_eligible_preview_session_plan"),
    exposure("src/facade/exports_policy.rs", "execute_read_only_preview_session_plan"),
    exposure("src/facade/exports_policy.rs", "execute_scoped_preview_live_session_plan"),
    exposure("src/facade/exports_policy.rs", "scoped_observation_basis_for_preview_binding"),
    exposure("src/facade/exports_policy.rs", "admit_query_basis_context"),
    exposure("src/facade/exports_policy.rs", "bind_diff_query_context"),
    exposure("src/facade/exports_policy.rs", "execute_and_build_query_basis_result_bundle"),
    exposure("src/facade/exports_policy.rs", "execute_query_basis_context"),
    exposure("src/facade/exports_policy.rs", "admit_relationship_proofs"),
    exposure("src/facade/exports_live_capability.rs", "declare"),
    exposure("src/facade/exports_live_capability.rs", "current"),
    exposure("src/facade/exports_read.rs", "declare_count"),
    exposure("src/facade/exports_read.rs", "current"),
    exposure("src/facade/exports_read.rs", "declare"),
    exposure("src/facade/exports_runtime.rs", "admit_causal_inspection"),
    exposure("src/facade/exports_runtime.rs", "admit_graph_read_access_authority"),
    exposure("src/facade/exports_runtime.rs", "admit_graph_read_access_authority_from_policy_tenant_request"),
    exposure("src/facade/exports_runtime.rs", "admit_graph_read_access_for_family"),
    exposure("src/facade/exports_runtime.rs", "admit_graph_read_access_for_family_in_authority"),
    exposure("src/facade/exports_runtime.rs", "explain_boolean_selectivity_shape_for_family"),
    exposure("src/facade/exports_runtime.rs", "explain_boolean_selectivity_shape_for_family_with_operation_registry"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirement_outcome_for_family"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirement_outcome_for_family_in_authority"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirement_outcome_for_family_with_operation_registry"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirements_for_family"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirements_for_family_in_authority"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_requirements_for_family_with_operation_registry"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_shape_for_family"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_shape_for_family_in_authority"),
    exposure("src/facade/exports_runtime.rs", "explain_graph_read_access_shape_for_family_with_operation_registry"),
    exposure("src/facade/exports_runtime.rs", "inspect_lower_runtime_boundary"),
    exposure("src/facade/exports_runtime.rs", "plan_admitted_graph_read_access_for_family"),
    exposure("src/facade/exports_runtime.rs", "plan_admitted_graph_read_access_for_family_in_authority"),
    exposure("src/facade/exports_runtime.rs", "resolve_causal_evidence_references"),
    exposure("src/facade/exports_runtime.rs", "resolve_graph_read_operations_for_family_in_authority_with_registry"),
    exposure("src/facade/exports_runtime.rs", "resolve_graph_read_operations_for_family_with_registry"),
    exposure("src/facade/exports_runtime.rs", "resolve_indexed_causal_evidence_references"),
    exposure("src/facade/exports_runtime_capabilities.rs", "admit_eligible_domain_capability_contribution"),
    exposure("src/facade/exports_runtime_products.rs", "admit_active_subscription_lane"),
    exposure("src/facade/exports_runtime_products.rs", "admit_preview_subscription_isolation"),
    exposure("src/facade/exports_runtime_products.rs", "admit_query_subscription"),
    exposure("src/facade/exports_runtime_products.rs", "admit_subscription_continuation_evidence"),
    exposure("src/facade/exports_runtime_products.rs", "declare_query_subscription"),
    exposure("src/facade/exports_runtime_products.rs", "explain_query_subscription_bridge_parity"),
    exposure("src/facade/exports_runtime_products.rs", "lower_query_subscription_maintenance_delta"),
    exposure("src/facade/exports_runtime_products.rs", "lower_query_subscription_to_bridge"),
    exposure("src/facade/exports_runtime_products.rs", "validate_canonical_bundle"),
    exposure("src/facade/exports_runtime_products.rs", "admit_view_shape"),
    exposure("src/facade/exports_runtime_products.rs", "plan_admitted_view_shape"),
    exposure("src/facade/exports_runtime_products.rs", "validate_canonical_bundle_for_admitted_view_shape"),
    exposure("src/facade/exports_runtime_products.rs", "admit_grouped_live_view"),
    exposure("src/facade/exports_runtime_products.rs", "execute_grouped_live_view_shape_change"),
    exposure("src/facade/exports_runtime_products.rs", "execute_live_view_shape_change"),
    exposure("src/facade/exports_runtime_products.rs", "lower_view_shape_plan_to_live"),
    exposure("src/facade/exports_runtime_products.rs", "admit_query_workflow_declaration"),
    exposure("src/facade/exports_runtime_products.rs", "bind_workflow_context"),
    exposure("src/facade/exports_runtime_products.rs", "inspect_merge_conflicts"),
    exposure("src/facade/exports_runtime_products.rs", "inspect_post_merge_outcome"),
    exposure("src/facade/exports_runtime_products.rs", "lower_merge_workflow_declaration"),
    exposure("src/facade/exports_runtime_products.rs", "lower_mutation_intent_declaration"),
    exposure("src/facade/exports_runtime_products.rs", "lower_query_writeback_declaration"),
    exposure("src/live/mod.rs", "admit_region_scoped_live_plan"),
    exposure("src/live/mod.rs", "execute_region_scoped_live_change"),
    exposure("src/live/mod.rs", "lower_region_scoped_execution_to_stream_contract"),
    exposure("src/preview/mod.rs", "admit_scoped_preview_live_session_plan"),
    exposure("src/preview/mod.rs", "admit_scoped_preview_session_plan_binding"),
    exposure("src/preview/mod.rs", "admit_scoped_preview_session_plan_binding_from_preview_binding"),
    exposure("src/preview/mod.rs", "execute_scoped_preview_live_session_plan"),
    exposure("src/preview/mod.rs", "scoped_observation_basis_for_preview_binding"),
];

const fn exposure(source: &'static str, function: &'static str) -> Exposure {
    Exposure { source, function }
}

fn exposure_row(source: &'static str, function: &'static str) -> Row {
    let ordinary = matches!(
        source,
        "src/facade/exports_live_capability.rs" | "src/facade/exports_read.rs"
    ) || source.starts_with("src/ordinary/");
    let certification = source == "src/facade/exports_certification.rs";
    let (current_class, target_class, consumer) = if ordinary {
        (
            Class::OrdinaryDeclaration,
            Class::OrdinaryDeclaration,
            "ordinary capability consumer",
        )
    } else if certification {
        (
            Class::Certification,
            Class::Certification,
            "Query certification",
        )
    } else {
        (
            Class::Compatibility,
            Class::InternalMechanism,
            "advanced integration or Query implementation",
        )
    };
    let family = if source.starts_with("src/ordinary/read/") {
        Family::Read
    } else {
        exposure_family(function)
    };
    Row::new(
        source,
        function,
        family,
        if function == "*" {
            Phase::Refine
        } else {
            exposure_phase(function)
        },
        current_class,
        target_class,
        consumer,
        replacement_for(family),
    )
}

fn exposure_family(function: &str) -> Family {
    if function.contains("historical") {
        Family::Historical
    } else if function.contains("correspondence")
        || function.contains("diff")
        || function.contains("compare")
        || function.contains("comparison")
    {
        Family::Comparison
    } else if function.contains("preview") {
        Family::Preview
    } else if function.contains("mutation")
        || function.contains("effect")
        || function.contains("writeback")
    {
        Family::Mutation
    } else if function.contains("workflow") || function.contains("merge") {
        Family::Workflow
    } else if function.contains("inspect") || function.contains("causal") {
        Family::Inspection
    } else if function.contains("count") {
        Family::Aggregate
    } else if function.contains("subscription") || function.contains("live") {
        Family::Live
    } else if function.contains("read") || function.contains("view_shape") {
        Family::Read
    } else if function.contains("domain_capability") {
        Family::DomainExtension
    } else {
        Family::GeneralDeclaration
    }
}

fn exposure_phase(function: &str) -> Phase {
    if function == "current" {
        Phase::Refine
    } else if function.starts_with("declare") || function.starts_with("compose") {
        Phase::Declare
    } else if function.starts_with("canonicalize") {
        Phase::Canonicalize
    } else if function.starts_with("bind") || function.starts_with("resolve") {
        Phase::Bind
    } else if function.starts_with("validate") {
        Phase::Validate
    } else if function.starts_with("admit") {
        Phase::Admit
    } else if function.starts_with("plan") {
        Phase::Plan
    } else if function.starts_with("lower") {
        Phase::Lower
    } else if function.starts_with("execute") {
        Phase::Execute
    } else if function.starts_with("inspect") || function.starts_with("explain") {
        Phase::Inspect
    } else {
        Phase::AssembleOutcome
    }
}

fn replacement_for(family: Family) -> &'static str {
    match family {
        Family::Read | Family::Aggregate => "ordinary read declaration",
        Family::Live => "managed live declaration",
        Family::Historical => "ordinary historical declaration",
        Family::Comparison => "ordinary comparison declaration",
        Family::Preview => "ordinary preview declaration",
        Family::Mutation => "ordinary mutation declaration",
        Family::Workflow => "ordinary workflow declaration",
        Family::Inspection => "ordinary inspection declaration",
        Family::DomainExtension => "typed domain extension contract",
        Family::GeneralDeclaration => "capability-specific ordinary declaration",
    }
}

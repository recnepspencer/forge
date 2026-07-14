use std::sync::OnceLock;

use super::{
    WorthQueryDomainAuthorityClass as Class, WorthQueryDomainAuthorityInventoryRow as Row,
};

const APPLICATION_FACADE: &str = "src/application/capability/facade.rs";
const RUNTIME_BUILDER: &str = "src/runtime/builder.rs";
const OPERATION_REGISTRY: &str = "src/runtime/graph_read_access/operation_resolution/registry.rs";
const RUNTIME_CORE_EXPORTS: &str = "src/facade/exports_runtime_core.rs";
const OPERATION_EXPLANATION: &str = "src/runtime/graph_read_access/explanation_api.rs";
const RUNTIME_CAPABILITY_EXPORTS: &str = "src/facade/exports_runtime_capabilities.rs";
const RAW_CONTRIBUTION_ROOT: &str = "src/domain_capabilities/dx/common/root.rs";

pub fn worth_query_domain_authority_inventory_rows() -> &'static [Row] {
    static ROWS: OnceLock<Vec<Row>> = OnceLock::new();
    ROWS.get_or_init(|| {
        let mut rows = CORE_ROWS.to_vec();
        rows.extend(LEGACY_MATERIALIZATION_EXPORTS.iter().map(|symbol| {
            Row::new(
                symbol,
                RUNTIME_CAPABILITY_EXPORTS,
                Some(RUNTIME_CAPABILITY_EXPORTS),
                Class::CompatibilityPath,
                Class::ProhibitedCompetingAuthority,
                "installed-domain-handle",
            )
        }));
        rows
    })
}

const CORE_ROWS: &[Row] = &[
    Row::new(
        "worth_query_domain",
        RAW_CONTRIBUTION_ROOT,
        Some(RUNTIME_CAPABILITY_EXPORTS),
        Class::CompatibilityPath,
        Class::ProhibitedCompetingAuthority,
        "installed-domain-handle",
    ),
    legacy(
        "WorthQueryApplicationFacade::domain",
        APPLICATION_FACADE,
        "domain-package",
    ),
    legacy(
        "WorthQueryApplicationFacade::domain_checked",
        APPLICATION_FACADE,
        "package-admission",
    ),
    legacy(
        "WorthQueryApplicationFacade::domain_proof_root",
        APPLICATION_FACADE,
        "package-admission",
    ),
    legacy(
        "WorthQueryApplicationFacade::domain_entry_support_snapshot",
        APPLICATION_FACADE,
        "diagnostic-projection",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::invariant_catalog",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::invariant_registration_artifact",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::graph_obligation",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::graph_scoped_custom_invariant",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::custom_invariant",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    legacy(
        "WorthQueryRuntimeBuilder::register_invariant",
        RUNTIME_BUILDER,
        "domain-package",
    ),
    Row::new(
        "WorthQueryGraphReadOperationRegistry",
        OPERATION_REGISTRY,
        Some(RUNTIME_CORE_EXPORTS),
        Class::CompatibilityPath,
        Class::ProhibitedCompetingAuthority,
        "installed-domain-execution-index",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::empty",
        OPERATION_REGISTRY,
        "installed-domain-execution-index",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::define",
        OPERATION_REGISTRY,
        "domain-package",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::admit",
        OPERATION_REGISTRY,
        "package-admission",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::with_registration",
        OPERATION_REGISTRY,
        "domain-package",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::admit_registration",
        OPERATION_REGISTRY,
        "package-admission",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::with_required_capability_for_relations",
        OPERATION_REGISTRY,
        "domain-package",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::with_unsupported_shape_for_relations",
        OPERATION_REGISTRY,
        "domain-package",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::with_unsupported_shape_for_operation",
        OPERATION_REGISTRY,
        "domain-package",
    ),
    legacy(
        "WorthQueryGraphReadOperationRegistry::registrations",
        OPERATION_REGISTRY,
        "diagnostic-projection",
    ),
    legacy(
        "explain_graph_read_access_shape_for_family_with_operation_registry",
        OPERATION_EXPLANATION,
        "installed-domain-execution-index",
    ),
    legacy(
        "explain_boolean_selectivity_shape_for_family_with_operation_registry",
        OPERATION_EXPLANATION,
        "installed-domain-execution-index",
    ),
    legacy(
        "explain_graph_read_access_requirement_outcome_for_family_with_operation_registry",
        OPERATION_EXPLANATION,
        "installed-domain-execution-index",
    ),
    legacy(
        "explain_graph_read_access_requirements_for_family_with_operation_registry",
        OPERATION_EXPLANATION,
        "installed-domain-execution-index",
    ),
];

const fn legacy(symbol: &'static str, path: &'static str, owner: &'static str) -> Row {
    Row::new(
        symbol,
        path,
        None,
        Class::CompatibilityPath,
        Class::ProhibitedCompetingAuthority,
        owner,
    )
}

const LEGACY_MATERIALIZATION_EXPORTS: &[&str] = &[
    "prepare_admitted_domain_capability_contribution_for_materialization",
    "materialize_admission_explanation_bundle",
    "materialize_admission_summary",
    "materialize_admission_support_report",
    "materialize_admission_trace_artifact",
    "materialize_admitted_preview_workflow_foundation",
    "materialize_admitted_projection_consumption",
    "materialize_aftermath_explanation_bundle",
    "materialize_aftermath_summary",
    "materialize_aftermath_support_report",
    "materialize_aftermath_trace_artifact",
    "materialize_canonical_admission_artifact",
    "materialize_canonical_aftermath_artifact",
    "materialize_canonical_continuity_artifact",
    "materialize_canonical_explanation_artifact",
    "materialize_canonical_invariant_capability_artifact",
    "materialize_canonical_support_traceability_artifact",
    "materialize_canonical_workflow_artifact",
    "materialize_continuity_explanation_bundle",
    "materialize_continuity_summary",
    "materialize_continuity_support_report",
    "materialize_continuity_trace_artifact",
    "materialize_correspondence_evidence_resolved",
    "materialize_domain_capability_canonical_runtime_artifact",
    "materialize_domain_capability_explanation_bundle",
    "materialize_domain_capability_summary",
    "materialize_domain_capability_support_report",
    "materialize_domain_capability_trace_artifact",
    "materialize_explanation_explanation_bundle",
    "materialize_explanation_summary",
    "materialize_explanation_support_report",
    "materialize_explanation_trace_artifact",
    "materialize_graph_composition_capability_support_row",
    "materialize_graph_composition_domain_invariant_denial",
    "materialize_intent_admission_support_traceability_report",
    "materialize_intent_admission_support_traceability_row",
    "materialize_intent_declaration_support_traceability_artifact",
    "materialize_invariant_capability_explanation_bundle",
    "materialize_invariant_capability_summary",
    "materialize_invariant_capability_support_report",
    "materialize_invariant_capability_trace_artifact",
    "materialize_lower_runtime_support_traceability_artifact",
    "materialize_lowered_merge_workflow_declaration",
    "materialize_lowered_mutation_intent_declaration",
    "materialize_projection_consumption_contract",
    "materialize_projection_consumption_eligibility",
    "materialize_projection_consumption_review",
    "materialize_projection_consumption_support_report",
    "materialize_query_causal_inspection_artifact",
    "materialize_query_causal_inspection_review",
    "materialize_query_conflict_inspection_artifact",
    "materialize_query_invariant_catalog_registration_artifact",
    "materialize_query_post_merge_inspection_artifact",
    "materialize_query_preview_workflow_artifact",
    "materialize_query_workflow_declaration",
    "materialize_query_writeback_lowering",
    "materialize_runtime_admission_decision",
    "materialize_runtime_admission_support_traceability_report",
    "materialize_runtime_admission_support_traceability_row",
    "materialize_runtime_continuity_evidence",
    "materialize_support_traceability_explanation_bundle",
    "materialize_support_traceability_summary",
    "materialize_support_traceability_support_report",
    "materialize_support_traceability_trace_artifact",
    "materialize_workflow_explanation_bundle",
    "materialize_workflow_summary",
    "materialize_workflow_support_report",
    "materialize_workflow_trace_artifact",
];

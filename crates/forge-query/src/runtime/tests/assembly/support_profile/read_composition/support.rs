use super::super::super::super::support::*;

#[test]
fn runtime_public_read_composition_support_report_freezes_phase_one_kernel_surface() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.read-composition-support")
        .expect("task runtime should open a named workspace");
    let report = workspace.public_read_composition_support_report();

    assert_eq!(
        report.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(
        report.entry_points(),
        &[
            "compose_read",
            "compose_read_with_invariant_pack",
            "define_read_family",
            "define_read_family_with_invariant_pack",
            "execute_read_family",
            "execute_read_family_in_basis_context",
        ]
    );
    assert_eq!(
        report.scope_classes(),
        &[
            "local_neighborhood",
            "anchored_expansion",
            "explicit_broad_search",
        ]
    );
    assert_eq!(report.graph_families(), &["detail", "collection"]);
    assert_eq!(
        report.execution_engines(),
        &[
            "query_runtime_current",
            "query_runtime_branch",
            "query_runtime_historical",
            "query_runtime_preview_derived",
        ]
    );
    assert_eq!(
        report.fallback_classes(),
        &["none", "snapshot_indexed_debt", "whole_view_debt"]
    );
    assert_eq!(
        report.built_in_operators(),
        &[
            "direct_edge",
            "successor_walk",
            "shared_endpoint",
            "shared_attachment",
            "bounded_ancestor",
            "bounded_descendant",
            "anchored_frontier",
            "frontier_search",
        ]
    );
    assert_eq!(
        report.relationship_proof_postures(),
        &["not_required", "descriptor_admitted_synthetic_runtime"]
    );
    assert_eq!(
        report.family_admission_modes(),
        &["kernel_only", "domain_invariant_admitted"]
    );
    assert_eq!(
        report.extension_hook_families(),
        &[
            "domain_read_family_lowering",
            "domain_invariant_pack",
            "domain_decoder",
            "domain_result_certification",
        ]
    );
    assert!(report
        .boundary_guards()
        .contains(&"operator_owned_builders_hide_traverse"));
    assert!(report
        .boundary_guards()
        .contains(&"scope_class_relabeling_denies_typed"));
    assert!(report
        .boundary_guards()
        .contains(&"domain_invariant_pack_denies_before_execution"));
    assert!(report.denial_lanes().contains(&"built_in_operator_denied"));
    assert!(report
        .denial_lanes()
        .contains(&"relationship_proof_admission_denied"));
    assert!(report.denial_lanes().contains(&"domain_invariant_denied"));
    assert!(report.rows().iter().any(|row| {
        row.capability_family() == "frontier_search"
            && row.capability_class() == ForgeQueryReadCompositionSupportClass::BuiltInOperator
    }));
    assert!(report.rows().iter().any(|row| {
        row.capability_family() == "whole_view_debt"
            && row.capability_class() == ForgeQueryReadCompositionSupportClass::FallbackClass
    }));
    assert!(report.rows().iter().any(|row| {
        row.capability_family() == "domain_decoder"
            && row.capability_class() == ForgeQueryReadCompositionSupportClass::ExtensionHook
    }));
    assert!(report.rows().iter().any(|row| {
        row.capability_family() == "operator_owned_builders_hide_traverse"
            && row.capability_class() == ForgeQueryReadCompositionSupportClass::BoundaryGuard
    }));
    assert_extension_hook_boundary(
        &report,
        ForgeQueryReadCompositionExtensionHookFamily::DomainReadFamilyLowering,
        ForgeQueryReadCompositionExtensionHookBoundary::Lowering,
    );
    assert_extension_hook_boundary(
        &report,
        ForgeQueryReadCompositionExtensionHookFamily::DomainInvariantPack,
        ForgeQueryReadCompositionExtensionHookBoundary::InvariantPack,
    );
    assert_extension_hook_boundary(
        &report,
        ForgeQueryReadCompositionExtensionHookFamily::DomainDecoder,
        ForgeQueryReadCompositionExtensionHookBoundary::Decoder,
    );
    assert_extension_hook_boundary(
        &report,
        ForgeQueryReadCompositionExtensionHookFamily::DomainResultCertification,
        ForgeQueryReadCompositionExtensionHookBoundary::Certification,
    );
    assert!(!report.support_digest().is_empty());
}

fn assert_extension_hook_boundary(
    report: &ForgeQueryReadCompositionSupportReport,
    family: ForgeQueryReadCompositionExtensionHookFamily,
    boundary: ForgeQueryReadCompositionExtensionHookBoundary,
) {
    assert!(report.extension_hooks().iter().any(|row| {
        row.family() == family && row.boundary() == boundary && !row.semantic_bypass_allowed()
    }));
}

use super::super::super::support::*;

#[test]
fn runtime_authoritative_mutation_support_exposes_graph_composition_capability_rows() {
    let support = WorthQueryRuntime::public_authoritative_mutation_evidence_support_for_posture(
        WorthQueryRuntimeBackendPosture::Scaffold,
    );
    let rows = support.graph_composition_capability_support_rows();

    assert_eq!(rows.len(), 13);
    assert!(rows.iter().any(|row| {
        row.capability_family() == "same_batch_entity_relation_identity_edges"
            && row.capability_class()
                == WorthQueryGraphCompositionCapabilityClass::TargetCombination
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_and_symbolic_entity_identity_edges"
            && row.capability_class()
                == WorthQueryGraphCompositionCapabilityClass::TargetCombination
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "same_batch_symbolic_entity_followup_mutation"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_followup_mutation"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_retarget"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_supersession"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_retirement"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_verified_retarget"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_verified_supersession"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_verified_followup_mutation"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().any(|row| {
        row.capability_family() == "mixed_existing_target_verified_retirement"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(rows.iter().all(|row| !row.row_digest().is_empty()));
}

#[test]
fn runtime_authoritative_mutation_support_exposes_graph_composition_hook_rows() {
    let support = WorthQueryRuntime::public_authoritative_mutation_evidence_support_for_posture(
        WorthQueryRuntimeBackendPosture::Scaffold,
    );
    let rows = support.graph_composition_extension_hook_support_rows();

    assert_eq!(rows.len(), 2);
    assert!(rows.iter().any(|row| {
        row.hook_family() == "domain_lowering_hook"
            && row.boundary() == WorthQueryGraphCompositionExtensionHookBoundary::Lowering
    }));
    assert!(rows.iter().any(|row| {
        row.hook_family() == "domain_interpretation_hook"
            && row.boundary() == WorthQueryGraphCompositionExtensionHookBoundary::Interpretation
    }));
    assert!(rows.iter().all(|row| !row.semantic_bypass_allowed()));
}

#[test]
fn runtime_authoritative_mutation_support_includes_contributed_graph_capability_rows() {
    let support_profile = WorthQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_graph_composition_capability_support_row(
        WorthQueryGraphCompositionCapabilitySupportRow::new(
            "graph.face_inner_loop_insertion",
            WorthQueryGraphCompositionCapabilityClass::LifecycleStep,
        ),
    );
    let support =
        WorthQueryRuntime::public_authoritative_mutation_evidence_support_for_support_profile(
            &support_profile,
        );
    let rows = support.graph_composition_capability_support_rows();

    assert!(rows.iter().any(|row| {
        row.capability_family() == "graph.face_inner_loop_insertion"
            && row.capability_class() == WorthQueryGraphCompositionCapabilityClass::LifecycleStep
    }));
    assert!(support
        .graph_composition_families()
        .contains(&"graph.face_inner_loop_insertion".to_string()));
}

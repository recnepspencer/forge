use super::{
    current_admission::view_action,
    fixture::{capability_world, request_scope, GrantSpec},
};
use bank_domain::{
    estate::EstateWorkflowStage,
    schema::{ViewEstateAdministrationCapability, ViewRestrictedEstateOperation},
};

#[test]
fn unrelated_grant_population_does_not_enter_warm_capability_work() {
    let baseline = capability_world(
        "scale-baseline",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let expanded = capability_world(
        "scale-expanded",
        GrantSpec::view(),
        EstateWorkflowStage::Administration,
        false,
        128,
    );
    let baseline_principal = baseline.authenticate();
    let expanded_principal = expanded.authenticate();
    let baseline_runtime = baseline.runtime.application_runtime();
    let expanded_runtime = expanded.runtime.application_runtime();
    let baseline_capability = baseline_runtime
        .installed_schema()
        .capability(
            ViewEstateAdministrationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let expanded_capability = expanded_runtime
        .installed_schema()
        .capability(
            ViewEstateAdministrationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let baseline_access = baseline_runtime
        .admit_capability_access(
            baseline_principal.query(),
            &baseline_capability,
            view_action(),
            &request_scope(),
        )
        .unwrap();
    let expanded_access = expanded_runtime
        .admit_capability_access(
            expanded_principal.query(),
            &expanded_capability,
            view_action(),
            &request_scope(),
        )
        .unwrap();

    assert_eq!(
        baseline_access.relational_counters(),
        expanded_access.relational_counters()
    );
    assert_eq!(
        baseline_access.signal_dependency_count(),
        expanded_access.signal_dependency_count()
    );
    assert_eq!(
        baseline_runtime.capability_plan_compilation_evidence(),
        expanded_runtime.capability_plan_compilation_evidence()
    );
    let cold = baseline_runtime.capability_plan_compilation_evidence();
    assert!(cold.capability_count() > 1);
    assert!(cold.path_count() > 0);
    assert!(cold.rule_count() > 0);
    assert_eq!(cold.canonical_basis_preparations(), 0);
    assert_eq!(cold.digest_derivations(), 0);
    assert_eq!(cold.digest_text_materializations(), 0);
    assert_eq!(baseline_capability.lookup_evidence().registry_probes(), 1);
    assert_eq!(expanded_capability.lookup_evidence().registry_probes(), 1);
    for work in [
        baseline_access.admission_canonical_work(),
        expanded_access.admission_canonical_work(),
    ] {
        assert_eq!(work.basis_preparations(), 0);
        assert_eq!(work.digest_derivations(), 0);
        assert_eq!(work.canonical_encoded_bytes(), 0);
        assert_eq!(work.canonical_material_allocation_bytes(), 0);
        assert_eq!(work.sha256_input_bytes(), 0);
        assert_eq!(work.sha256_compression_blocks(), 0);
        assert_eq!(work.digest_text_materializations(), 0);
    }
}

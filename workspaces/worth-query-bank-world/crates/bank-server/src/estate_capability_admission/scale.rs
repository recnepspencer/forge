use super::{
    current_admission::identity_disclosure_action,
    fixture::{capability_world, request_scope, GrantSpec},
};
use bank_domain::{
    estate::EstateWorkflowStage,
    schema::{ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation},
};
use std::num::NonZeroUsize;
use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryControls;

use crate::application_query::execute_estate_customer_disclosure_with;

#[test]
fn unrelated_grant_population_does_not_enter_warm_capability_work() {
    let baseline = capability_world(
        "scale-baseline",
        GrantSpec::identity_verification(),
        EstateWorkflowStage::Administration,
        false,
        0,
    );
    let expanded = capability_world(
        "scale-expanded",
        GrantSpec::identity_verification(),
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
            ViewEstateIdentityVerificationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let expanded_capability = expanded_runtime
        .installed_schema()
        .capability(
            ViewEstateIdentityVerificationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .unwrap();
    let baseline_prepared = baseline_runtime
        .prepare_capability_access(
            baseline_principal.query(),
            &baseline_capability,
            identity_disclosure_action(),
            &request_scope(),
        )
        .unwrap();
    let expanded_prepared = expanded_runtime
        .prepare_capability_access(
            expanded_principal.query(),
            &expanded_capability,
            identity_disclosure_action(),
            &request_scope(),
        )
        .unwrap();
    let baseline_work = baseline_prepared.admission_canonical_work();
    let expanded_work = expanded_prepared.admission_canonical_work();
    drop(baseline_prepared);
    drop(expanded_prepared);
    let baseline_request = request_scope();
    let expanded_request = request_scope();
    let (_, baseline_authorization_work) = execute_estate_customer_disclosure_with(
        &baseline.runtime,
        &baseline_principal,
        super::fixture::ESTATE,
        controls(&baseline_request),
        |plan| plan.authorization_work(),
    )
    .unwrap();
    let (_, expanded_authorization_work) = execute_estate_customer_disclosure_with(
        &expanded.runtime,
        &expanded_principal,
        super::fixture::ESTATE,
        controls(&expanded_request),
        |plan| plan.authorization_work(),
    )
    .unwrap();

    assert_eq!(baseline_authorization_work, expanded_authorization_work);
    assert_eq!(baseline_authorization_work.requirement_count(), 2);
    assert!(baseline_authorization_work.paths_evaluated() > 0);
    assert!(baseline_authorization_work.signal_dependency_count() > 0);
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
    for work in [baseline_work, expanded_work] {
        assert_eq!(work.basis_preparations(), 0);
        assert_eq!(work.digest_derivations(), 0);
        assert_eq!(work.canonical_encoded_bytes(), 0);
        assert_eq!(work.canonical_material_allocation_bytes(), 0);
        assert_eq!(work.sha256_input_bytes(), 0);
        assert_eq!(work.sha256_compression_blocks(), 0);
        assert_eq!(work.digest_text_materializations(), 0);
    }
}

fn controls<'a>(
    request: &'a worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope,
) -> WorthQueryApplicationQueryControls<'a, bank_domain::schema::BankSchema> {
    WorthQueryApplicationQueryControls::current_one_shot(
        NonZeroUsize::new(1).unwrap(),
        NonZeroUsize::new(256).unwrap(),
        request,
    )
}

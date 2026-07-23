use worth_foundational::facade::FieldKey;
use worth_query::facade::{domain, foundation};

use super::installed_operation_fixture::{
    configured_runtime, GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};

#[test]
fn prior_generation_key_is_distinct_from_runtime_and_capability_drift() {
    let mut workspace = configured_runtime()
        .controlled_workspace("installed-native-key-generation")
        .unwrap();
    let prior_domain = workspace.domain(GeometryDomain).unwrap();
    let prior = bind(&workspace, &prior_domain);
    let (_prior_request, prior_key) = native_id_request(&prior);

    workspace.advance_domain_installation_generation().unwrap();
    let (current_domain, _) = workspace
        .rebind_domain(prior_domain.rebind_request())
        .unwrap()
        .into_parts();
    let current = bind(&workspace, &current_domain);
    let (current_request, current_key) = native_id_request(&current);
    let settled = current
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume_bound(current_request)
        .unwrap()
        .settle()
        .unwrap();

    assert!(settled.native_value(&current_key, 0).is_ok());
    let denial = settled.native_value(&prior_key, 0).unwrap_err();
    assert_eq!(
        denial.kind(),
        domain::WorthQueryNativeAccessDenialKind::AccessKeyInstallationGenerationMismatch
    );
    assert_eq!(denial.contract_identity(), prior_key.contract_identity());
    assert_eq!(denial.counters().authority_checks, 3);
    assert_eq!(denial.counters().indexed_accesses, 0);
    assert_eq!(denial.counters().refinement_checks, 0);
}

fn bind(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    domain: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(domain, ReadVertex)
        .unwrap()
}

fn native_id_request(
    bound: &domain::WorthQueryBoundDomainOperation<
        GeometryDomain,
        ReadVertex,
        ReadFamily,
        foundation::ObservationLaneWitness,
    >,
) -> (
    domain::WorthQueryBoundProjectionRequest<
        GeometryDomain,
        ReadVertex,
        ReadFamily,
        foundation::ObservationLaneWitness,
    >,
    domain::WorthQueryNativeAccessKey,
) {
    let mut builder = bound
        .consumer_projection_contract()
        .unwrap()
        .projection_request();
    let selection = builder
        .select_display_native_field(FieldKey::new("id").unwrap())
        .unwrap();
    let request = builder.build().unwrap();
    let key = request.resolve_native_key(&selection).unwrap().into_key();
    (request, key)
}

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}

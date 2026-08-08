use worth_query_declaration::facade::application_schema::ApplicationOperationProgramTarget;

use super::{
    WorthQueryCompiledApplicationOperationContracts,
    WorthQueryInstalledApplicationOperationAuthorization,
    WorthQueryInstalledApplicationOperationExecutionPosture, APPLICATION_AUTHORIZATION_FACT_FAMILY,
};
use crate::domain_operation::WorthQueryDecisionFactCardinality;

#[test]
fn activation_operation_compiles_its_union_from_selected_capability_targets() {
    let contract =
        crate::application_capability::tests::delegation_activation_fixture::activated_contract();
    let selected = worth_query_declaration::facade::application_capability::
        application_capability_delegation_activation_program_targets(&contract)
        .expect("selected activation target has derived effects");
    let members = vec![worth_query_declaration::facade::application_schema::
        ApplicationSchemaMember::ApplicationCapability { contract }];

    assert_eq!(
        super::contract_resolution::operation_program_from_members(
            &members,
            "Activation",
            std::any::type_name::<()>(),
        ),
        selected
    );
}

#[test]
fn delegation_activation_compiles_specialized_posture_support_and_digest_budget() {
    let contracts = compiled(
        WorthQueryInstalledApplicationOperationExecutionPosture::DelegationActivation,
        1,
    );

    assert_eq!(
        contracts.execution_posture(),
        WorthQueryInstalledApplicationOperationExecutionPosture::DelegationActivation
    );
    assert_eq!(
        contracts
            .decision_facts()
            .family(APPLICATION_AUTHORIZATION_FACT_FAMILY)
            .expect("capability command plus parent support require authorization facts")
            .cardinality(),
        WorthQueryDecisionFactCardinality::Exact(3)
    );
    let budget = contracts
        .delegation_activation_proposal_canonical_work_budget()
        .expect("delegation activation owns a proposal digest budget");
    assert_eq!(budget.maximum_entry_count(), 22);
    assert_eq!(budget.maximum_encoded_bytes(), 256 * 1_024);
}

#[test]
fn ordinary_operations_have_no_delegation_proposal_digest_allowance() {
    let contracts = compiled(
        WorthQueryInstalledApplicationOperationExecutionPosture::Ordinary,
        0,
    );
    assert_eq!(
        contracts.execution_posture(),
        WorthQueryInstalledApplicationOperationExecutionPosture::Ordinary
    );
    assert!(contracts
        .delegation_activation_proposal_canonical_work_budget()
        .is_none());
    assert!(contracts
        .capability_revocation_proposal_canonical_work_budget()
        .is_none());
}

#[test]
fn capability_revocation_compiles_specialized_posture_and_digest_budget() {
    let contracts = compiled(
        WorthQueryInstalledApplicationOperationExecutionPosture::CapabilityRevocation,
        0,
    );
    assert_eq!(
        contracts.execution_posture(),
        WorthQueryInstalledApplicationOperationExecutionPosture::CapabilityRevocation
    );
    let budget = contracts
        .capability_revocation_proposal_canonical_work_budget()
        .expect("revocation owns a private governed-target digest budget");
    assert_eq!(budget.maximum_entry_count(), 16);
    assert_eq!(budget.maximum_encoded_bytes(), 64 * 1_024);
}

fn compiled(
    posture: WorthQueryInstalledApplicationOperationExecutionPosture,
    support_count: usize,
) -> WorthQueryCompiledApplicationOperationContracts {
    WorthQueryCompiledApplicationOperationContracts::compile(
        super::contracts::WorthQueryApplicationOperationContractSources {
            authorization: WorthQueryInstalledApplicationOperationAuthorization::Capability,
            ability_requirements: Vec::new(),
            program: vec![ApplicationOperationProgramTarget::Create {
                entity: "Grant".to_owned(),
            }],
            decision_reads: Vec::new(),
            decision_fact_budget: 1,
            projection_work_budget: 16,
            additional_authorization_fact_count: support_count,
            mutation_preconditions: Vec::new(),
            execution_posture: posture,
            external_effect: crate::application_aftermath::InstalledExternalEffectContract::None,
            aftermath: None,
        },
    )
}

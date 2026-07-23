use worth_query::facade::{domain, foundation, runtime};

use super::installed_operation_fixture::{
    aftermath_workspace, AftermathCandidate, AftermathContract, AftermathFamily, AftermathOriginal,
    GeometryDomain,
};

pub(super) fn assert_posture_denial(
    contract: AftermathContract,
    expected_denial: domain::WorthQueryAftermathAdmissionDenial,
    expected_posture: domain::WorthQueryAftermathPosture,
) {
    let mut workspace = aftermath_workspace("aftermath-posture", contract).unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    match original.admit_aftermath(bind_candidate(&workspace, basis)) {
        domain::WorthQueryAftermathAdmission::Denied {
            denial,
            posture,
            counters,
        } => {
            assert_eq!(denial, expected_denial);
            assert_eq!(posture, expected_posture);
            assert_eq!(counters.effect_receipt_checks, 0);
            assert_eq!(counters.execution_contacts, 0);
        }
        _ => panic!("non-executable posture opened an aftermath capability"),
    }
}

pub(super) fn intent(input: &str) -> domain::WorthQueryNormalizedWorkflowIntent {
    domain::WorthQueryNormalizedWorkflowIntent::new(vec![
        domain::WorthQueryWorkflowIntentStage::new(
            "apply",
            domain::WorthQueryWorkflowIntentValue::EntityIdentity(input.into()),
        ),
    ])
    .unwrap()
}

pub(super) fn bind_original(
    workspace: &runtime::WorthQueryWorkspace,
    basis: foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    AftermathOriginal,
    AftermathFamily,
    foundation::MutationPreparationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .operating_world(basis)
        .family(AftermathFamily)
        .bind(&installed, AftermathOriginal)
        .unwrap()
}

pub(super) fn bind_candidate(
    workspace: &runtime::WorthQueryWorkspace,
    basis: foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    AftermathCandidate,
    AftermathFamily,
    foundation::MutationPreparationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .operating_world(basis)
        .family(AftermathFamily)
        .bind(&installed, AftermathCandidate)
        .unwrap()
}

pub(super) fn mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn branch_mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .branch_head("aftermath-foreign-basis", true)
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}

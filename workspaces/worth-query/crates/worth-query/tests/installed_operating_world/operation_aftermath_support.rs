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
    let original = bind_original(&workspace)
        .admit_workflow_resources(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    match original.admit_aftermath(bind_candidate(&workspace)) {
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
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    AftermathOriginal,
    AftermathFamily,
    foundation::MutationPreparationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .prepare_mutation_operating_world()
        .unwrap()
        .family(AftermathFamily)
        .bind(&installed, AftermathOriginal)
        .unwrap()
}

pub(super) fn bind_candidate(
    workspace: &runtime::WorthQueryWorkspace,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    AftermathCandidate,
    AftermathFamily,
    foundation::MutationPreparationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .prepare_mutation_operating_world()
        .unwrap()
        .family(AftermathFamily)
        .bind(&installed, AftermathCandidate)
        .unwrap()
}

pub(super) fn bind_branch_candidate(
    workspace: &runtime::WorthQueryWorkspace,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    AftermathCandidate,
    AftermathFamily,
    foundation::MutationPreparationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .prepare_branch_mutation_operating_world(
            worth_query::facade::installed::WorthQueryBranchHeadIdentity::new(
                "aftermath-foreign-basis",
            )
            .unwrap(),
        )
        .unwrap()
        .family(AftermathFamily)
        .bind(&installed, AftermathCandidate)
        .unwrap()
}

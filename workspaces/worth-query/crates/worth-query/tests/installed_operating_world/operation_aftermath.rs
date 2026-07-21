use worth_foundational::FoundationalBoundaryEvidenceContinuityAttachmentScope;
use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, runtime};

use super::installed_operation_fixture::{
    aftermath_workspace, provisional_workflow_workspace, AftermathCandidate, AftermathContract,
    AftermathFamily, AftermathOriginal, GeometryDomain, ProvisionalWorkflow,
};

#[test]
fn compensation_is_a_new_bound_operation_with_a_proof_carrying_relation() {
    let mut workspace =
        aftermath_workspace("aftermath-compensation", AftermathContract::Compensation).unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let original_identity = original.identity().to_owned();
    let candidate = bind_candidate(&workspace, basis);
    let capability = match original.admit_aftermath(candidate) {
        domain::WorthQueryAftermathAdmission::Compensation(capability) => capability,
        _ => panic!("installed compensation was not admitted"),
    };

    let executed = capability.execute_workflow(&mut workspace).unwrap();
    let relation = executed.relation();
    assert_eq!(relation.original_trace_identity(), original_identity);
    assert_ne!(relation.aftermath_execution_identity(), original_identity);
    assert_eq!(
        relation.kind(),
        domain::WorthQueryAftermathKind::Compensation
    );
    assert_eq!(
        relation.postcondition(),
        &domain::WorthQueryAftermathPostcondition::BusinessPostcondition {
            identity: "original-obligation-settled".into(),
        }
    );
    assert!(!relation.authority_identity().is_empty());
    assert!(!relation.basis_identity().is_empty());
    assert!(!relation.original_operation_identity().is_empty());
    assert!(!relation.aftermath_operation_identity().is_empty());
    assert_ne!(
        relation.original_binding_identity(),
        relation.aftermath_binding_identity()
    );
    assert_ne!(
        relation.original_capability_identity(),
        relation.aftermath_capability_identity()
    );
    assert_eq!(relation.effect_receipt_identities().len(), 1);
    assert_eq!(relation.counters().execution_contacts, 1);
    assert_eq!(relation.counters().unrelated_trace_scans, 0);
    assert_eq!(
        relation
            .foundational_attachment()
            .materialized()
            .continuity_scope(),
        None
    );
    assert_eq!(
        relation
            .foundational_attachment()
            .admit_current_basis()
            .payload()
            .target(),
        relation.foundational_attachment().materialized().target()
    );
}

#[test]
fn exact_inverse_has_its_own_typed_surface_and_exact_postcondition() {
    let mut workspace =
        aftermath_workspace("aftermath-exact", AftermathContract::ExactInverse).unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let original_entity = original.stage_receipts()[0].effect_evidence()[0]
        .mutation_receipt()
        .unwrap()
        .target_entity_identity()
        .unwrap()
        .relational_entity_record_parts()
        .unwrap();
    let candidate = bind_candidate(&workspace, basis);
    let capability = match original.admit_aftermath(candidate) {
        domain::WorthQueryAftermathAdmission::ExactInverse(capability) => capability,
        _ => panic!("installed exact inverse was not admitted"),
    };
    let executed = capability.execute_workflow(&mut workspace).unwrap();
    assert_eq!(
        executed.relation().postcondition(),
        &domain::WorthQueryAftermathPostcondition::ExactPriorTruth
    );
    assert_eq!(
        executed.relation().kind(),
        domain::WorthQueryAftermathKind::ExactInverse
    );
    assert_eq!(
        executed
            .relation()
            .foundational_attachment()
            .materialized()
            .continuity_scope(),
        Some(FoundationalBoundaryEvidenceContinuityAttachmentScope::ObjectLevel)
    );
    assert_eq!(
        executed
            .relation()
            .counters()
            .candidate_effect_receipt_checks,
        1
    );
    assert_eq!(
        executed
            .relation()
            .counters()
            .postcondition_verification_checks,
        1
    );
    let receipt = executed.trace().stage_receipts()[0].effect_evidence()[0]
        .mutation_receipt()
        .unwrap();
    assert_eq!(
        receipt
            .target_entity_identity()
            .unwrap()
            .relational_entity_record_parts(),
        Some(original_entity)
    );
}

#[test]
fn aftermath_admission_rejects_foreign_runtime_and_wrong_operation() {
    let mut workspace =
        aftermath_workspace("aftermath-scope", AftermathContract::Compensation).unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let foreign =
        aftermath_workspace("aftermath-foreign", AftermathContract::Compensation).unwrap();
    let foreign_candidate = bind_candidate(&foreign, mutation_basis());
    assert!(matches!(
        original.admit_aftermath(foreign_candidate),
        domain::WorthQueryAftermathAdmission::Denied {
            denial: domain::WorthQueryAftermathAdmissionDenial::ForeignRuntime,
            ..
        }
    ));

    let mismatched_basis_candidate = bind_candidate(&workspace, branch_mutation_basis());
    assert!(matches!(
        original.admit_aftermath(mismatched_basis_candidate),
        domain::WorthQueryAftermathAdmission::Denied {
            denial: domain::WorthQueryAftermathAdmissionDenial::BasisMismatch,
            ..
        }
    ));

    let wrong_candidate = bind_original(&workspace, basis);
    assert!(matches!(
        original.admit_aftermath(wrong_candidate),
        domain::WorthQueryAftermathAdmission::Denied {
            denial: domain::WorthQueryAftermathAdmissionDenial::CandidateOperationMismatch,
            counters,
            ..
        } if counters.execution_contacts == 0
    ));
}

#[test]
fn non_executable_aftermath_postures_remain_distinct() {
    assert_posture_denial(
        AftermathContract::Irreversible,
        domain::WorthQueryAftermathAdmissionDenial::Irreversible,
        domain::WorthQueryAftermathPosture::Irreversible,
    );
    assert_posture_denial(
        AftermathContract::IncompleteCompensation,
        domain::WorthQueryAftermathAdmissionDenial::DeclarationIncomplete,
        domain::WorthQueryAftermathPosture::DeclarationIncomplete,
    );
    assert_posture_denial(
        AftermathContract::RebuildRequired,
        domain::WorthQueryAftermathAdmissionDenial::RebuildRequired,
        domain::WorthQueryAftermathPosture::RebuildRequired {
            recovery_family: "geometry-rebuild-v1".into(),
        },
    );
}

#[test]
fn false_business_postcondition_cannot_mint_an_aftermath_relation() {
    let mut workspace = aftermath_workspace(
        "aftermath-false-postcondition",
        AftermathContract::FalsePostcondition,
    )
    .unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let capability = match original.admit_aftermath(bind_candidate(&workspace, basis)) {
        domain::WorthQueryAftermathAdmission::Compensation(capability) => capability,
        _ => panic!("installed compensation was not admitted"),
    };
    let TransitionOutcome::Failed(domain::WorthQueryWorkflowReexecutionStop::Aftermath(denial)) =
        capability.execute_workflow(&mut workspace)
    else {
        panic!("false postcondition minted aftermath authority");
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryAftermathExecutionDenialKind::PostconditionNotEstablished
    );
    assert!(denial.candidate_trace_identity().is_some());
    assert_eq!(denial.partial_effects().len(), 1);
    assert_eq!(
        denial.recovery_posture(),
        Some(
            domain::WorthQueryAftermathFailureRecoveryPosture::DomainRecoveryRequired {
                attempted_kind: domain::WorthQueryAftermathKind::Compensation,
            }
        )
    );
}

#[test]
fn candidate_failure_after_effect_retains_original_stop_and_recovery_truth() {
    let mut workspace = aftermath_workspace(
        "aftermath-candidate-failure",
        AftermathContract::CandidateFailureAfterEffect,
    )
    .unwrap();
    let basis = mutation_basis();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let capability = match original.admit_aftermath(bind_candidate(&workspace, basis)) {
        domain::WorthQueryAftermathAdmission::Compensation(capability) => capability,
        _ => panic!("installed compensation was not admitted"),
    };
    let TransitionOutcome::Failed(domain::WorthQueryWorkflowReexecutionStop::Aftermath(denial)) =
        capability.execute_workflow(&mut workspace)
    else {
        panic!("candidate failure lost aftermath recovery truth");
    };

    assert_eq!(
        denial.kind(),
        domain::WorthQueryAftermathExecutionDenialKind::CandidateExecutionFailed
    );
    assert_eq!(denial.partial_effects().len(), 1);
    assert!(matches!(
        denial.candidate_execution_stop(),
        Some(domain::WorthQueryWorkflowReexecutionStop::Advance(_))
    ));
    assert_eq!(
        denial.recovery_posture(),
        Some(
            domain::WorthQueryAftermathFailureRecoveryPosture::DomainRecoveryRequired {
                attempted_kind: domain::WorthQueryAftermathKind::Compensation,
            }
        )
    );
}

#[test]
fn exact_inverse_on_a_different_target_retains_effect_truth_but_mints_no_relation() {
    let mut workspace = aftermath_workspace(
        "aftermath-wrong-inverse-target",
        AftermathContract::WrongInverseTarget,
    )
    .unwrap();
    let basis = mutation_basis();
    let _decoy = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let original = bind_original(&workspace, basis.clone())
        .reexecute(intent("apply"), &mut workspace)
        .unwrap();
    let capability = match original.admit_aftermath(bind_candidate(&workspace, basis)) {
        domain::WorthQueryAftermathAdmission::ExactInverse(capability) => capability,
        _ => panic!("installed exact inverse was not admitted"),
    };
    let TransitionOutcome::Failed(domain::WorthQueryWorkflowReexecutionStop::Aftermath(denial)) =
        capability.execute_workflow(&mut workspace)
    else {
        panic!("wrong-target inverse minted aftermath authority");
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryAftermathExecutionDenialKind::ExactInverseScopeMismatch
    );
    assert!(denial.candidate_trace_identity().is_some());
    assert_eq!(denial.partial_effects().len(), 1);
    assert!(denial.partial_effects()[0].mutation_receipt().is_some());
    assert_eq!(
        denial.recovery_posture(),
        Some(
            domain::WorthQueryAftermathFailureRecoveryPosture::DomainRecoveryRequired {
                attempted_kind: domain::WorthQueryAftermathKind::ExactInverse,
            }
        )
    );
}

#[test]
fn provisional_discard_consumes_only_an_effect_free_provisional_trace() {
    let mut workspace = provisional_workflow_workspace("provisional-discard").unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let trace = workspace
        .operating_world(mutation_basis())
        .family(AftermathFamily)
        .bind(&installed, ProvisionalWorkflow)
        .unwrap()
        .reexecute(intent("discard"), &mut workspace)
        .unwrap();
    let trace_identity = trace.identity().to_owned();
    assert_eq!(
        trace.aftermath_posture(),
        domain::WorthQueryAftermathPosture::ProvisionalDiscard
    );
    let discarded = trace.discard_provisional().unwrap();
    assert_eq!(discarded.original_trace_identity(), trace_identity);
    assert_ne!(discarded.identity(), trace_identity);
}

fn assert_posture_denial(
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

fn intent(input: &str) -> domain::WorthQueryNormalizedWorkflowIntent {
    domain::WorthQueryNormalizedWorkflowIntent::new(vec![
        domain::WorthQueryWorkflowIntentStage::new(
            "apply",
            domain::WorthQueryWorkflowIntentValue::EntityIdentity(input.into()),
        ),
    ])
    .unwrap()
}

fn bind_original(
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

fn bind_candidate(
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

fn mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}

fn branch_mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .branch_head("aftermath-foreign-basis", true)
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}

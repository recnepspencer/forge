use super::*;

#[test]
fn controller_rejects_incoherent_process_contract_projections() {
    let valid = declaration(
        ProcessRoleProjection::OfflineVerifier,
        ProcessIsolationProjection::IndependentObserver,
        ProcessTerminationRequirementProjection::GracefulExit,
    );
    let invalid_role = DeclarationProjection {
        role: ProcessRoleProjection::Writer,
        ..valid.clone()
    };
    let invalid_termination = DeclarationProjection {
        required_termination: ProcessTerminationRequirementProjection::ParentKill,
        ..valid.clone()
    };

    assert!(process_contract_matches(&valid));
    assert!(!process_contract_matches(&invalid_role));
    assert!(!process_contract_matches(&invalid_termination));
}

fn declaration(
    role: ProcessRoleProjection,
    isolation: ProcessIsolationProjection,
    required_termination: ProcessTerminationRequirementProjection,
) -> DeclarationProjection {
    DeclarationProjection {
        scenario_identity: "projection".to_owned(),
        role,
        isolation,
        required_termination,
        input_identity: [1; 32],
        executable_identity: [2; 32],
        working_directory: "workspace".to_owned(),
        environment_identity: [3; 32],
        declaration_identity: [4; 32],
    }
}

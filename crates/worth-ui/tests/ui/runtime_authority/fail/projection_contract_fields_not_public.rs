use worth_ui::facade::{
    WorthUiAdmittedProjectionPlan, WorthUiHeaderMenuPlan, WorthUiProjectionDependencySet,
    WorthUiProjectionDependencyValidationProof, WorthUiProjectionEquivalenceBasis,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanProof, WorthUiRuntimeInstanceWitness,
    WorthUiValidatedProjectionDependencyContract,
};

fn main() {
    let _dependency_proof = WorthUiProjectionDependencyValidationProof {
        dependency_digest: 0,
    };
    let _validated = WorthUiValidatedProjectionDependencyContract {
        identity: identity(),
        family: family(),
        dependencies: WorthUiProjectionDependencySet::empty(),
        validation_proof: dependency_proof(),
    };
    let _basis = WorthUiProjectionEquivalenceBasis {
        identity: identity(),
        family: family(),
        kind: basis_kind(),
        value: 0,
    };
    let _plan_proof = WorthUiProjectionPlanProof {
        runtime_instance: runtime_instance(),
        dependency_digest: 0,
        equivalence_digest: 0,
    };
    let _admitted = WorthUiAdmittedProjectionPlan::<WorthUiHeaderMenuPlan> {
        runtime_instance: runtime_instance(),
        plan: header_menu_plan(),
        dependencies: validated_contract(),
        equivalence_basis: equivalence_basis(),
        proof: plan_proof(),
    };
}

fn identity() -> WorthUiProjectionIdentity {
    panic!("fixture only")
}

fn family() -> WorthUiProjectionFamily {
    WorthUiProjectionFamily::HeaderMenu
}

fn basis_kind() -> WorthUiProjectionEquivalenceBasisKind {
    WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
}

fn dependency_proof() -> WorthUiProjectionDependencyValidationProof {
    panic!("fixture only")
}

fn header_menu_plan() -> WorthUiHeaderMenuPlan {
    panic!("fixture only")
}

fn validated_contract() -> WorthUiValidatedProjectionDependencyContract {
    panic!("fixture only")
}

fn equivalence_basis() -> WorthUiProjectionEquivalenceBasis {
    panic!("fixture only")
}

fn plan_proof() -> WorthUiProjectionPlanProof {
    panic!("fixture only")
}

fn runtime_instance() -> WorthUiRuntimeInstanceWitness {
    panic!("fixture only")
}

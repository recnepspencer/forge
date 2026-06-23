use super::{
    WorthUiProjectionDependencyAdmissionDenial, WorthUiProjectionEquivalenceBasis,
    WorthUiProjectionPlanContract, WorthUiValidatedProjectionDependencyContract,
};
use crate::runtime::WorthUiRuntimeInstanceWitness;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionPlanProof {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    dependency_digest: u64,
    equivalence_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedProjectionPlan<P: WorthUiProjectionPlanContract> {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    plan: P,
    dependencies: WorthUiValidatedProjectionDependencyContract,
    equivalence_basis: WorthUiProjectionEquivalenceBasis,
    proof: WorthUiProjectionPlanProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionPlanAdmissionDenial {
    Dependency(WorthUiProjectionDependencyAdmissionDenial),
}

impl<P: WorthUiProjectionPlanContract> WorthUiAdmittedProjectionPlan<P> {
    pub(crate) fn admit(
        plan: P,
        runtime_instance: WorthUiRuntimeInstanceWitness,
    ) -> Result<Self, WorthUiProjectionPlanAdmissionDenial> {
        let identity = plan.projection_identity();
        let family = plan.projection_family();
        let dependencies = WorthUiValidatedProjectionDependencyContract::admit(
            identity.clone(),
            family,
            plan.projection_dependency_declaration(),
        )
        .map_err(WorthUiProjectionPlanAdmissionDenial::Dependency)?;
        let equivalence_basis = WorthUiProjectionEquivalenceBasis::new(
            identity,
            family,
            plan.projection_equivalence_basis_kind(),
            plan.projection_equivalence_digest(),
        );
        let proof = WorthUiProjectionPlanProof {
            runtime_instance,
            dependency_digest: dependencies.validation_proof().dependency_digest(),
            equivalence_digest: equivalence_basis.digest(),
        };
        Ok(Self {
            runtime_instance,
            plan,
            dependencies,
            equivalence_basis,
            proof,
        })
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn plan(&self) -> &P {
        &self.plan
    }

    pub fn dependencies(&self) -> &WorthUiValidatedProjectionDependencyContract {
        &self.dependencies
    }

    pub fn equivalence_basis(&self) -> &WorthUiProjectionEquivalenceBasis {
        &self.equivalence_basis
    }

    pub fn proof(&self) -> WorthUiProjectionPlanProof {
        self.proof
    }
}

impl WorthUiProjectionPlanProof {
    pub fn runtime_instance(self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn dependency_digest(self) -> u64 {
        self.dependency_digest
    }

    pub fn equivalence_digest(self) -> u64 {
        self.equivalence_digest
    }
}

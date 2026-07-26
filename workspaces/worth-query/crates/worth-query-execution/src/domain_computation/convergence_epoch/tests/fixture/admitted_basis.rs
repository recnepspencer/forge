use worth_query_admission::facade::basis::{
    admit_basis_capability, evaluate_basis_observation_eligibility, normalize_raw_basis_intent,
    AdmittedBasisCapability, BasisOperationLane, ObservationLaneWitness, RawBasisIntent,
};

pub(super) fn admitted_basis() -> AdmittedBasisCapability<ObservationLaneWitness> {
    admit(RawBasisIntent::CurrentHead)
}

pub(super) fn admitted_alternate_basis() -> AdmittedBasisCapability<ObservationLaneWitness> {
    admit(RawBasisIntent::BranchHead {
        branch_identity: "alternate-convergence-basis".into(),
        accessible: true,
    })
}

fn admit(raw: RawBasisIntent) -> AdmittedBasisCapability<ObservationLaneWitness> {
    let normalized = normalize_raw_basis_intent(raw, ObservationLaneWitness::lane_name())
        .expect("fixture basis must normalize");
    admit_basis_capability(
        evaluate_basis_observation_eligibility(normalized)
            .expect("fixture basis must be observation eligible"),
    )
}

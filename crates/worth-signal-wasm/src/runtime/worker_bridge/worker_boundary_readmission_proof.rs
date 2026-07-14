use worth_proof::{
    Admitted, AssumptionBasis, BoundaryBridgedAuthorityRevalidationRequiredBasis, Recipe,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerBoundaryReadmissionBasis {
    envelope_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryBridgedWorkerEnvelope {
    envelope_family: &'static str,
}

pub type BoundaryBridgedWorkerEnvelopeReadmissionProof = Recipe<
    Admitted,
    BoundaryBridgedWorkerEnvelope,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<
        AssumptionBasis<WorkerBoundaryReadmissionBasis>,
    >,
>;

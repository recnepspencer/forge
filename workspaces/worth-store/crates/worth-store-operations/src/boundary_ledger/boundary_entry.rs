#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalBoundaryDirection {
    OwnerToOperations,
    OwnerToOwner,
    OperationsToOwner,
    ExternalObservationToVerifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalCostClass {
    Constant,
    BoundedStreaming,
    Reconstructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalProofLane {
    OwnerReceipt,
    UntrustedObservation,
    LoweredOwnerPlan,
    SupportProjection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalRecoveryBoundaryEntry {
    pub artifact: &'static str,
    pub authority_owner: &'static str,
    pub observation_owner: &'static str,
    pub mutation_owner: &'static str,
    pub consumer: &'static str,
    pub direction: OperationalBoundaryDirection,
    pub construction_authority: &'static str,
    pub cost_class: OperationalCostClass,
    pub failure_topology: &'static str,
    pub proof_lane: OperationalProofLane,
}

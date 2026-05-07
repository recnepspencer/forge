use forge_proof::{AssumptionBasis, CurrentValidity, FreshnessScopedBasis, Lowered, Recipe};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementLoweringBasis {
    placement_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredWorkerExecutionPlan {
    declaration_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredMainThreadHostedExecutionPlan {
    declaration_identity: String,
}

pub type LoweredWorkerExecutionPlanProof = Recipe<
    Lowered,
    LoweredWorkerExecutionPlan,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<PlacementLoweringBasis>>,
>;

pub type LoweredMainThreadHostedExecutionPlanProof = Recipe<
    Lowered,
    LoweredMainThreadHostedExecutionPlan,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<PlacementLoweringBasis>>,
>;

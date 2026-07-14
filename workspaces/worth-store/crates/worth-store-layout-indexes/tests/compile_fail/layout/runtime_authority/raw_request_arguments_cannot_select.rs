use worth_store_budgets::S8PreExecutionBudgetEnvelope;
use worth_store_layout_indexes::{
    AccessPlanSelector, AccessShapeContract, ArtifactFamilyLifecycleAdmission,
    ConcretePhysicalKeyWitness,
};

fn select_raw(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key: ConcretePhysicalKeyWitness,
    shape: AccessShapeContract,
    budget: S8PreExecutionBudgetEnvelope,
) {
    let _ = AccessPlanSelector.select_concrete_with_budget(lifecycle, key, shape, budget);
}

fn main() {}

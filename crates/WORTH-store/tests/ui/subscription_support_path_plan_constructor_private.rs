use worth_store::{
    SupportActionBreadthBudget, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass, SupportProgramPathPlan,
};

fn main() {
    let budget = SupportActionBreadthBudget::new(1, 64).unwrap();
    let _ = SupportProgramPathPlan::new(
        SupportPathClass::OperationalPlanning,
        SupportProgramDensityClass::FamilyLocalBatch,
        SupportAllocationScope::FamilyLocalBatch,
        budget,
        1,
        32,
    );
}

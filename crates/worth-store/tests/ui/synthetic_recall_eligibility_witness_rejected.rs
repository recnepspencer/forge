use worth_store::{RecallAmplificationBudget, RecallCostClass, RecallEligibilityWitness};

fn main() {
    let _ = RecallEligibilityWitness::new(
        "snapshot:42",
        RecallCostClass::Bounded,
        RecallAmplificationBudget::SingleFamilyLocalUnit,
    );
}

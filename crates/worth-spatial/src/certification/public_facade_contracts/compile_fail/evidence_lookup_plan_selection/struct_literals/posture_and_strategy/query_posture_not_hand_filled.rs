use worth_spatial::facade::evidence_lookup_plan_selection::{
    EvidenceLookupPlanQueryPosture, EvidenceLookupPlanQueryPostureState,
};

fn main() {
    let _ = EvidenceLookupPlanQueryPosture {
        state: EvidenceLookupPlanQueryPostureState::NotRequired,
    };
}

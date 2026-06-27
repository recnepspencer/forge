use worth_spatial::facade::evidence_lookup_plan_selection::{
    EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanTopologyPostureState,
};

fn main() {
    let _ = EvidenceLookupPlanTopologyPosture {
        state: EvidenceLookupPlanTopologyPostureState::NotRequired,
    };
}

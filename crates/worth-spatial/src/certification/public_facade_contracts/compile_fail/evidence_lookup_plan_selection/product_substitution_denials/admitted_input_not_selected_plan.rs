use worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupAdmittedInput;
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

fn main() {
    fn admitted_input_is_not_selected_plan(admitted: EvidenceLookupAdmittedInput) {
        let _: EvidenceLookupSelectedPlan = admitted;
    }
}

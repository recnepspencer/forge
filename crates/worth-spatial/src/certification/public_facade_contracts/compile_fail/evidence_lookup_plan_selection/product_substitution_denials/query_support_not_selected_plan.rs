use worth_spatial::facade::evidence_lookup_input_admission::EvidenceLookupQueryAdmissionSupport;
use worth_spatial::facade::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

fn main() {
    fn query_support_is_not_selected_plan(query: EvidenceLookupQueryAdmissionSupport) {
        let _: EvidenceLookupSelectedPlan = query;
    }
}

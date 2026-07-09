use worth_query::facade::runtime::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionBoundary,
    WorthQueryIntentAdmissionFamily, WorthQueryIntentAdmissionSupportPosture,
    WorthQueryIntentAdmissionSupportDetail, WorthQueryIntentAdmissionSupportRow,
};

fn main() {
    let _ = WorthQueryIntentAdmissionSupportRow {
        family: WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        posture: WorthQueryIntentAdmissionSupportPosture::Admitted,
        execution_boundary: WorthQueryIntentAdmissionExecutionBoundary::deferred_neighbor("Worthd"),
        detail: WorthQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    };
}

use forge_query::facade::runtime::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionBoundary,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdmissionSupportPosture,
    ForgeQueryIntentAdmissionSupportDetail, ForgeQueryIntentAdmissionSupportRow,
};

fn main() {
    let _ = ForgeQueryIntentAdmissionSupportRow {
        family: ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        posture: ForgeQueryIntentAdmissionSupportPosture::Admitted,
        execution_boundary: ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor("forged"),
        detail: ForgeQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    };
}

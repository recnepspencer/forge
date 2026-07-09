use worth_query::facade::runtime::{
    WorthQueryIntentAdmissionCoverageRow, WorthQueryIntentAdmissionCoverageStatus,
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionDecisionClass,
    WorthQueryIntentAdmissionEligibilityAuthority, WorthQueryIntentAdmissionExecutionBoundary,
    WorthQueryIntentAdmissionExecutionHandoffInventory, WorthQueryIntentAdmissionFamily,
    WorthQueryIntentAdmissionPlanKind, WorthQueryIntentAdmissionResultArtifact,
    WorthQueryIntentAdmissionSurfaceDescriptor,
};

fn main() {
    let _ = WorthQueryIntentAdmissionCoverageRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        WorthQueryIntentAdmissionCoverageStatus::Implemented,
        WorthQueryIntentAdmissionEligibilityAuthority::RuntimeIntentAuthorityAdapter,
        WorthQueryIntentAdmissionPlanKind::AuthoritativeIntentExecutionPlan,
        WorthQueryIntentAdmissionExecutionHandoffInventory::available(
            "WorthQueryAdmittedIntentExecutionHandoff",
        ),
        WorthQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint,
        WorthQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation,
        WorthQueryIntentAdmissionResultArtifact::WorthQueryIntentReceipt,
        WorthQueryIntentAdmissionSurfaceDescriptor::available("Worthd"),
        WorthQueryIntentAdmissionSurfaceDescriptor::available("Worthd"),
        WorthQueryIntentAdmissionSurfaceDescriptor::available("Worthd"),
    );
}

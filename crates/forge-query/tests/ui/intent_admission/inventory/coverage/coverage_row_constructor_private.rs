use forge_query::facade::runtime::{
    ForgeQueryIntentAdmissionCoverageRow, ForgeQueryIntentAdmissionCoverageStatus,
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecisionClass,
    ForgeQueryIntentAdmissionEligibilityAuthority, ForgeQueryIntentAdmissionExecutionBoundary,
    ForgeQueryIntentAdmissionExecutionHandoffInventory, ForgeQueryIntentAdmissionFamily,
    ForgeQueryIntentAdmissionPlanKind, ForgeQueryIntentAdmissionResultArtifact,
    ForgeQueryIntentAdmissionSurfaceDescriptor,
};

fn main() {
    let _ = ForgeQueryIntentAdmissionCoverageRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        ForgeQueryIntentAdmissionCoverageStatus::Implemented,
        ForgeQueryIntentAdmissionEligibilityAuthority::RuntimeIntentAuthorityAdapter,
        ForgeQueryIntentAdmissionPlanKind::AuthoritativeIntentExecutionPlan,
        ForgeQueryIntentAdmissionExecutionHandoffInventory::available(
            "ForgeQueryAdmittedIntentExecutionHandoff",
        ),
        ForgeQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint,
        ForgeQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation,
        ForgeQueryIntentAdmissionResultArtifact::ForgeQueryIntentReceipt,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available("forged"),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available("forged"),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available("forged"),
    );
}

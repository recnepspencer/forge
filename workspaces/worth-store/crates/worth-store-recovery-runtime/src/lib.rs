#![forbid(unsafe_code)]

mod entry;
mod handoff;
mod orchestration;
mod progression;

pub use entry::{
    PhysicalManifestObservationDenial, PhysicalRecoveryAdmissionCounters, PhysicalRecoveryBlock,
    PhysicalRecoveryBlockEvidence, PhysicalRecoveryBlockKind, PhysicalRecoveryEntryBindingDrift,
    PhysicalRecoveryLimitDeclaration, PhysicalRecoveryLimitDenial, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimitFailure, PhysicalRecoveryLimits, PhysicalRecoveryMediaObservationFailure,
    PhysicalRecoveryOpenRequest, PhysicalRecoveryOutcome, PhysicalRecoveryPageAdmissionDenial,
    PhysicalRecoveryPlanningDenial, PhysicalRecoveryPlatformAdmissionError,
    PhysicalRecoveryPlatformAuthority, PhysicalRecoveryPublicationCounters,
    PhysicalRecoveryPublicationDenial, PhysicalRecoveryPublicationIndeterminate,
    PhysicalRecoveryPublicationSettlement, PhysicalRecoveryPublicationSettlementLedger,
    PhysicalRecoveryRefusal, PhysicalRecoveryRefusalKind, PhysicalRecoveryReopenCounters,
    PhysicalRecoveryReopenFailure, PhysicalRecoverySessionIdentity, PhysicalRecoverySourceDenial,
    PhysicalRecoveryStagingCounters, PhysicalRecoveryStagingDenial,
    PhysicalRecoveryStagingSettlement, PhysicalRecoveryStagingSettlementLedger,
    PhysicalRecoveryStaticConfiguration,
};
pub use handoff::{RecoveredPhysicalRuntimeHandoff, RecoveryOperationFateSet};
pub use progression::{
    AdmittedPhysicalRecovery, ClosedRecoveryStagingGeneration, DiscoveredPhysicalRecovery,
    NamespaceDurablePhysicalRecovery, PhysicalRecoveryDiscoveryCounters,
    PhysicalRecoveryStagingCancellation, PlannedPhysicalRecovery, RecoveryBaseImageAction,
    RecoveryBaseImagePlan, RecoveryPayloadManifestAction, RecoveryPublicationAction,
    RecoveryPublicationCandidateArtifact, RecoveryPublicationExpectation, RecoveryPublicationPlan,
    RecoveryQuiescencePlan, RecoverySegmentRoutingAction, RecoveryStagingAction,
    RecoveryStagingCommandPlan, RecoveryStagingLayoutPlan, RecoveryStagingRedoStep,
    ReopenedPhysicalRecovery, SelectedPhysicalRecovery, StagedPhysicalRecovery,
};

/// The single production composition facade for one fresh-process physical
/// recovery attempt.
pub struct WorthStoreRecovery {
    _private: (),
}

impl WorthStoreRecovery {
    pub fn recover(request: PhysicalRecoveryOpenRequest) -> PhysicalRecoveryOutcome {
        let admitted = match request.admit() {
            Ok(admitted) => admitted,
            Err(refusal) => return PhysicalRecoveryOutcome::Refused(refusal),
        };
        let discovered = match admitted.discover() {
            Ok(discovered) => discovered,
            Err(outcome) => return outcome,
        };
        let selected = match discovered.select() {
            Ok(selected) => selected,
            Err(outcome) => return outcome,
        };
        let planned = match selected.plan() {
            Ok(planned) => planned,
            Err(outcome) => return outcome,
        };
        let staged = match planned.stage() {
            Ok(staged) => staged,
            Err(outcome) => return outcome,
        };
        let published = match staged.publish() {
            Ok(published) => published,
            Err(outcome) => return outcome,
        };
        match published.reopen() {
            Ok(reopened) => reopened.finish(),
            Err(outcome) => outcome,
        }
    }
}

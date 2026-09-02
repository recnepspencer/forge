use worth_runtime_world::facade::RuntimeWorldPublicationPhase;

#[test]
fn publication_phase_vocabulary_is_complete_for_diagnostics() {
    // Phase 1 intentionally fences only the public diagnostic vocabulary.
    // Real consuming token progression belongs to the later owner lanes.
    let phases = [
        RuntimeWorldPublicationPhase::ProductBranchIntent,
        RuntimeWorldPublicationPhase::ResolvedExpectedProductHead,
        RuntimeWorldPublicationPhase::AdmittedCompositeRuntimeWorldBasis,
        RuntimeWorldPublicationPhase::LoweredOwnerComponentPlan,
        RuntimeWorldPublicationPhase::ReservedCompositePublicationAttempt,
        RuntimeWorldPublicationPhase::OwnerExecutionSettlement,
        RuntimeWorldPublicationPhase::CompositePublicationReady,
        RuntimeWorldPublicationPhase::RuntimeWorldPublicationOutcome,
    ];

    for (index, phase) in phases.iter().enumerate() {
        assert!(phases[..index].iter().all(|prior| prior != phase));
    }
}

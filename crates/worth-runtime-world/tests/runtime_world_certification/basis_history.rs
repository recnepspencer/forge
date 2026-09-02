use worth_runtime_world::facade::RuntimeWorldPublicationPhase;

#[test]
fn publication_phase_vocabulary_is_complete_for_diagnostics() {
    // The real consuming token chain is exercised by the owner-side
    // reservation/cancellation test. This external lane hook protects only
    // the public diagnostic vocabulary; enum adjacency is not progression
    // evidence.
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

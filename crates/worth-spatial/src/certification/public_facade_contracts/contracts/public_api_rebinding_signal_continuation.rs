use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryDeclarationEnvelope,
    ForgeQuerySignalCompatibilityOrchestrationInput,
};
use worth_spatial::facade::bindings::{
    PrimitiveRebindingDeclarationEntry, PrimitiveRebindingQueryDomain, PrimitiveRebindingQueryWorld,
};
use worth_spatial::facade::continuation::{
    primitive_rebinding_continuation_target, primitive_rebinding_signal_workflow,
    PrimitiveRebindingContinuationExecution, PrimitiveRebindingContinuationExecutionChecked,
    PrimitiveRebindingContinuationExecutionOutcome, PrimitiveRebindingContinuationExecutionProof,
    PrimitiveRebindingContinuationTarget, PrimitiveRebindingPreparedContinuation,
    PrimitiveRebindingPreparedContinuationChecked, PrimitiveRebindingPreparedContinuationOutcome,
    PrimitiveRebindingPreparedContinuationProof, PrimitiveRebindingSignalCompatibilityArtifact,
    PrimitiveRebindingSignalCompatibilityChecked, PrimitiveRebindingSignalCompatibilityInput,
    PrimitiveRebindingSignalCompatibilityOutcome, PrimitiveRebindingSignalCompatibilityProof,
    PrimitiveRebindingSignalCompatibilitySubject,
};

#[test]
fn spatial_public_rebinding_signal_and_continuation_surface_exports_typed_aliases() {
    let _: Option<PrimitiveRebindingSignalCompatibilitySubject> = None;
    let _: Option<PrimitiveRebindingSignalCompatibilityInput> = None;
    let _: Option<PrimitiveRebindingSignalCompatibilityArtifact> = None;
    let _: Option<PrimitiveRebindingSignalCompatibilityChecked> = None;
    let _: Option<PrimitiveRebindingSignalCompatibilityOutcome> = None;
    let _: Option<PrimitiveRebindingSignalCompatibilityProof> = None;
    let _: Option<PrimitiveRebindingContinuationTarget> = None;
    let _: Option<PrimitiveRebindingPreparedContinuation> = None;
    let _: Option<PrimitiveRebindingPreparedContinuationChecked> = None;
    let _: Option<PrimitiveRebindingPreparedContinuationOutcome> = None;
    let _: Option<PrimitiveRebindingPreparedContinuationProof> = None;
    let _: Option<PrimitiveRebindingContinuationExecution> = None;
    let _: Option<PrimitiveRebindingContinuationExecutionChecked> = None;
    let _: Option<PrimitiveRebindingContinuationExecutionOutcome> = None;
    let _: Option<PrimitiveRebindingContinuationExecutionProof> = None;
}

#[test]
fn spatial_public_rebinding_signal_and_continuation_surface_exports_builder_functions() {
    let _: fn(
        ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> PrimitiveRebindingSignalCompatibilityInput = primitive_rebinding_signal_workflow;
    let _: fn(
        ForgeQueryDeclarationEnvelope<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ) -> PrimitiveRebindingContinuationTarget = primitive_rebinding_continuation_target;
}

#[test]
fn spatial_public_rebinding_signal_and_continuation_surface_matches_generic_handle_lane() {
    let _ = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(
            "public-api-rebinding-signal",
        ));

    let _ = ForgeQuerySignalCompatibilityOrchestrationInput::<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingDeclarationEntry,
    >::new;

    let _ = forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle::<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >::orchestrate_signal_compatibility_checked::<PrimitiveRebindingDeclarationEntry>;
    let _ = forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle::<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >::prepare_continuation_from_target_checked::<PrimitiveRebindingDeclarationEntry>;
    let _ = forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle::<
        PrimitiveRebindingQueryDomain,
        PrimitiveRebindingQueryWorld,
    >::execute_prepared_continuation_checked::<PrimitiveRebindingDeclarationEntry>;
}

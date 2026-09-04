use worth_runtime_world::facade::{
    AdmittedRelationalBranchBasis, AdmittedRuntimeWorldCorrespondenceBasis,
    AdmittedSignalBranchBasis, CompositeAttemptProgress, PreparedCompositePublicationWithSignal,
    PreparedCompositePublicationWithoutSignal, RelationalOwnerServicePorts,
    RelationalTransactionIntent, ResolvedExpectedProductHead, RuntimeWorldCancellationSource,
    RuntimeWorldCancellationToken, RuntimeWorldCorrespondencePort, RuntimeWorldOwnerInputs,
    RuntimeWorldPublicationOutcome, SignalError, SignalOwnerCancellationToken,
    SignalOwnerServicePorts, SignalTransaction,
};

fn takes_public_return_types(
    progress: &CompositeAttemptProgress,
    expected: &ResolvedExpectedProductHead,
    outcome: &RuntimeWorldPublicationOutcome,
) {
    let _ = progress.relational_posture();
    let _ = expected.expected();
    let _ = format!("{outcome:?}");
}

fn names_public_component_signatures(
    _relational: &RelationalOwnerServicePorts,
    _signal: &SignalOwnerServicePorts<(), (), (), u32, ()>,
    _bridge: &RuntimeWorldCorrespondencePort,
    _relational_basis: &AdmittedRelationalBranchBasis,
    _signal_basis: &AdmittedSignalBranchBasis,
    _correspondence_basis: &AdmittedRuntimeWorldCorrespondenceBasis,
    _intent: Option<RelationalTransactionIntent>,
    _token: Option<SignalOwnerCancellationToken>,
    _world_inputs: Option<RuntimeWorldOwnerInputs<(), (), (), u32, ()>>,
) {
    let _error: Option<SignalError> = None;
    let _transaction: Option<SignalTransaction<'static, (), (), (), u32, ()>> = None;
}

/// The two prepared stages are separate public types. A caller can name both
/// without any conversion between them existing.
fn names_both_prepared_stages(
    _without_signal: Option<PreparedCompositePublicationWithoutSignal>,
    _with_signal: Option<PreparedCompositePublicationWithSignal>,
) {
}

/// The Signal execution seam is an unboxed `FnOnce` carrying exactly the owner
/// port's bound, so a public caller names it without a trait object.
fn names_the_signal_execution_seam<F>(_mutation: F)
where
    F: FnOnce(&mut SignalTransaction<'_, (), (), (), u32, ()>) -> Result<(), SignalError>,
{
}

fn main() {
    let cancellation = RuntimeWorldCancellationSource::new();
    let _token: RuntimeWorldCancellationToken = cancellation.token();
    names_the_signal_execution_seam(|_transaction| Ok(()));
    let _ = (
        takes_public_return_types,
        names_public_component_signatures,
        names_both_prepared_stages,
    );
}

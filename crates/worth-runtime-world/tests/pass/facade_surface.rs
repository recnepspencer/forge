use worth_runtime_world::facade::{
    AdmittedRelationalBranchBasis, AdmittedSignalBranchBasis,
    AdmittedRuntimeWorldCorrespondenceBasis, CompositeAttemptProgress, CompositeExecutionBorrow,
    RelationalOwnerServicePorts, RelationalTransactionIntent, ResolvedExpectedProductHead,
    RuntimeWorldCorrespondencePort, RuntimeWorldOwnerInputs, RuntimeWorldPublicationOutcome,
    SignalError, SignalOwnerCancellationToken, SignalOwnerServicePorts, SignalTransaction,
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
    let _mutation: Option<
        worth_runtime_world::facade::SignalTransactionMutation<'static, (), (), (), u32, ()>,
    > = None;
    let _error: Option<SignalError> = None;
    let _transaction: Option<SignalTransaction<'static, (), (), (), u32, ()>> = None;
}

fn main() {
    let _borrow = CompositeExecutionBorrow::<(), (), (), u32, ()>::without_signal();
    let _ = (takes_public_return_types, names_public_component_signatures);
}

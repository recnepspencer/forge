pub(super) fn success<T>(
    outcome: crate::domain_capabilities::WorthQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        worth_proof::TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}

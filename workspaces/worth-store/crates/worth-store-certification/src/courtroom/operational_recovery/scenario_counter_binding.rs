use std::collections::BTreeSet;

use worth_store_formal_models::OperationalRecoveryRefinementReceipt;
use worth_store_operations::{
    OperationalCounterReceipt, OperationalOperationId, OperationalSessionIdentity,
};
use worth_store_physical_certification::OperationalRecoveryDriverTrace;

use super::S10ScenarioCertificationDenial;

pub(super) fn require_operation_counter_bindings(
    refinement: &OperationalRecoveryRefinementReceipt,
    trace: &OperationalRecoveryDriverTrace,
    counters: &[OperationalCounterReceipt],
) -> Result<(), S10ScenarioCertificationDenial> {
    let driver_operations = trace
        .operation_identities()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if refinement
        .operation_identities()
        .iter()
        .any(|identity| !driver_operations.contains(identity))
    {
        return Err(S10ScenarioCertificationDenial::DriverModelOperationMismatch);
    }
    let driver_sessions = driver_operations
        .iter()
        .map(|identity| OperationalOperationId::new(identity.clone()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| S10ScenarioCertificationDenial::CounterModelOperationMismatch)?
        .iter()
        .map(OperationalSessionIdentity::from_operation)
        .collect::<BTreeSet<_>>();
    let mut counter_sessions = BTreeSet::new();
    for counter in counters {
        if !counter_sessions.insert(counter.session()) {
            return Err(
                S10ScenarioCertificationDenial::DuplicateOperationCounterSession(counter.session()),
            );
        }
    }
    if counter_sessions
        .iter()
        .any(|session| !driver_sessions.contains(session))
    {
        return Err(S10ScenarioCertificationDenial::CounterModelOperationMismatch);
    }
    if driver_sessions
        .iter()
        .any(|session| !counter_sessions.contains(session))
    {
        return Err(S10ScenarioCertificationDenial::MissingOperationCounters);
    }
    Ok(())
}

use std::collections::HashMap;

use super::{OperationalControlHistoryViolationKind, OperationalOperationId};

pub(super) fn observe_authorization_consumption(
    consumed: &mut HashMap<[u8; 32], ([u8; 32], OperationalOperationId)>,
    operation: &OperationalOperationId,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
) -> Result<(), OperationalControlHistoryViolationKind> {
    if let Some((existing_plan, existing_operation)) = consumed.get(&authorization_identity) {
        if *existing_plan != plan_fingerprint || existing_operation != operation {
            return Err(OperationalControlHistoryViolationKind::AuthorizationConsumptionConflict);
        }
        return Ok(());
    }
    consumed
        .try_reserve(1)
        .map_err(|_| OperationalControlHistoryViolationKind::AuthorizationConsumptionConflict)?;
    consumed.insert(
        authorization_identity,
        (plan_fingerprint, operation.clone()),
    );
    Ok(())
}

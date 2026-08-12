//! Safe-retry transition — consumes proof of one completed re-dispatch (R8.66).

use worth_query_installation::facade::InstalledAftermathRecoveryContract;

use crate::domain_computation::managed_run::WorthQueryRecoveryResourceTerminal;

use super::super::external_effect::WorthQueryExternalEffectDispatch;
use super::super::recovery_handle::{
    RelinquishOnDenial, WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleDenial, WorthQueryRecoveryHandleDenialKind,
};
use super::super::recovery_posture::WorthQueryDispatchOutboxDurabilityPosture;
use super::authority::{require_fresh_effect_authority, WorthQueryRecoveryEffectAuthority};
use super::redispatch::WorthQueryPerformedExternalRedispatch;

/// Proof that safe-retry was admitted after a real re-dispatch and the handle
/// was consumed.
#[derive(Debug)]
pub struct WorthQueryRecoverySafeRetryAdmission {
    binding: WorthQueryRecoveryHandleBinding,
    dispatch: WorthQueryExternalEffectDispatch,
    outbox_durability: WorthQueryDispatchOutboxDurabilityPosture,
}

impl WorthQueryRecoverySafeRetryAdmission {
    pub const fn binding(&self) -> &WorthQueryRecoveryHandleBinding {
        &self.binding
    }

    pub const fn dispatch(&self) -> &WorthQueryExternalEffectDispatch {
        &self.dispatch
    }

    /// Process-local outbox lifetime, stated rather than implied (R8.71).
    pub const fn outbox_durability(&self) -> WorthQueryDispatchOutboxDurabilityPosture {
        self.outbox_durability
    }
}

pub fn safe_retry_recovery_handle(
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
    redispatch: WorthQueryPerformedExternalRedispatch,
) -> Result<WorthQueryRecoverySafeRetryAdmission, WorthQueryRecoveryHandleDenial> {
    let handle = handle.admit(|handle| {
        require_fresh_effect_authority(handle, authority)?;
        match handle.binding().installed_aftermath().recovery() {
            InstalledAftermathRecoveryContract::NotAdmitted => {
                return Err(WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::TransitionNotAdmitted,
                ));
            }
            InstalledAftermathRecoveryContract::Admissible { .. } => {}
        }
        // Rules out swapping a redispatch proof performed for handle A into
        // safe-retry for handle B (same runtime, different binding). Rung 3:
        // both sides are runtime values, so this is a comparison rather than a
        // type. The substitution is performed — not merely asserted — by
        // `safe_retry_tests::redispatch_performed_for_handle_a_cannot_safe_retry_handle_b`,
        // and the `None` arm by its sibling; both fail if either arm is removed.
        if redispatch.handle_authority() != handle.authority_identity() {
            return Err(WorthQueryRecoveryHandleDenial::new(
                WorthQueryRecoveryHandleDenialKind::CorrelationMismatch,
            ));
        }
        Ok(())
    })?;
    let dispatch = redispatch.into_dispatch();
    Ok(WorthQueryRecoverySafeRetryAdmission {
        binding: handle.consume(WorthQueryRecoveryResourceTerminal::Consumed)?,
        dispatch,
        outbox_durability: WorthQueryDispatchOutboxDurabilityPosture::StoreCapabilityRequired,
    })
}

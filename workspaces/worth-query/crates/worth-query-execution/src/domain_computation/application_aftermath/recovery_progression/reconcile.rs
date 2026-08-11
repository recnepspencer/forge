//! Reconcile transition — keys RuntimeWithExternalOwner authority (R8.30).

use worth_query_installation::facade::InstalledCorrectionAuthority;

use crate::domain_computation::managed_run::WorthQueryRecoveryResourceTerminal;

use super::super::recovery_handle::{
    RelinquishOnDenial, WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
    WorthQueryRecoveryHandleDenial, WorthQueryRecoveryHandleDenialKind,
};
use super::authority::{require_fresh_effect_authority, WorthQueryRecoveryEffectAuthority};

/// Proof that reconcile was admitted against the installed authority axis.
#[derive(Debug)]
pub struct WorthQueryRecoveryReconcileAdmission {
    binding: WorthQueryRecoveryHandleBinding,
}

impl WorthQueryRecoveryReconcileAdmission {
    pub const fn binding(&self) -> &WorthQueryRecoveryHandleBinding {
        &self.binding
    }
}

pub fn reconcile_recovery_handle(
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
) -> Result<WorthQueryRecoveryReconcileAdmission, WorthQueryRecoveryHandleDenial> {
    let handle = handle.admit(|handle| {
        require_fresh_effect_authority(handle, authority)?;
        match handle.binding().installed_aftermath().authority() {
            InstalledCorrectionAuthority::RuntimeWithExternalOwner => Ok(()),
            InstalledCorrectionAuthority::RuntimeAlone
            | InstalledCorrectionAuthority::NotCorrectable => {
                Err(WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::ReconciliationNotAdmitted,
                ))
            }
        }
    })?;
    Ok(WorthQueryRecoveryReconcileAdmission {
        binding: handle.consume(WorthQueryRecoveryResourceTerminal::Consumed)?,
    })
}

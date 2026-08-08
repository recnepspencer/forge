//! Dispatching a co-committed external effect after the commit is durable.
//!
//! Dispatch runs strictly after the mutation transaction has committed, so the
//! outbox record it reads is already recoverable truth. A host that installed
//! no transport, or an operation that declared no external effect, pays nothing
//! and observes nothing here.
//!
//! Safe-retry re-dispatch (Gate 8.7) shares this module so
//! [`dispatch_external_effect`] remains the single classification site (R8.67).

use std::sync::Arc;

use worth_query_installation::facade::ApplicationSchema;

use super::super::WorthQueryApplicationCommitOutcome;
use crate::domain_computation::application_aftermath::{
    dispatch_external_effect, require_fresh_effect_authority, WorthQueryExternalEffectDispatch,
    WorthQueryExternalEffectTransport, WorthQueryPerformedExternalRedispatch,
    WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryHandle, WorthQueryRecoveryHandleDenial,
    WorthQueryRecoveryHandleDenialKind,
};
use crate::domain_computation::authorization::WorthQueryAdmittedApplicationOperation;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

/// Why a host could not install an external-effect transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExternalTransportInstallationDenial {
    /// A transport is already installed; it is never replaced in flight.
    AlreadyInstalled,
}

/// Why an admitted re-dispatch could not run.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExternalRedispatchDenial {
    /// Fresh effect authority or current admission failed before transport.
    AdmissionDenied,
    /// The live handle binding carries no co-committed outbox record.
    BindingOutboxMissing,
    /// No host transport is installed on this runtime.
    TransportNotInstalled,
    /// Canonical derivation for the dispatch event identity failed.
    DerivationFailed,
}

impl From<WorthQueryExternalRedispatchDenial> for WorthQueryRecoveryHandleDenial {
    fn from(denial: WorthQueryExternalRedispatchDenial) -> Self {
        match denial {
            WorthQueryExternalRedispatchDenial::AdmissionDenied => {
                WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::FreshAuthorityDenied,
                )
            }
            WorthQueryExternalRedispatchDenial::BindingOutboxMissing => {
                WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::CorrelationMismatch,
                )
            }
            WorthQueryExternalRedispatchDenial::TransportNotInstalled
            | WorthQueryExternalRedispatchDenial::DerivationFailed => {
                WorthQueryRecoveryHandleDenial::new(
                    WorthQueryRecoveryHandleDenialKind::TransitionNotAdmitted,
                )
            }
        }
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    /// Installs the host's external-effect transport, once, for this runtime.
    pub fn install_external_effect_transport(
        &self,
        transport: Arc<dyn WorthQueryExternalEffectTransport>,
    ) -> Result<(), WorthQueryExternalTransportInstallationDenial> {
        self.external_effect_transport
            .set(transport)
            .map_err(|_| WorthQueryExternalTransportInstallationDenial::AlreadyInstalled)
    }

    /// True when this runtime can carry a declared effect out to its rail.
    pub fn has_external_effect_transport(&self) -> bool {
        self.external_effect_transport.get().is_some()
    }

    /// Re-dispatch the handle binding's co-committed outbox through the transport.
    ///
    /// Fresh effect authority is required before any transport call (R8.69). The
    /// outbox is read from the live handle binding — never from a caller-held
    /// receipt copy. Classification stays inside [`dispatch_external_effect`]
    /// (R8.67). The returned proof is privately minted.
    pub fn redispatch_admitted_external_effect<Operation, Input, Scope>(
        &self,
        handle: &WorthQueryRecoveryHandle,
        authority: &WorthQueryRecoveryEffectAuthority,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    ) -> Result<WorthQueryPerformedExternalRedispatch, WorthQueryExternalRedispatchDenial> {
        require_fresh_effect_authority(handle, authority)
            .map_err(|_| WorthQueryExternalRedispatchDenial::AdmissionDenied)?;
        admission
            .validate_current_authority()
            .map_err(|_| WorthQueryExternalRedispatchDenial::AdmissionDenied)?;
        if !admission.belongs_to(
            self.runtime.authority_identity(),
            &self.installed_schema.binding_identity(),
        ) {
            return Err(WorthQueryExternalRedispatchDenial::AdmissionDenied);
        }
        if handle.binding().dispatch_outbox().is_none() {
            return Err(WorthQueryExternalRedispatchDenial::BindingOutboxMissing);
        }
        let committed = self
            .primary_provider
            .committed_dispatch_outbox_for_binding(handle.binding())
            .map_err(|_| WorthQueryExternalRedispatchDenial::DerivationFailed)?;
        let Some(transport) = self.external_effect_transport.get() else {
            return Err(WorthQueryExternalRedispatchDenial::TransportNotInstalled);
        };
        let dispatch = self
            .dispatch_committed_observation_with_fresh_attempt(transport.as_ref(), committed)
            .ok_or(WorthQueryExternalRedispatchDenial::DerivationFailed)?
            .map_err(|_| WorthQueryExternalRedispatchDenial::DerivationFailed)?;
        Ok(WorthQueryPerformedExternalRedispatch::record(
            handle.authority_identity(),
            dispatch,
        ))
    }

    pub(super) fn dispatch_committed_external_effect(
        &self,
        outcome: WorthQueryApplicationCommitOutcome,
    ) -> WorthQueryApplicationCommitOutcome {
        let WorthQueryApplicationCommitOutcome::Committed(receipt) = outcome else {
            return outcome;
        };
        if receipt.dispatch_outbox().is_none() {
            return WorthQueryApplicationCommitOutcome::Committed(receipt);
        }
        let Some(transport) = self.external_effect_transport.get() else {
            return WorthQueryApplicationCommitOutcome::Committed(receipt);
        };
        let Ok(Some(committed)) = self.observe_committed_dispatch_outbox(&receipt) else {
            return WorthQueryApplicationCommitOutcome::Committed(receipt);
        };
        let Some(dispatch) =
            self.dispatch_committed_observation_with_fresh_attempt(transport.as_ref(), committed)
        else {
            return WorthQueryApplicationCommitOutcome::Committed(receipt);
        };
        match dispatch {
            Ok(dispatch) => WorthQueryApplicationCommitOutcome::Committed(
                receipt.with_external_dispatch(dispatch),
            ),
            Err(_) => WorthQueryApplicationCommitOutcome::Committed(receipt),
        }
    }

    fn dispatch_committed_observation_with_fresh_attempt(
        &self,
        transport: &dyn WorthQueryExternalEffectTransport,
        committed: crate::domain_computation::primary_graph::WorthQueryCommittedDispatchOutboxObservation,
    ) -> Option<
        Result<
            WorthQueryExternalEffectDispatch,
            crate::domain_computation::application_aftermath::WorthQueryAftermathDerivationFailure,
        >,
    > {
        let admitted = self.admit_external_dispatch_attempt(committed).ok()?;
        Some(dispatch_external_effect(transport, admitted))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::domain_computation::application_aftermath::{
        external_effect::tests::committed_outbox, WorthQueryExternalDispatchRequest,
        WorthQueryExternalTransportOutcome,
    };
    use crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world;

    struct RetryTransport(AtomicUsize);

    impl WorthQueryExternalEffectTransport for RetryTransport {
        fn dispatch(
            &self,
            _request: WorthQueryExternalDispatchRequest<'_>,
        ) -> WorthQueryExternalTransportOutcome {
            match self.0.fetch_add(1, Ordering::AcqRel) {
                0 => WorthQueryExternalTransportOutcome::LostResponse,
                _ => WorthQueryExternalTransportOutcome::Completed,
            }
        }
    }

    #[test]
    fn production_fresh_attempt_operation_distinguishes_safe_redispatch() {
        let world = installed_authorization_world(true);
        let transport = RetryTransport(AtomicUsize::new(0));
        let original_observation = committed_outbox(11);
        let retry_observation = committed_outbox(11);
        assert_eq!(
            original_observation.record().correlation(),
            retry_observation.record().correlation()
        );
        let original = world
            .application
            .dispatch_committed_observation_with_fresh_attempt(&transport, original_observation)
            .expect("runtime attempt ordinal")
            .expect("original dispatch");
        let retry = world
            .application
            .dispatch_committed_observation_with_fresh_attempt(&transport, retry_observation)
            .expect("runtime retry ordinal")
            .expect("safe redispatch");
        assert_eq!(
            original.causal_ladder().emission().identity(),
            retry.causal_ladder().emission().identity()
        );
        assert_ne!(
            original.causal_ladder().attempt().identity(),
            retry.causal_ladder().attempt().identity()
        );
        assert!(retry.is_external_completion());
    }
}

use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedLiveLifecyclePosture {
    Active,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryManagedLiveLifecycleObservation {
    resource_name: String,
    posture: WorthQueryManagedLiveLifecyclePosture,
    installation_identity: WorthQueryEvidenceIdentity,
    basis_binding_identity: WorthQueryEvidenceIdentity,
    pending_delivery_batch_count: usize,
    last_delivery_sequence: Option<u64>,
}

impl WorthQueryManagedLiveLifecycleObservation {
    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn posture(&self) -> WorthQueryManagedLiveLifecyclePosture {
        self.posture
    }

    pub fn installation_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.installation_identity
    }

    pub fn basis_binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn pending_delivery_batch_count(&self) -> usize {
        self.pending_delivery_batch_count
    }

    pub fn last_delivery_sequence(&self) -> Option<u64> {
        self.last_delivery_sequence
    }

    pub(super) fn from_state(state: &WorthQueryRuntimeLiveSubscriptionState) -> Self {
        Self {
            resource_name: state.installation.view_name().to_string(),
            posture: WorthQueryManagedLiveLifecyclePosture::Active,
            installation_identity: state.installation.installation_identity().clone(),
            basis_binding_identity: state.installation.basis_binding_identity().clone(),
            pending_delivery_batch_count: state.delivery_batches.len(),
            last_delivery_sequence: state
                .last_delivery
                .as_ref()
                .map(|delivery| delivery.sequence()),
        }
    }
}

impl WorthQueryRuntime {
    pub(crate) fn observe_managed_live_view<T>(
        &mut self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<WorthQueryManagedLiveLifecycleObservation, WorthQueryRuntimeError> {
        self.reap_abandoned_managed_live_resources()?;
        let target = WorthQueryLiveArtifactTarget::from_subscription_installation(
            view.subscription_installation(),
        );
        let state = self.live_subscriptions.get(&target).ok_or_else(|| {
            WorthQueryRuntimeError::MissingLiveSubscription(view.name().to_string())
        })?;
        if state.installation != *view.subscription_installation()
            || state.read_authority_binding.is_none()
        {
            return Err(WorthQueryRuntimeError::MissingLiveSubscription(
                view.name().to_string(),
            ));
        }
        Ok(WorthQueryManagedLiveLifecycleObservation::from_state(state))
    }
}

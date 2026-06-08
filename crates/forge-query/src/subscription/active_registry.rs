use std::collections::BTreeMap;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_error::{
    ActiveSubscriptionLifecycleDenialKind, ActiveSubscriptionLifecycleError,
};
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::active_lane::{ActiveSubscriptionLane, ActiveSubscriptionLaneAdmission};
use super::active_posture::ActiveSubscriptionLifecyclePosture;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneRegistry {
    lanes: Vec<Option<ActiveSubscriptionLane>>,
    lane_index_by_digest: BTreeMap<String, usize>,
    attachment_lane_by_digest: BTreeMap<String, usize>,
    generation: u64,
}

impl ActiveSubscriptionLaneRegistry {
    pub(super) fn open_lane(
        &mut self,
        admission: ActiveSubscriptionLaneAdmission,
    ) -> Result<
        (ActiveSubscriptionLaneHandle, ActiveSubscriptionCounters),
        ActiveSubscriptionLifecycleError,
    > {
        let mut counters = admission.counters.clone();
        counters.active_lane_registry_lookup_count = 1;

        if self
            .lane_index_by_digest
            .contains_key(admission.lane_digest.as_str())
        {
            let lane_index = *self
                .lane_index_by_digest
                .get(admission.lane_digest.as_str())
                .expect("lane index exists after contains check");
            self.generation += 1;
            counters.active_lane_join_count = 1;
            counters.shared_lane_count = 1;
            counters.active_lane_handle_issue_count = 1;
            self.lanes[lane_index]
                .as_mut()
                .expect("live lane index must reference active lane")
                .lifecycle_posture = ActiveSubscriptionLifecyclePosture::SharedEquivalent;
            let handle = ActiveSubscriptionLaneHandle::new(
                admission.lane_digest.clone(),
                admission.future_selection.clone(),
                admission.basis_binding_digest.clone(),
                admission.checkpoint_identity_digest.clone(),
                lane_index as u64,
                self.generation,
            );
            return Ok((handle, counters));
        }

        self.generation += 1;
        counters.active_lane_creation_count = 1;
        counters.active_lane_handle_issue_count = 1;
        let lane_index = self.lanes.len();
        let handle = ActiveSubscriptionLaneHandle::new(
            admission.lane_digest.clone(),
            admission.future_selection.clone(),
            admission.basis_binding_digest.clone(),
            admission.checkpoint_identity_digest.clone(),
            lane_index as u64,
            self.generation,
        );
        self.lane_index_by_digest
            .insert(admission.lane_digest.as_str().to_string(), lane_index);
        self.lanes.push(Some(ActiveSubscriptionLane {
            lane_digest: admission.lane_digest,
            activation_digest: admission.activation_digest,
            admission_digest: admission.admission_digest,
            query_declaration_digest: admission.query_declaration_digest,
            bridge_declaration_digest: admission.bridge_declaration_digest,
            future_selection: admission.future_selection,
            basis_binding_digest: admission.basis_binding_digest,
            checkpoint_identity_digest: admission.checkpoint_identity_digest,
            signal_strategy_digest: admission.signal_strategy_digest,
            lifecycle_posture: admission.lifecycle_posture,
            delivery_posture: admission.delivery_posture,
            lookup_class: admission.lookup_class,
            allocation_policy: admission.allocation_policy,
            attachment_count: 0,
        }));
        Ok((handle, counters))
    }

    pub(super) fn join_lane(
        &mut self,
        handle: &ActiveSubscriptionLaneHandle,
        admission: ActiveSubscriptionLaneAdmission,
    ) -> Result<
        (ActiveSubscriptionLaneHandle, ActiveSubscriptionCounters),
        ActiveSubscriptionLifecycleError,
    > {
        self.validate_handle(handle)?;
        let mut counters = admission.counters.clone();
        counters.active_lane_registry_lookup_count = 1;

        if handle.lane_digest() != &admission.lane_digest {
            counters.active_lane_join_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane join requires matching subscription equivalence evidence",
                admission.lane_digest.as_str(),
                counters,
            ));
        }

        self.open_lane(admission)
    }

    pub fn lane_count(&self) -> usize {
        self.lane_index_by_digest.len()
    }

    pub(super) fn lane_lifecycle_posture(
        &self,
        handle: &ActiveSubscriptionLaneHandle,
    ) -> Option<&ActiveSubscriptionLifecyclePosture> {
        let index = self
            .lane_index_by_digest
            .get(handle.lane_digest().as_str())
            .copied()?;
        if index as u64 != handle.lane_index() {
            return None;
        }
        self.lanes
            .get(index)
            .and_then(|lane| lane.as_ref())
            .map(|lane| lane.lifecycle_posture())
    }

    pub(super) fn validate_handle(
        &self,
        handle: &ActiveSubscriptionLaneHandle,
    ) -> Result<(), ActiveSubscriptionLifecycleError> {
        let mut counters = ActiveSubscriptionCounters::default();
        counters.consumer_attachment_denial_count = 1;
        let Some(index) = self
            .lane_index_by_digest
            .get(handle.lane_digest().as_str())
            .copied()
        else {
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane handle does not belong to this registry",
                handle.lane_digest().as_str(),
                counters,
            ));
        };
        if index as u64 != handle.lane_index() {
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane handle index does not match registry lane digest",
                handle.lane_digest().as_str(),
                counters,
            ));
        }
        if self
            .lanes
            .get(index)
            .and_then(|lane| lane.as_ref())
            .is_none()
        {
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane handle references a closed lifecycle lane",
                handle.lane_digest().as_str(),
                counters,
            ));
        }
        Ok(())
    }

    pub(super) fn register_attachment(
        &mut self,
        handle: &ActiveSubscriptionLaneHandle,
        attachment_digest: &str,
    ) -> Result<(), ActiveSubscriptionLifecycleError> {
        self.validate_handle(handle)?;
        let index = *self
            .lane_index_by_digest
            .get(handle.lane_digest().as_str())
            .expect("validated handle must resolve lane index");
        let lane = self.lanes[index]
            .as_mut()
            .expect("validated handle must reference live lane");
        lane.attachment_count += 1;
        self.attachment_lane_by_digest
            .insert(attachment_digest.to_string(), index);
        Ok(())
    }

    pub(super) fn close_attachment(
        &mut self,
        handle: &ActiveSubscriptionLaneHandle,
        attachment_digest: &str,
    ) -> Result<bool, ActiveSubscriptionLifecycleError> {
        self.validate_handle(handle)?;
        let mut counters = ActiveSubscriptionCounters::default();
        let Some(index) = self.attachment_lane_by_digest.remove(attachment_digest) else {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "subscription lifecycle closeout requires an active registered consumer attachment",
                attachment_digest,
                counters,
            ));
        };
        if index as u64 != handle.lane_index() {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "subscription lifecycle closeout attachment does not belong to the requested lane handle",
                attachment_digest,
                counters,
            ));
        }

        let lane = self.lanes[index]
            .as_mut()
            .expect("validated handle must reference live lane");
        lane.attachment_count = lane.attachment_count.saturating_sub(1);
        if lane.attachment_count == 0 {
            self.lane_index_by_digest
                .remove(handle.lane_digest().as_str());
            self.lanes[index] = None;
            return Ok(true);
        }
        Ok(false)
    }
}

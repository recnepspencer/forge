use std::collections::BTreeMap;

use super::active_counters::ActiveSubscriptionCounters;
use super::active_error::{
    ActiveSubscriptionLifecycleDenialKind, ActiveSubscriptionLifecycleError,
};
use super::active_handle::ActiveSubscriptionLaneHandle;
use super::active_lane::{ActiveSubscriptionLane, ActiveSubscriptionLaneAdmission};
use super::active_posture::ActiveSubscriptionLifecyclePosture;
use super::attachment_digest::SubscriptionConsumerAttachmentDigest;
use super::ActiveSubscriptionLaneDigest;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct ActiveSubscriptionLaneRegistry {
    lanes: Vec<Option<ActiveSubscriptionLane>>,
    lane_index_by_digest: BTreeMap<ActiveSubscriptionLaneDigest, usize>,
    attachment_lane_by_digest: BTreeMap<SubscriptionConsumerAttachmentDigest, usize>,
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

        if let Some(lane_index) = self
            .lane_index_by_digest
            .get(&admission.lane_digest)
            .copied()
        {
            return Ok(self.join_equivalent_open_lane(admission, lane_index, counters));
        }

        Ok(self.create_open_lane(admission, counters))
    }

    fn join_equivalent_open_lane(
        &mut self,
        admission: ActiveSubscriptionLaneAdmission,
        lane_index: usize,
        mut counters: ActiveSubscriptionCounters,
    ) -> (ActiveSubscriptionLaneHandle, ActiveSubscriptionCounters) {
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
            admission.basis_binding_identity.clone(),
            admission.checkpoint_identity.clone(),
            lane_index as u64,
            self.generation,
        );
        (handle, counters)
    }

    fn create_open_lane(
        &mut self,
        admission: ActiveSubscriptionLaneAdmission,
        mut counters: ActiveSubscriptionCounters,
    ) -> (ActiveSubscriptionLaneHandle, ActiveSubscriptionCounters) {
        self.generation += 1;
        counters.active_lane_creation_count = 1;
        counters.active_lane_handle_issue_count = 1;
        let lane_index = self.lanes.len();
        let handle = ActiveSubscriptionLaneHandle::new(
            admission.lane_digest.clone(),
            admission.future_selection.clone(),
            admission.basis_binding_identity.clone(),
            admission.checkpoint_identity.clone(),
            lane_index as u64,
            self.generation,
        );
        self.lane_index_by_digest
            .insert(admission.lane_digest.clone(), lane_index);
        self.lanes.push(Some(ActiveSubscriptionLane {
            lane_digest: admission.lane_digest,
            activation_identity: admission.activation_identity,
            admission_identity: admission.admission_identity,
            query_declaration_identity: admission.query_declaration_identity,
            bridge_declaration_identity: admission.bridge_declaration_identity,
            future_selection: admission.future_selection,
            basis_binding_identity: admission.basis_binding_identity,
            scoped_declaration_basis: admission.scoped_declaration_basis,
            scoped_activation_basis: admission.scoped_activation_basis,
            checkpoint_identity: admission.checkpoint_identity,
            signal_strategy_identity: admission.signal_strategy_identity,
            lifecycle_posture: admission.lifecycle_posture,
            delivery_posture: admission.delivery_posture,
            lookup_class: admission.lookup_class,
            allocation_policy: admission.allocation_policy,
            attachment_count: 0,
        }));
        (handle, counters)
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
                admission.lane_digest.evidence_identity().clone(),
                counters,
            ));
        }

        self.open_lane(admission)
    }

    pub fn lane_count(&self) -> usize {
        self.lane_index_by_digest.len()
    }

    pub(super) fn attachment_count(&self) -> usize {
        self.attachment_lane_by_digest.len()
    }

    pub(super) fn lane_lifecycle_posture(
        &self,
        handle: &ActiveSubscriptionLaneHandle,
    ) -> Option<&ActiveSubscriptionLifecyclePosture> {
        let index = self
            .lane_index_by_digest
            .get(handle.lane_digest())
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
        let Some(index) = self.lane_index_by_digest.get(handle.lane_digest()).copied() else {
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane handle does not belong to this registry",
                handle.lane_digest().evidence_identity().clone(),
                counters,
            ));
        };
        if index as u64 != handle.lane_index() {
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::RegistryEquivalenceMismatch,
                "active lane handle index does not match registry lane digest",
                handle.lane_digest().evidence_identity().clone(),
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
                handle.lane_digest().evidence_identity().clone(),
                counters,
            ));
        }
        Ok(())
    }

    pub(super) fn register_attachment(
        &mut self,
        handle: &ActiveSubscriptionLaneHandle,
        attachment_digest: &SubscriptionConsumerAttachmentDigest,
    ) -> Result<(), ActiveSubscriptionLifecycleError> {
        self.validate_handle(handle)?;
        let index = *self
            .lane_index_by_digest
            .get(handle.lane_digest())
            .expect("validated handle must resolve lane index");
        let lane = self.lanes[index]
            .as_mut()
            .expect("validated handle must reference live lane");
        lane.attachment_count += 1;
        self.attachment_lane_by_digest
            .insert(attachment_digest.clone(), index);
        Ok(())
    }

    pub(super) fn close_attachment(
        &mut self,
        handle: &ActiveSubscriptionLaneHandle,
        attachment_digest: &SubscriptionConsumerAttachmentDigest,
    ) -> Result<bool, ActiveSubscriptionLifecycleError> {
        self.validate_handle(handle)?;
        let mut counters = ActiveSubscriptionCounters::default();
        let Some(index) = self.attachment_lane_by_digest.remove(attachment_digest) else {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::AttachmentNotActive,
                "subscription lifecycle closeout requires an active registered consumer attachment",
                attachment_digest.evidence_identity().clone(),
                counters,
            ));
        };
        if index as u64 != handle.lane_index() {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::AttachmentLaneMismatch,
                "subscription lifecycle closeout attachment does not belong to the requested lane handle",
                attachment_digest.evidence_identity().clone(),
                counters,
            ));
        }

        let lane = self.lanes[index]
            .as_mut()
            .expect("validated handle must reference live lane");
        lane.attachment_count = lane.attachment_count.saturating_sub(1);
        if lane.attachment_count == 0 {
            self.lane_index_by_digest.remove(handle.lane_digest());
            self.lanes[index] = None;
            return Ok(true);
        }
        Ok(false)
    }

    pub(super) fn commit_prepared_attachment_close(
        &mut self,
        lane_digest: &ActiveSubscriptionLaneDigest,
        lane_index: usize,
        attachment_digest: &SubscriptionConsumerAttachmentDigest,
    ) -> bool {
        assert_eq!(
            self.lane_index_by_digest.get(lane_digest).copied(),
            Some(lane_index),
            "prepared lifecycle close lane must remain exact under exclusive runtime ownership"
        );
        assert_eq!(
            self.attachment_lane_by_digest.remove(attachment_digest),
            Some(lane_index),
            "prepared lifecycle close attachment must remain active until commit"
        );
        let lane = self.lanes[lane_index]
            .as_mut()
            .expect("prepared lifecycle close must retain its active lane");
        assert!(lane.attachment_count > 0);
        lane.attachment_count -= 1;
        if lane.attachment_count == 0 {
            self.lane_index_by_digest.remove(lane_digest);
            self.lanes[lane_index] = None;
            true
        } else {
            false
        }
    }

    pub(super) fn validate_attachment_close(
        &self,
        handle: &ActiveSubscriptionLaneHandle,
        attachment_digest: &SubscriptionConsumerAttachmentDigest,
    ) -> Result<(), ActiveSubscriptionLifecycleError> {
        self.validate_handle(handle)?;
        let mut counters = ActiveSubscriptionCounters::default();
        let Some(index) = self
            .attachment_lane_by_digest
            .get(attachment_digest)
            .copied()
        else {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::AttachmentNotActive,
                "subscription lifecycle closeout requires an active registered consumer attachment",
                attachment_digest.evidence_identity().clone(),
                counters,
            ));
        };
        if index as u64 != handle.lane_index() {
            counters.subscription_lifecycle_closeout_denial_count = 1;
            return Err(ActiveSubscriptionLifecycleError::new(
                ActiveSubscriptionLifecycleDenialKind::AttachmentLaneMismatch,
                "subscription lifecycle closeout attachment does not belong to the requested lane handle",
                attachment_digest.evidence_identity().clone(),
                counters,
            ));
        }
        Ok(())
    }
}

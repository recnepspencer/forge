use crate::{
    WorthUiOperationLiveCloseOutcome, WorthUiOperationLiveRefreshError,
    WorthUiOperationLiveRefreshOutcome,
};

pub(super) struct LiveBindingFixture {
    pub(super) owner: crate::certification::WorthUiOperationLiveTestFixture,
    pub(super) reference: crate::WorthUiInstalledQueryBindingReference,
    pub(super) binding: crate::WorthUiRuntimeQueryBinding,
}

impl LiveBindingFixture {
    pub(super) fn new(label: &str) -> Self {
        Self::from_owner(crate::certification::WorthUiOperationLiveTestFixture::new(
            label,
        ))
    }

    pub(super) fn with_rows(label: &str, identities: &[&str], breadth: u32) -> Self {
        Self::from_owner(
            crate::certification::WorthUiOperationLiveTestFixture::with_rows(
                label, identities, breadth,
            ),
        )
    }

    pub(super) fn without_collection_entity_lookup(label: &str) -> Self {
        Self::from_owner(
            crate::certification::WorthUiOperationLiveTestFixture::without_collection_entity_lookup(
                label,
            ),
        )
    }

    pub(super) fn with_tail_rows(label: &str, identities: &[&str], breadth: u32) -> Self {
        let mut owner = crate::certification::WorthUiOperationLiveTestFixture::with_rows(
            label, identities, breadth,
        );
        let resource = owner.open_tail_resource();
        Self::from_owner_and_resource(owner, resource)
    }

    pub(super) fn with_failed_close(label: &str) -> Self {
        Self::from_owner(
            crate::certification::WorthUiOperationLiveTestFixture::with_failed_close(label),
        )
    }

    fn from_owner(mut owner: crate::certification::WorthUiOperationLiveTestFixture) -> Self {
        let resource = owner.open_resource();
        Self::from_owner_and_resource(owner, resource)
    }

    fn from_owner_and_resource(
        owner: crate::certification::WorthUiOperationLiveTestFixture,
        resource: crate::WorthUiOperationLiveResource,
    ) -> Self {
        let reference = owner.reference().clone();
        let plan = owner.binding_plan();
        let mut binding = plan.prepare_downstream_state();
        binding.admit_operation_live(resource).unwrap();
        Self {
            owner,
            reference,
            binding,
        }
    }

    pub(super) fn refresh(
        &mut self,
    ) -> Result<WorthUiOperationLiveRefreshOutcome, WorthUiOperationLiveRefreshError> {
        self.binding
            .refresh_operation_live(self.owner.refresh_request())
    }

    pub(super) fn admit_and_publish(
        &mut self,
        consequence: crate::WorthUiCollectionChangeConsequence,
    ) {
        self.binding
            .admit_operation_live_change(consequence)
            .unwrap();
        assert_eq!(
            self.binding
                .publish_staged_operation_live_changes()
                .published_change_count(),
            1
        );
    }

    pub(super) fn close(&mut self) {
        let resource = self
            .binding
            .take_operation_live_resource(&self.reference)
            .expect("fixture retains its live resource");
        assert!(matches!(
            self.owner.close_resource(resource),
            WorthUiOperationLiveCloseOutcome::Closed(_)
        ));
    }
}

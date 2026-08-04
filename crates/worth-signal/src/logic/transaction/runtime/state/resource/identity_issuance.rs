use super::ResourceRuntimeState;
use crate::data::resource::{
    AsyncDenialId, ResourceCancellationOrdinal, ResourceCompletionOrdinal, ResourceDescriptorId,
    ResourceGeneration, ResourceLifecycleOrdinal, ResourceRejectionOrdinal, ResourceRequestId,
    ResourceRetryOrdinal, ResourceSupersessionOrdinal, ResourceTimeoutOrdinal,
};

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn issue_descriptor_id(
        &mut self,
    ) -> ResourceDescriptorId {
        let id = self.next_descriptor_id;
        self.next_descriptor_id = id.next();
        id
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_request_id(
        &mut self,
    ) -> ResourceRequestId {
        let id = self.next_request_id;
        self.next_request_id = ResourceRequestId::new(id.get().saturating_add(1));
        id
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_generation(
        &mut self,
    ) -> ResourceGeneration {
        self.next_generation =
            ResourceGeneration::new(self.next_generation.get().saturating_add(1));
        self.next_generation
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_lifecycle_ordinal(
        &mut self,
    ) -> ResourceLifecycleOrdinal {
        self.next_lifecycle_ordinal =
            ResourceLifecycleOrdinal::new(self.next_lifecycle_ordinal.get().saturating_add(1));
        self.next_lifecycle_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_denial_id(
        &mut self,
    ) -> AsyncDenialId {
        let id = self.next_denial_id;
        self.next_denial_id = AsyncDenialId::new(id.get().saturating_add(1));
        id
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_completion_ordinal(
        &mut self,
    ) -> ResourceCompletionOrdinal {
        self.next_completion_ordinal =
            ResourceCompletionOrdinal::new(self.next_completion_ordinal.get().saturating_add(1));
        self.next_completion_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_cancellation_ordinal(
        &mut self,
    ) -> ResourceCancellationOrdinal {
        self.next_cancellation_ordinal = ResourceCancellationOrdinal::new(
            self.next_cancellation_ordinal.get().saturating_add(1),
        );
        self.next_cancellation_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_timeout_ordinal(
        &mut self,
    ) -> ResourceTimeoutOrdinal {
        self.next_timeout_ordinal =
            ResourceTimeoutOrdinal::new(self.next_timeout_ordinal.get().saturating_add(1));
        self.next_timeout_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_rejection_ordinal(
        &mut self,
    ) -> ResourceRejectionOrdinal {
        self.next_rejection_ordinal =
            ResourceRejectionOrdinal::new(self.next_rejection_ordinal.get().saturating_add(1));
        self.next_rejection_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_supersession_ordinal(
        &mut self,
    ) -> ResourceSupersessionOrdinal {
        self.next_supersession_ordinal = ResourceSupersessionOrdinal::new(
            self.next_supersession_ordinal.get().saturating_add(1),
        );
        self.next_supersession_ordinal
    }

    pub(in crate::logic::transaction::runtime::state::resource) fn issue_retry_ordinal(
        &mut self,
    ) -> ResourceRetryOrdinal {
        self.next_retry_ordinal =
            ResourceRetryOrdinal::new(self.next_retry_ordinal.get().saturating_add(1));
        self.next_retry_ordinal
    }
}

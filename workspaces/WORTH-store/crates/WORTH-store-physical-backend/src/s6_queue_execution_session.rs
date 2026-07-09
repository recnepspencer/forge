use crate::{
    AdmittedBackendCapabilityWitness, BackendQueueExecutionAdaptation,
    BackendQueueExecutionBackpressure, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding, BackendQueueExecutionTicketDenial,
    BackendQueueSpeculativeScope, PhysicalReference, PhysicalStoreBackend,
};

#[derive(Debug, Eq, PartialEq)]
pub enum BackendQueueExecutionRunError<BackendError> {
    TicketDenied(BackendQueueExecutionTicketDenial),
    Backend(BackendError),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BackendQueueExecutionObservedCounters {
    queue_depth_sample: u32,
    read_ahead_units: u64,
    read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    write_back_units: u64,
    write_back_scope: Option<BackendQueueSpeculativeScope>,
    mechanical_retries: u64,
    partial_read_events: u64,
    short_write_events: u64,
    backpressure: Option<BackendQueueExecutionBackpressure>,
    foreground_wait_events: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoreOwnedBackendQueueExecution {
    _private: (),
}

pub struct BackendQueueExecutionSession<'backend, Backend> {
    backend: &'backend mut Backend,
    _authority: StoreOwnedBackendQueueExecution,
}

impl BackendQueueExecutionObservedCounters {
    pub const fn new() -> Self {
        Self {
            queue_depth_sample: 0,
            read_ahead_units: 0,
            read_ahead_scope: None,
            write_back_units: 0,
            write_back_scope: None,
            mechanical_retries: 0,
            partial_read_events: 0,
            short_write_events: 0,
            backpressure: None,
            foreground_wait_events: 0,
        }
    }

    pub const fn observe_queue_depth(mut self, queue_depth_sample: u32) -> Self {
        self.queue_depth_sample = queue_depth_sample;
        self
    }

    pub const fn observe_read_ahead(
        mut self,
        units: u64,
        scope: BackendQueueSpeculativeScope,
    ) -> Self {
        self.read_ahead_units = units;
        self.read_ahead_scope = Some(scope);
        self
    }

    pub const fn observe_write_back(
        mut self,
        units: u64,
        scope: BackendQueueSpeculativeScope,
    ) -> Self {
        self.write_back_units = units;
        self.write_back_scope = Some(scope);
        self
    }

    pub const fn observe_mechanical_adaptation(
        mut self,
        retries: u64,
        partial_reads: u64,
        short_writes: u64,
    ) -> Self {
        self.mechanical_retries = retries;
        self.partial_read_events = partial_reads;
        self.short_write_events = short_writes;
        self
    }

    pub const fn observe_backpressure(
        mut self,
        backpressure: BackendQueueExecutionBackpressure,
    ) -> Self {
        self.backpressure = Some(backpressure);
        self
    }

    pub const fn observe_foreground_wait_events(mut self, foreground_wait_events: u64) -> Self {
        self.foreground_wait_events = foreground_wait_events;
        self
    }
}

impl StoreOwnedBackendQueueExecution {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }
}

impl<'backend, Backend> BackendQueueExecutionSession<'backend, Backend>
where
    Backend: PhysicalStoreBackend,
{
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedBackendQueueExecution,
    ) -> Self {
        Self {
            backend,
            _authority: authority,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedBackendQueueExecution::store_owned())
    }

    pub fn complete_after_append(
        &mut self,
        binding: BackendQueueExecutionPlanBinding,
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
        bytes: &[u8],
        observations: BackendQueueExecutionObservedCounters,
    ) -> Result<
        (PhysicalReference, BackendQueueExecutionCompletion),
        BackendQueueExecutionRunError<Backend::Error>,
    > {
        let reference = self
            .backend
            .append_framed_record(bytes)
            .map_err(BackendQueueExecutionRunError::Backend)?;
        let completion = self.complete_ticket(binding, witness, adaptation, observations)?;
        Ok((reference, completion))
    }

    pub fn complete_after_read(
        &self,
        binding: BackendQueueExecutionPlanBinding,
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
        reference: PhysicalReference,
        observations: BackendQueueExecutionObservedCounters,
    ) -> Result<
        (Vec<u8>, BackendQueueExecutionCompletion),
        BackendQueueExecutionRunError<Backend::Error>,
    > {
        let bytes = self
            .backend
            .read_framed_record(reference)
            .map_err(BackendQueueExecutionRunError::Backend)?;
        let completion = self.complete_ticket(binding, witness, adaptation, observations)?;
        Ok((bytes, completion))
    }

    fn complete_ticket(
        &self,
        binding: BackendQueueExecutionPlanBinding,
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
        observations: BackendQueueExecutionObservedCounters,
    ) -> Result<BackendQueueExecutionCompletion, BackendQueueExecutionRunError<Backend::Error>>
    {
        let ticket = crate::BackendQueueExecutionAuthority::store_owned()
            .issue_ticket(binding, witness, adaptation)
            .map_err(BackendQueueExecutionRunError::TicketDenied)?;
        let mut completion = ticket
            .begin_completion()
            .observe_queue_depth(observations.queue_depth_sample)
            .observe_mechanical_adaptation(
                observations.mechanical_retries,
                observations.partial_read_events,
                observations.short_write_events,
            )
            .observe_foreground_wait_events(observations.foreground_wait_events);
        if let Some(scope) = observations.read_ahead_scope {
            completion = completion.observe_read_ahead(observations.read_ahead_units, scope);
        }
        if let Some(scope) = observations.write_back_scope {
            completion = completion.observe_write_back(observations.write_back_units, scope);
        }
        if let Some(backpressure) = observations.backpressure {
            completion = completion.observe_backpressure(backpressure);
        }
        Ok(completion.complete())
    }
}

#[cfg(test)]
mod tests;

use crate::{AdmittedBackendCapabilityWitness, BackendQueueExecutionAdaptation};
use crate::{
    BackendQueueExecutionBackpressure, BackendQueueExecutionCompletion,
    BackendQueueExecutionPlanBinding, BackendQueueExecutionPosture, BackendQueueSpeculativeScope,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendQueueExecutionTicketDenial {
    BackendProfileMismatch,
    BackendEvidenceClassMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionAuthority {
    _private: (),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionTicket {
    binding: BackendQueueExecutionPlanBinding,
    posture: BackendQueueExecutionPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackendQueueExecutionCompletionBuilder {
    ticket: BackendQueueExecutionTicket,
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

impl BackendQueueExecutionAuthority {
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub(crate) fn issue_ticket(
        self,
        binding: BackendQueueExecutionPlanBinding,
        witness: &AdmittedBackendCapabilityWitness,
        adaptation: BackendQueueExecutionAdaptation,
    ) -> Result<BackendQueueExecutionTicket, BackendQueueExecutionTicketDenial> {
        if binding.backend_profile() != witness.profile() {
            return Err(BackendQueueExecutionTicketDenial::BackendProfileMismatch);
        }
        if binding.backend_evidence_class() != witness.evidence_class() {
            return Err(BackendQueueExecutionTicketDenial::BackendEvidenceClassMismatch);
        }
        Ok(BackendQueueExecutionTicket::from_backend_authority(
            binding,
            BackendQueueExecutionPosture::from_admitted_capability_unchecked(witness, adaptation),
        ))
    }
}

impl BackendQueueExecutionTicket {
    pub(crate) const fn from_backend_authority(
        binding: BackendQueueExecutionPlanBinding,
        posture: BackendQueueExecutionPosture,
    ) -> Self {
        Self { binding, posture }
    }

    pub const fn binding(self) -> BackendQueueExecutionPlanBinding {
        self.binding
    }

    pub const fn posture(self) -> BackendQueueExecutionPosture {
        self.posture
    }

    pub const fn begin_completion(self) -> BackendQueueExecutionCompletionBuilder {
        BackendQueueExecutionCompletionBuilder {
            ticket: self,
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
}

impl BackendQueueExecutionCompletionBuilder {
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

    pub const fn complete(self) -> BackendQueueExecutionCompletion {
        BackendQueueExecutionCompletion::from_backend_ticket(
            self.ticket.binding,
            self.ticket.posture,
            self.queue_depth_sample,
            self.read_ahead_units,
            self.read_ahead_scope,
            self.write_back_units,
            self.write_back_scope,
            self.mechanical_retries,
            self.partial_read_events,
            self.short_write_events,
            self.backpressure,
            self.foreground_wait_events,
        )
    }
}

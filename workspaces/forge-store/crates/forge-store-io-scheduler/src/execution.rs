use crate::IoQueueResourceEnvelope;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IoQueueCounterSnapshot {
    peak_queue_depth: u32,
    interference_events: u32,
}

impl IoQueueCounterSnapshot {
    pub const fn peak_queue_depth(self) -> u32 {
        self.peak_queue_depth
    }

    pub const fn interference_events(self) -> u32 {
        self.interference_events
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoQueueExecutionRecorder {
    envelope: IoQueueResourceEnvelope,
    counters: IoQueueCounterSnapshot,
    denial: Option<IoQueueExecutionDenial>,
}

impl IoQueueExecutionRecorder {
    pub const fn from_envelope(envelope: IoQueueResourceEnvelope) -> Self {
        Self {
            envelope,
            counters: IoQueueCounterSnapshot {
                peak_queue_depth: 0,
                interference_events: 0,
            },
            denial: None,
        }
    }

    pub fn observe_queue_depth(&mut self, depth: u32) -> Result<(), IoQueueExecutionDenial> {
        if let Some(denial) = self.denial {
            return Err(denial);
        }
        if depth > self.envelope.max_queue_depth() {
            let denial = IoQueueExecutionDenial::QueueDepthExceeded {
                maximum: self.envelope.max_queue_depth(),
                actual: depth,
            };
            self.denial = Some(denial);
            return Err(denial);
        }
        self.counters.peak_queue_depth = self.counters.peak_queue_depth.max(depth);
        Ok(())
    }

    pub fn record_interference_event(&mut self) -> Result<(), IoQueueExecutionDenial> {
        if let Some(denial) = self.denial {
            return Err(denial);
        }
        let Some(next) = self.counters.interference_events.checked_add(1) else {
            self.denial = Some(IoQueueExecutionDenial::InterferenceCounterOverflow);
            return Err(IoQueueExecutionDenial::InterferenceCounterOverflow);
        };
        if next > self.envelope.max_interference_events() {
            let denial = IoQueueExecutionDenial::InterferenceEventsExceeded {
                maximum: self.envelope.max_interference_events(),
                actual: next,
            };
            self.denial = Some(denial);
            return Err(denial);
        }
        self.counters.interference_events = next;
        Ok(())
    }

    pub fn executed_evidence(
        self,
    ) -> Result<IoQueueExecutedEvidenceSource, IoQueueExecutionDenial> {
        if let Some(denial) = self.denial {
            return Err(denial);
        }
        Ok(IoQueueExecutedEvidenceSource {
            counters: self.counters,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoQueueExecutedEvidenceSource {
    counters: IoQueueCounterSnapshot,
}

impl IoQueueExecutedEvidenceSource {
    pub const fn counters(self) -> IoQueueCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoQueueExecutionDenial {
    QueueDepthExceeded { maximum: u32, actual: u32 },
    InterferenceEventsExceeded { maximum: u32, actual: u32 },
    InterferenceCounterOverflow,
}

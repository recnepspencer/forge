#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoQueueResourceEnvelope {
    max_queue_depth: u32,
    max_interference_events: u32,
}

impl IoQueueResourceEnvelope {
    pub fn bounded(
        max_queue_depth: u32,
        max_interference_events: u32,
    ) -> Result<Self, IoQueueResourceEnvelopeDenial> {
        if max_queue_depth == 0 {
            return Err(IoQueueResourceEnvelopeDenial::QueueDepthIsZero);
        }
        Ok(Self {
            max_queue_depth,
            max_interference_events,
        })
    }

    pub const fn max_queue_depth(self) -> u32 {
        self.max_queue_depth
    }

    pub const fn max_interference_events(self) -> u32 {
        self.max_interference_events
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoQueueResourceEnvelopeDenial {
    QueueDepthIsZero,
}

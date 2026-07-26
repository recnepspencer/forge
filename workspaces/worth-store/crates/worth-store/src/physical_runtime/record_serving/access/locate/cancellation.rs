use super::RecordReadSession;
use crate::physical_runtime::RecordReadObservation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordReadCancellation {
    observation: RecordReadObservation,
    unread_payload_bytes: u64,
}

impl RecordReadSession {
    /// Stops future delivery from this synchronous session.
    ///
    /// Physical work started by an earlier `open`, `read_next`, or
    /// `next_chunk` call is already terminal when that call returns.
    /// Cancellation therefore releases session-local residency/lifecycle
    /// leases and reports the undelivered logical range; it does not claim to
    /// abort a media effect.
    pub fn cancel(self) -> RecordReadCancellation {
        let observation = self.observation;
        let unread_payload_bytes = observation
            .bytes_requested()
            .saturating_sub(observation.bytes_completed());
        RecordReadCancellation {
            observation,
            unread_payload_bytes,
        }
    }
}

impl RecordReadCancellation {
    pub const fn observation(self) -> RecordReadObservation {
        self.observation
    }

    pub const fn unread_payload_bytes(self) -> u64 {
        self.unread_payload_bytes
    }

    pub const fn delivery_was_complete(self) -> bool {
        self.unread_payload_bytes == 0
    }
}

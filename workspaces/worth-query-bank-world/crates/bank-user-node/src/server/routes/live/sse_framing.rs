use axum::body::Bytes;

const MAXIMUM_SSE_EVENT_BYTES: usize = 64 * 1024;

pub(super) struct SseEventFramer {
    pending: Vec<u8>,
}

impl SseEventFramer {
    pub(super) const fn new() -> Self {
        Self {
            pending: Vec::new(),
        }
    }

    pub(super) fn push(&mut self, chunk: &[u8]) -> Result<Vec<Bytes>, SseFramingFailure> {
        self.pending.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some(end) = event_end(&self.pending) {
            if end > MAXIMUM_SSE_EVENT_BYTES {
                return Err(SseFramingFailure);
            }
            let remainder = self.pending.split_off(end);
            events.push(Bytes::from(std::mem::replace(&mut self.pending, remainder)));
        }
        if self.pending.len() > MAXIMUM_SSE_EVENT_BYTES {
            return Err(SseFramingFailure);
        }
        Ok(events)
    }

    pub(super) fn finish(self) -> Result<(), SseFramingFailure> {
        if self.pending.is_empty() {
            Ok(())
        } else {
            Err(SseFramingFailure)
        }
    }
}

#[derive(Debug)]
pub(super) struct SseFramingFailure;

fn event_end(bytes: &[u8]) -> Option<usize> {
    let lf = bytes
        .windows(2)
        .position(|window| window == b"\n\n")
        .map(|at| at + 2);
    let crlf = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|at| at + 4);
    match (lf, crlf) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(end), None) | (None, Some(end)) => Some(end),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_transport_chunks_emit_only_complete_sse_events() {
        let mut framer = SseEventFramer::new();
        assert!(framer.push(b"event: bank_account_").unwrap().is_empty());
        assert!(framer
            .push(b"activity\ndata: {\"event\":\"opened\"")
            .unwrap()
            .is_empty());
        let events = framer.push(b",\"request_id\":\"split\"}\n\n").unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0],
            "event: bank_account_activity\ndata: {\"event\":\"opened\",\"request_id\":\"split\"}\n\n"
        );
        assert!(framer.finish().is_ok());
    }

    #[test]
    fn one_transport_chunk_can_carry_multiple_complete_events() {
        let mut framer = SseEventFramer::new();
        let events = framer.push(b"data: one\n\ndata: two\r\n\r\n").unwrap();
        assert_eq!(
            events,
            [
                Bytes::from_static(b"data: one\n\n"),
                Bytes::from_static(b"data: two\r\n\r\n")
            ]
        );
        assert!(framer.finish().is_ok());
    }
}

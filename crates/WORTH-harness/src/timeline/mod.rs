use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ClockDomain {
    Logical,
    WallClock,
    Replay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExecutionPhase {
    Ingest,
    Evaluate,
    Simulate,
    Render,
    Audit,
    Settlement,
    Background,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeMarker {
    pub clock_domain: ClockDomain,
    pub sequence: u64,
    pub tick: Option<u64>,
    pub phase: Option<ExecutionPhase>,
    pub wall_time_rfc3339: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedBatch {
    pub feed_name: String,
    pub phase: Option<ExecutionPhase>,
    pub sequence_start: u64,
    pub sequence_end: u64,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FeedSequencingPolicy {
    StrictContiguous,
    AllowGaps,
    AllowOverlap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineCheckpoint {
    pub checkpoint_name: String,
    pub marker: TimeMarker,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineSession {
    pub session_name: String,
    pub clock_domain: ClockDomain,
    pub sequencing_policy: FeedSequencingPolicy,
    pub phases: Vec<ExecutionPhase>,
    pub feed_batches: Vec<FeedBatch>,
    pub checkpoints: Vec<TimelineCheckpoint>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineSessionError {
    ClockDomainMismatch,
    NonContiguousBatch,
    OverlappingBatch,
}

impl FeedBatch {
    pub fn new(feed_name: impl Into<String>, sequence_start: u64, sequence_end: u64) -> Self {
        Self {
            feed_name: feed_name.into(),
            phase: None,
            sequence_start,
            sequence_end,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_phase(mut self, phase: ExecutionPhase) -> Self {
        self.phase = Some(phase);
        self
    }
}

impl TimelineSession {
    pub fn new(session_name: impl Into<String>, clock_domain: ClockDomain) -> Self {
        Self {
            session_name: session_name.into(),
            clock_domain,
            sequencing_policy: FeedSequencingPolicy::StrictContiguous,
            phases: Vec::new(),
            feed_batches: Vec::new(),
            checkpoints: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_sequencing_policy(mut self, sequencing_policy: FeedSequencingPolicy) -> Self {
        self.sequencing_policy = sequencing_policy;
        self
    }

    pub fn with_phase(mut self, phase: ExecutionPhase) -> Self {
        self.phases.push(phase);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn push_feed_batch(mut self, batch: FeedBatch) -> Result<Self, TimelineSessionError> {
        self.validate_feed_batch(&batch)?;
        self.feed_batches.push(batch);
        Ok(self)
    }

    pub fn push_checkpoint(
        mut self,
        checkpoint_name: impl Into<String>,
        marker: TimeMarker,
    ) -> Result<Self, TimelineSessionError> {
        if marker.clock_domain != self.clock_domain {
            return Err(TimelineSessionError::ClockDomainMismatch);
        }
        self.checkpoints.push(TimelineCheckpoint {
            checkpoint_name: checkpoint_name.into(),
            marker,
            metadata: BTreeMap::new(),
        });
        Ok(self)
    }

    pub fn validate_feed_batch(&self, batch: &FeedBatch) -> Result<(), TimelineSessionError> {
        if let Some(last) = self.feed_batches.last() {
            match self.sequencing_policy {
                FeedSequencingPolicy::StrictContiguous => {
                    if batch.sequence_start != last.sequence_end + 1 {
                        return Err(TimelineSessionError::NonContiguousBatch);
                    }
                }
                FeedSequencingPolicy::AllowGaps => {
                    if batch.sequence_start <= last.sequence_end {
                        return Err(TimelineSessionError::OverlappingBatch);
                    }
                }
                FeedSequencingPolicy::AllowOverlap => {}
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ClockDomain, ExecutionPhase, FeedBatch, FeedSequencingPolicy, TimeMarker, TimelineSession,
        TimelineSessionError,
    };

    #[test]
    fn timeline_session_enforces_sequencing_policy() {
        let session = TimelineSession::new("session", ClockDomain::Logical)
            .push_feed_batch(FeedBatch::new("feed", 1, 1))
            .unwrap();
        let error = session
            .clone()
            .push_feed_batch(FeedBatch::new("feed", 3, 3))
            .unwrap_err();
        assert_eq!(error, TimelineSessionError::NonContiguousBatch);

        let permissive = session
            .with_sequencing_policy(FeedSequencingPolicy::AllowGaps)
            .push_feed_batch(FeedBatch::new("feed", 4, 4))
            .unwrap();
        assert_eq!(permissive.feed_batches.len(), 2);
    }

    #[test]
    fn timeline_session_rejects_checkpoint_clock_mismatch() {
        let error = TimelineSession::new("session", ClockDomain::Logical)
            .push_checkpoint(
                "checkpoint",
                TimeMarker {
                    clock_domain: ClockDomain::Replay,
                    sequence: 1,
                    tick: None,
                    phase: Some(ExecutionPhase::Evaluate),
                    wall_time_rfc3339: None,
                },
            )
            .unwrap_err();
        assert_eq!(error, TimelineSessionError::ClockDomainMismatch);
    }
}

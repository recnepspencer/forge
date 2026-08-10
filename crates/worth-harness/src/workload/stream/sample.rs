use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::timeline::{ExecutionPhase, FeedBatch};

use super::profile::{FeedStreamEventKind, FeedVolatilityRegime};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedStreamSample {
    pub feed_name: String,
    pub sequence: u64,
    pub value_microunits: i64,
    pub delta_microunits: i64,
    pub event_kind: FeedStreamEventKind,
    pub regime: FeedVolatilityRegime,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedStreamBatch {
    pub feed_name: String,
    pub phase: Option<ExecutionPhase>,
    pub sequence_start: u64,
    pub sequence_end: u64,
    pub samples: Vec<FeedStreamSample>,
    pub metadata: BTreeMap<String, String>,
}

impl FeedStreamBatch {
    pub fn as_feed_batch(&self) -> FeedBatch {
        let mut feed_batch = FeedBatch::new(
            self.feed_name.clone(),
            self.sequence_start,
            self.sequence_end,
        );
        if let Some(phase) = self.phase {
            feed_batch = feed_batch.with_phase(phase);
        }
        feed_batch.metadata.extend(self.metadata.clone());
        feed_batch
            .metadata
            .insert("sample_count".to_owned(), self.samples.len().to_string());
        if let Some(last) = self.samples.last() {
            feed_batch.metadata.insert(
                "last_value_microunits".to_owned(),
                last.value_microunits.to_string(),
            );
            feed_batch.metadata.insert(
                "last_event_kind".to_owned(),
                format!("{:?}", last.event_kind),
            );
            feed_batch
                .metadata
                .insert("last_regime".to_owned(), format!("{:?}", last.regime));
        }
        feed_batch
    }
}

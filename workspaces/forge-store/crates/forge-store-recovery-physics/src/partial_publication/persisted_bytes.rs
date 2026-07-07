use std::str;

use crate::{LogSequenceNumber, PartialPublicationCrashEdge, WalLsnRange};

use super::PartialPublicationObservedSource;

const FORMAT_VERSION: &str = "forge-store.partial-publication.v1";
const BEFORE_WAL_APPEND: &str = "before-wal-append";
const AFTER_WAL_APPEND_BEFORE_DURABILITY: &str = "after-wal-append-before-durability";
const DURING_CHECKPOINT_CUTOVER: &str = "during-checkpoint-cutover";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationPersistedBytes {
    bytes: Vec<u8>,
}

impl PartialPublicationPersistedBytes {
    pub(crate) fn from_replay_read_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    pub fn after_wal_append_before_durability(
        wal_range: WalLsnRange,
        operation_digest: impl Into<String>,
    ) -> Self {
        let start = wal_range.start().get().to_string();
        let end_exclusive = wal_range.end_exclusive().get().to_string();
        encode_persisted_record([
            FORMAT_VERSION,
            AFTER_WAL_APPEND_BEFORE_DURABILITY,
            &start,
            &end_exclusive,
            &operation_digest.into(),
        ])
    }

    pub fn during_checkpoint_cutover(checkpoint_digest: impl Into<String>) -> Self {
        encode_persisted_record([
            FORMAT_VERSION,
            DURING_CHECKPOINT_CUTOVER,
            &checkpoint_digest.into(),
        ])
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn persisted_bytes_digest(&self) -> String {
        persisted_bytes_digest(&self.bytes)
    }

    pub fn observe(&self) -> PartialPublicationObservedSource {
        decode_persisted_bytes(&self.bytes)
    }
}

fn decode_persisted_bytes(bytes: &[u8]) -> PartialPublicationObservedSource {
    let Ok(text) = str::from_utf8(bytes) else {
        return invalid_persisted_bytes(bytes);
    };
    let fields = text.split('\n').collect::<Vec<_>>();
    match fields.as_slice() {
        [FORMAT_VERSION, BEFORE_WAL_APPEND, operation_digest] => {
            PartialPublicationObservedSource::persisted_crash_edge(
                PartialPublicationCrashEdge::before_wal_append(*operation_digest),
            )
        }
        [FORMAT_VERSION, AFTER_WAL_APPEND_BEFORE_DURABILITY, start, end_exclusive, operation_digest] => {
            decode_wal_append_before_durability(start, end_exclusive, operation_digest)
        }
        [FORMAT_VERSION, DURING_CHECKPOINT_CUTOVER, checkpoint_digest] => {
            PartialPublicationObservedSource::persisted_crash_edge(
                PartialPublicationCrashEdge::during_checkpoint_cutover(*checkpoint_digest),
            )
        }
        _ => invalid_persisted_bytes(bytes),
    }
}

fn decode_wal_append_before_durability(
    start: &str,
    end_exclusive: &str,
    operation_digest: &str,
) -> PartialPublicationObservedSource {
    let Ok(start) = start.parse::<u64>() else {
        return invalid_persisted_text(operation_digest);
    };
    let Ok(end_exclusive) = end_exclusive.parse::<u64>() else {
        return invalid_persisted_text(operation_digest);
    };
    let Ok(wal_range) = WalLsnRange::new(
        LogSequenceNumber::new(start),
        LogSequenceNumber::new(end_exclusive),
    ) else {
        return invalid_persisted_text(operation_digest);
    };
    PartialPublicationObservedSource::persisted_crash_edge(
        PartialPublicationCrashEdge::after_wal_append_before_durability(
            wal_range,
            operation_digest,
        ),
    )
}

fn invalid_persisted_bytes(bytes: &[u8]) -> PartialPublicationObservedSource {
    PartialPublicationObservedSource::insufficient_persisted_evidence(format!(
        "partial-publication:invalid-bytes:{}",
        bytes.len()
    ))
}

fn invalid_persisted_text(text: &str) -> PartialPublicationObservedSource {
    PartialPublicationObservedSource::insufficient_persisted_evidence(format!(
        "partial-publication:invalid-record:{text}"
    ))
}

fn encode_persisted_record<'a>(
    fields: impl IntoIterator<Item = &'a str>,
) -> PartialPublicationPersistedBytes {
    PartialPublicationPersistedBytes {
        bytes: fields
            .into_iter()
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes(),
    }
}

fn persisted_bytes_digest(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push_str(&format!("{byte:02x}"));
    }
    format!(
        "partial-publication-persisted-bytes:v1:len={}:hex={encoded}",
        bytes.len()
    )
}

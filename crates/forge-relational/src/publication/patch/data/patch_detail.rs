use crate::payloads::data::{canonicalize_json, RecordPayload};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchCompatibilityClass {
    StructuredCompatible,
    DenseCompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchDetail {
    StructuredJson(Value),
    Payload(RecordPayload),
    DenseBitset(Vec<u64>),
}

impl PatchDetail {
    pub fn canonicalized(&self) -> Self {
        match self {
            Self::StructuredJson(value) => Self::StructuredJson(canonicalize_json(value)),
            Self::Payload(payload) => Self::Payload(payload.canonicalized()),
            Self::DenseBitset(bits) => Self::DenseBitset(bits.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchFragmentBudget {
    pub worker_local_fragments: bool,
    pub deterministic_merge_required: bool,
}

impl Default for PatchFragmentBudget {
    fn default() -> Self {
        Self {
            worker_local_fragments: true,
            deterministic_merge_required: true,
        }
    }
}

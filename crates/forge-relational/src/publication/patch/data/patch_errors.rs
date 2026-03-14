use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchStreamReadErrorClass {
    UnknownResumePosition,
    InvalidBatchSize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStreamReadError {
    pub class: PatchStreamReadErrorClass,
    pub detail: String,
}

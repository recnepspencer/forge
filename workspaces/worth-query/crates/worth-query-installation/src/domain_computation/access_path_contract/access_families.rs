use worth_foundational::facade::{AspectContract, AspectKey};

use super::WorthQueryArtifactNativeAlignment;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactRowBatchPosture {
    Denied,
    Borrowed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactFieldSlicePosture {
    Denied,
    Borrowed,
    ProviderNativeProjectionOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactChunkContract {
    max_rows: usize,
}

impl WorthQueryArtifactChunkContract {
    pub const fn bounded(max_rows: usize) -> Self {
        Self { max_rows }
    }

    pub const fn max_rows(self) -> usize {
        self.max_rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactBulkProjectionContract {
    identity: String,
    source_fields: Vec<AspectKey>,
    destination_alignment: WorthQueryArtifactNativeAlignment,
    destination_fields: Vec<AspectContract>,
}

impl WorthQueryArtifactBulkProjectionContract {
    pub fn new(
        identity: impl Into<String>,
        source_fields: impl IntoIterator<Item = AspectKey>,
        destination_alignment: WorthQueryArtifactNativeAlignment,
        destination_fields: impl IntoIterator<Item = AspectContract>,
    ) -> Self {
        Self {
            identity: identity.into(),
            source_fields: source_fields.into_iter().collect(),
            destination_alignment,
            destination_fields: destination_fields.into_iter().collect(),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn source_fields(&self) -> &[AspectKey] {
        &self.source_fields
    }

    pub const fn destination_alignment(&self) -> WorthQueryArtifactNativeAlignment {
        self.destination_alignment
    }

    pub fn destination_fields(&self) -> &[AspectContract] {
        &self.destination_fields
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactScalarFallbackPosture {
    Denied,
    Admitted {
        max_calls_per_admission: usize,
        max_call_amplification: usize,
    },
}

impl WorthQueryArtifactScalarFallbackPosture {
    pub const fn admitted(max_calls_per_admission: usize, max_call_amplification: usize) -> Self {
        Self::Admitted {
            max_calls_per_admission,
            max_call_amplification,
        }
    }
}

use std::collections::BTreeMap;

use crate::source::{WorthUiArtifactHandle, WorthUiArtifactSourceOrigin};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactProvenanceMap {
    source_origins: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSourceOrigin>,
    canonical_handle_order: Vec<WorthUiArtifactHandle>,
}

impl WorthUiArtifactProvenanceMap {
    pub(crate) fn new(
        source_origins: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSourceOrigin>,
        canonical_handle_order: Vec<WorthUiArtifactHandle>,
    ) -> Self {
        Self {
            source_origins,
            canonical_handle_order,
        }
    }

    pub(crate) fn source_origin(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> Option<&WorthUiArtifactSourceOrigin> {
        self.source_origins.get(handle)
    }

    pub(crate) fn handles(&self) -> &[WorthUiArtifactHandle] {
        &self.canonical_handle_order
    }
}

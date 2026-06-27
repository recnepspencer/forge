use std::collections::BTreeMap;

use crate::source::{WorthUiArtifactHandle, WorthUiArtifactSourceOrigin};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInspectionBasis {
    source_origins: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSourceOrigin>,
}

impl WorthUiArtifactInspectionBasis {
    pub(crate) fn new(
        source_origins: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSourceOrigin>,
    ) -> Self {
        Self { source_origins }
    }

    pub(crate) fn source_origin(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> Option<&WorthUiArtifactSourceOrigin> {
        self.source_origins.get(handle)
    }

    pub(crate) fn without_handle(&self, handle: &WorthUiArtifactHandle) -> Self {
        let mut source_origins = self.source_origins.clone();
        source_origins.remove(handle);
        Self { source_origins }
    }
}

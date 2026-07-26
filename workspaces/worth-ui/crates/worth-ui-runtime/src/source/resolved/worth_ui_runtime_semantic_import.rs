use worth_ui_dsl::{WorthUiArtifactInputProvenance, WorthUiArtifactInputReference};

/// Runtime-owned import meaning produced only while consuming a sealed DSL
/// package. Full provenance crosses the boundary once and is not duplicated in
/// the sealed package's declaration payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRuntimeSemanticImport {
    target: WorthUiArtifactInputReference,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiRuntimeSemanticImport {
    pub(crate) fn new(
        target: WorthUiArtifactInputReference,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self { target, provenance }
    }

    pub(crate) fn target(&self) -> &WorthUiArtifactInputReference {
        &self.target
    }

    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }
}

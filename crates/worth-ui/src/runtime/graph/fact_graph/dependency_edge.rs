use crate::runtime::WorthUiRuntimeFactId;

use super::WorthUiGraphFactDerivationKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiGraphDependencyEdge {
    source: WorthUiRuntimeFactId,
    target: WorthUiRuntimeFactId,
    derivation: WorthUiGraphFactDerivationKind,
}

impl WorthUiGraphDependencyEdge {
    pub(super) fn new(
        source: WorthUiRuntimeFactId,
        target: WorthUiRuntimeFactId,
        derivation: WorthUiGraphFactDerivationKind,
    ) -> Self {
        Self {
            source,
            target,
            derivation,
        }
    }

    pub fn source(&self) -> &WorthUiRuntimeFactId {
        &self.source
    }

    pub fn target(&self) -> &WorthUiRuntimeFactId {
        &self.target
    }

    pub fn derivation(&self) -> WorthUiGraphFactDerivationKind {
        self.derivation
    }
}

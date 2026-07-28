use std::rc::Rc;

use crate::declaration::{UiDeclarationArtifact, UiDeclarationAuthoredEvidenceIndex};
use crate::graph::UiGraphNodeEvidenceIndex;

use super::WorthUiPreparedApplicationGenerationIdentity;

/// Clone-only access to the exact prepared-generation truth needed by visual
/// identity tracing.
///
/// This is deliberately not an application authority. It cannot launch,
/// mutate, replace, or publish anything. Mounted retention may keep it alive
/// across replacement so the explicit inspection lane can join an old mounted
/// receipt against the declaration and evidence indexes that actually produced
/// that frame.
#[derive(Clone)]
pub(crate) struct WorthUiPreparedVisualTraceSource {
    generation: WorthUiPreparedApplicationGenerationIdentity,
    declaration_artifacts: Rc<[UiDeclarationArtifact]>,
    authored_evidence_index: Rc<UiDeclarationAuthoredEvidenceIndex>,
    graph_node_evidence_index: Rc<UiGraphNodeEvidenceIndex>,
}

impl WorthUiPreparedVisualTraceSource {
    pub(super) fn new(
        generation: WorthUiPreparedApplicationGenerationIdentity,
        declaration_artifacts: Rc<[UiDeclarationArtifact]>,
        authored_evidence_index: Rc<UiDeclarationAuthoredEvidenceIndex>,
        graph_node_evidence_index: Rc<UiGraphNodeEvidenceIndex>,
    ) -> Self {
        Self {
            generation,
            declaration_artifacts,
            authored_evidence_index,
            graph_node_evidence_index,
        }
    }

    pub(crate) fn generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation
    }

    pub(crate) fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        self.declaration_artifacts.as_ref()
    }

    pub(crate) fn authored_evidence_index(&self) -> &UiDeclarationAuthoredEvidenceIndex {
        self.authored_evidence_index.as_ref()
    }

    pub(crate) fn graph_node_evidence_index(&self) -> &UiGraphNodeEvidenceIndex {
        self.graph_node_evidence_index.as_ref()
    }

    pub(crate) fn minimum_retained_structural_bytes(&self) -> Option<usize> {
        let declaration_bytes = self.declaration_artifacts.iter().try_fold(
            self.declaration_artifacts
                .len()
                .checked_mul(std::mem::size_of::<UiDeclarationArtifact>())?,
            |bytes, artifact| bytes.checked_add(artifact.identity().authored_semantic_name().len()),
        )?;
        std::mem::size_of::<Self>()
            .checked_add(declaration_bytes)?
            .checked_add(
                self.authored_evidence_index
                    .minimum_retained_structural_bytes()?,
            )?
            .checked_add(
                self.graph_node_evidence_index
                    .minimum_retained_structural_bytes()?,
            )
    }
}

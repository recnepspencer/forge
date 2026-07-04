mod aspect_evidence_neighborhood;
mod aspect_lookup_boundary;
#[cfg(test)]
mod aspect_evidence_test_support;
mod consumed_aspect_evidence_index;
mod published_aspect_evidence_index;
#[cfg(test)]
mod aspect_evidence_index_tests;

use consumed_aspect_evidence_index::UiConsumedAspectEvidenceIndex;
use published_aspect_evidence_index::UiPublishedAspectEvidenceIndex;

use crate::graph::{UiGraphNodeEvidenceIndex, UiGraphSnapshot};
use worth_ui_inspection::UiInspectionTarget;

pub(crate) use aspect_lookup_boundary::WorthUiAspectInspectionBoundary;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphAspectEvidenceIndexes {
    published: UiPublishedAspectEvidenceIndex,
    consumed: UiConsumedAspectEvidenceIndex,
}

impl UiGraphAspectEvidenceIndexes {
    pub(crate) fn rebuild(
        graph_snapshot: &UiGraphSnapshot,
        graph_node_evidence_index: &UiGraphNodeEvidenceIndex,
    ) -> Self {
        Self {
            published: UiPublishedAspectEvidenceIndex::rebuild(
                graph_snapshot,
                graph_node_evidence_index,
            ),
            consumed: UiConsumedAspectEvidenceIndex::rebuild(
                graph_snapshot,
                graph_node_evidence_index,
            ),
        }
    }

    pub(crate) fn lookup_published_aspect(
        &self,
        canonical_label: &str,
    ) -> Option<aspect_evidence_neighborhood::UiAspectEvidenceLookup<'_>> {
        self.published.lookup(canonical_label)
    }

    pub(crate) fn lookup_consumed_aspect(
        &self,
        canonical_label: &str,
    ) -> Option<aspect_evidence_neighborhood::UiAspectEvidenceLookup<'_>> {
        self.consumed.lookup(canonical_label)
    }

    pub(crate) fn lookup_ref_target(&self, identity_digest: u64) -> Option<UiInspectionTarget> {
        self.published
            .lookup_ref_identity_digest(identity_digest)
            .map(UiInspectionTarget::published_aspect)
            .or_else(|| {
                self.consumed
                    .lookup_ref_identity_digest(identity_digest)
                    .map(UiInspectionTarget::consumed_aspect)
            })
    }
}

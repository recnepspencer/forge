use crate::evidence::UiInspectionObligationReasonProjection;
use crate::obligations::selection::{UiSelectedObligation, UiSelectedObligationSet};
use worth_ui_inspection::{
    UiInspectionEvidenceSource, UiInspectionObligationDecision, UiInspectionObligationFamily,
    UiInspectionObligationSelectionReason,
};

use super::projection_mapping::{inspection_family, inspection_source};
use super::selection_reason_mapping::inspection_selection_reason;
use super::{
    prerequisite_sources_from_refs, UiObligationEvidenceDecision, UiObligationEvidenceRecord,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSelectedObligationEvidenceProjection {
    handle_digest: u64,
    graph_node_digest: u64,
    touch_identity_digest: u64,
    family: UiInspectionObligationFamily,
    selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
    prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
}

impl UiSelectedObligationEvidenceProjection {
    pub fn from_selected_obligation_set_entry(
        selected: &UiSelectedObligationSet,
        obligation: &UiSelectedObligation,
    ) -> Self {
        Self {
            handle_digest: obligation.evidence_handle().digest(),
            graph_node_digest: selected.touch().target().graph_node_identity().digest(),
            touch_identity_digest: selected.touch().identity_digest(),
            family: inspection_family(obligation.family()),
            selection_reasons: obligation
                .selection_reasons()
                .iter()
                .copied()
                .map(inspection_selection_reason)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            prerequisite_sources: prerequisite_sources_from_refs(
                obligation.prerequisite_evidence_refs(),
            )
            .into_iter()
            .map(inspection_source)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        }
    }

    pub fn from_selected_record(record: &UiObligationEvidenceRecord) -> Option<Self> {
        if record.decision() != UiObligationEvidenceDecision::Selected {
            return None;
        }
        Some(Self {
            handle_digest: record.handle().digest(),
            graph_node_digest: record.graph_node_digest(),
            touch_identity_digest: record.touch_identity_digest()?,
            family: inspection_family(record.family()?),
            selection_reasons: record
                .selection_reasons()
                .iter()
                .copied()
                .map(inspection_selection_reason)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            prerequisite_sources: record
                .prerequisite_sources()
                .iter()
                .copied()
                .map(inspection_source)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    pub fn from_selected_projection(
        projection: &UiInspectionObligationReasonProjection,
    ) -> Option<Self> {
        if projection.decision() != UiInspectionObligationDecision::Selected {
            return None;
        }
        Some(Self {
            handle_digest: projection.handle_digest(),
            graph_node_digest: projection.graph_node_digest(),
            touch_identity_digest: projection.touch_identity_digest()?,
            family: projection.family()?,
            selection_reasons: projection.selection_reasons().to_vec().into_boxed_slice(),
            prerequisite_sources: projection
                .prerequisite_sources()
                .to_vec()
                .into_boxed_slice(),
        })
    }

    pub fn handle_digest(&self) -> u64 {
        self.handle_digest
    }
}

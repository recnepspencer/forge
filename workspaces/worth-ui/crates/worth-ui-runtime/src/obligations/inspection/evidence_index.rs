use worth_ui_inspection::UiInspectionObligationEvidenceReceipt;

use super::{UiObligationEvidenceHandle, UiObligationEvidenceQuery, UiObligationEvidenceRecord};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationEvidenceIndex {
    records: Box<[UiObligationEvidenceRecord]>,
}

impl UiObligationEvidenceIndex {
    pub(crate) fn new(records: Box<[UiObligationEvidenceRecord]>) -> Self {
        Self { records }
    }

    pub(crate) fn empty() -> Self {
        Self::new(Box::new([]))
    }

    pub(crate) fn with_appended(&self, mut additional: Vec<UiObligationEvidenceRecord>) -> Self {
        let mut records = self.records.to_vec();
        records.append(&mut additional);
        Self::new(records.into_boxed_slice())
    }

    pub fn records(&self) -> &[UiObligationEvidenceRecord] {
        &self.records
    }

    pub fn record(
        &self,
        handle: UiObligationEvidenceHandle,
    ) -> Option<&UiObligationEvidenceRecord> {
        self.records.iter().find(|record| record.handle() == handle)
    }

    pub fn inspect(
        &self,
        query: &UiObligationEvidenceQuery,
    ) -> UiInspectionObligationEvidenceReceipt {
        let projections = self
            .records
            .iter()
            .filter(|record| {
                query
                    .handle_digest()
                    .is_none_or(|digest| record.handle().digest() == digest)
                    && query
                        .graph_node_digest()
                        .is_none_or(|digest| record.graph_node_digest() == digest)
                    && query
                        .touch_identity_digest()
                        .is_none_or(|digest| record.touch_identity_digest() == Some(digest))
                    && query
                        .family()
                        .is_none_or(|family| record.family() == Some(family))
                    && query
                        .denial_posture()
                        .is_none_or(|posture| record.denial_posture() == Some(posture))
                    && query
                        .prerequisite_source()
                        .is_none_or(|source| record.prerequisite_sources().contains(&source))
            })
            .map(UiObligationEvidenceRecord::to_projection)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        UiInspectionObligationEvidenceReceipt::new(projections)
    }
}

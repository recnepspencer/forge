use worth_ui_inspection::{UiInspectionPosture, UiInspectionQuery};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionReceipt {
    query: UiInspectionQuery,
    posture: UiInspectionPosture,
}

impl UiInspectionReceipt {
    pub(crate) fn new(query: UiInspectionQuery, posture: UiInspectionPosture) -> Self {
        Self { query, posture }
    }

    pub fn query(&self) -> &UiInspectionQuery {
        &self.query
    }

    pub fn posture(&self) -> UiInspectionPosture {
        self.posture
    }
}

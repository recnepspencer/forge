use super::row_projection::inspection_rows_for_participation;
use crate::runtime::composition_participation::projection::{
    digest_parts, WorthUiCompositionParticipationReceipt,
};
use crate::runtime::WorthUiRuntimeFactId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAccessibilityHostInspectionPosture {
    ProjectedFromRuntimeReceipt,
    UnsupportedHostApi,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAccessibilityHostInspectionRowFeature {
    Role,
    Name,
    Description,
    Enabled,
    Focusable,
    TabOrder,
    LabelFor,
    DescribedBy,
    ErrorMessage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityHostInspectionRow {
    node_id: String,
    feature: WorthUiAccessibilityHostInspectionRowFeature,
    value: Option<String>,
    posture: WorthUiAccessibilityHostInspectionPosture,
    row_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiAccessibilityHostInspectionCounters {
    inspected_node_count: usize,
    inspected_row_count: usize,
    unsupported_host_api_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAccessibilityHostInspectionReceipt {
    participation_digest: u64,
    posture: WorthUiAccessibilityHostInspectionPosture,
    rows: Vec<WorthUiAccessibilityHostInspectionRow>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    counters: WorthUiAccessibilityHostInspectionCounters,
    receipt_digest: u64,
}

impl WorthUiAccessibilityHostInspectionReceipt {
    pub(crate) fn from_participation(
        participation: &WorthUiCompositionParticipationReceipt,
    ) -> Self {
        let rows = inspection_rows_for_participation(participation);
        let consumed_facts = participation.consumed_facts().to_vec();
        let unsupported_host_api_count = rows
            .iter()
            .filter(|row| {
                row.posture() == WorthUiAccessibilityHostInspectionPosture::UnsupportedHostApi
            })
            .count();
        let posture = if unsupported_host_api_count == 0 {
            WorthUiAccessibilityHostInspectionPosture::ProjectedFromRuntimeReceipt
        } else {
            WorthUiAccessibilityHostInspectionPosture::UnsupportedHostApi
        };
        let counters = WorthUiAccessibilityHostInspectionCounters {
            inspected_node_count: participation.accessibility_nodes().len(),
            inspected_row_count: rows.len(),
            unsupported_host_api_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        };
        let receipt_digest = digest_parts(
            [
                "accessibility_host_inspection".to_owned(),
                participation.receipt_digest().to_string(),
                posture.token().to_owned(),
            ]
            .into_iter()
            .chain(rows.iter().map(|row| row.row_digest().to_string()))
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            participation_digest: participation.receipt_digest(),
            posture,
            rows,
            consumed_facts,
            counters,
            receipt_digest,
        }
    }

    pub fn participation_digest(&self) -> u64 {
        self.participation_digest
    }

    pub fn posture(&self) -> WorthUiAccessibilityHostInspectionPosture {
        self.posture
    }

    pub fn rows(&self) -> &[WorthUiAccessibilityHostInspectionRow] {
        &self.rows
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn counters(&self) -> WorthUiAccessibilityHostInspectionCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiAccessibilityHostInspectionRow {
    pub(super) fn projected(
        node_id: impl Into<String>,
        feature: WorthUiAccessibilityHostInspectionRowFeature,
        value: impl Into<Option<String>>,
    ) -> Self {
        Self::new(
            node_id,
            feature,
            value.into(),
            WorthUiAccessibilityHostInspectionPosture::ProjectedFromRuntimeReceipt,
        )
    }

    fn new(
        node_id: impl Into<String>,
        feature: WorthUiAccessibilityHostInspectionRowFeature,
        value: Option<String>,
        posture: WorthUiAccessibilityHostInspectionPosture,
    ) -> Self {
        let node_id = node_id.into();
        let row_digest = digest_parts([
            "accessibility_host_inspection_row",
            node_id.as_str(),
            feature.token(),
            value.as_deref().unwrap_or_default(),
            posture.token(),
        ]);
        Self {
            node_id,
            feature,
            value,
            posture,
            row_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn feature(&self) -> WorthUiAccessibilityHostInspectionRowFeature {
        self.feature
    }

    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    pub fn posture(&self) -> WorthUiAccessibilityHostInspectionPosture {
        self.posture
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiAccessibilityHostInspectionCounters {
    pub fn inspected_node_count(self) -> usize {
        self.inspected_node_count
    }

    pub fn inspected_row_count(self) -> usize {
        self.inspected_row_count
    }

    pub fn unsupported_host_api_count(self) -> usize {
        self.unsupported_host_api_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

impl WorthUiAccessibilityHostInspectionPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::ProjectedFromRuntimeReceipt => "projected_from_runtime_receipt",
            Self::UnsupportedHostApi => "unsupported_host_api",
        }
    }
}

impl WorthUiAccessibilityHostInspectionRowFeature {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Role => "role",
            Self::Name => "name",
            Self::Description => "description",
            Self::Enabled => "enabled",
            Self::Focusable => "focusable",
            Self::TabOrder => "tab_order",
            Self::LabelFor => "label_for",
            Self::DescribedBy => "described_by",
            Self::ErrorMessage => "error_message",
        }
    }
}

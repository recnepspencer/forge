use super::super::WorthUiBoxEdges;
use super::receipt::{
    WorthUiFlowLayoutAlign, WorthUiFlowLayoutCrossAlign, WorthUiFlowLayoutFill,
    WorthUiFlowLayoutFit, WorthUiFlowLayoutKind,
};
use super::WorthUiFlowLayoutValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiFlowLayoutAdmissionReport {
    surface_id: String,
    status: WorthUiFlowLayoutAdmissionStatus,
    counters: WorthUiFlowLayoutAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiFlowLayoutAdmissionStatus {
    Accepted(WorthUiFlowLayoutAdmissionReceipt),
    Rejected(WorthUiFlowLayoutValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiFlowLayoutAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedFlowLayoutPropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedFlowLayoutPropSet {
    kind: WorthUiFlowLayoutKind,
    gap_token: String,
    gap_points: f32,
    padding_token: String,
    padding_edges: WorthUiBoxEdges,
    align: WorthUiFlowLayoutAlign,
    cross_align: WorthUiFlowLayoutCrossAlign,
    fit: WorthUiFlowLayoutFit,
    fill: WorthUiFlowLayoutFill,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFlowLayoutAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiFlowLayoutValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiFlowLayoutValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiFlowLayoutAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiFlowLayoutAdmissionReceipt,
        counters: WorthUiFlowLayoutAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiFlowLayoutAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiFlowLayoutValueDenialSet,
        counters: WorthUiFlowLayoutAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiFlowLayoutAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiFlowLayoutAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiFlowLayoutAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiFlowLayoutAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiFlowLayoutAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiFlowLayoutValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiFlowLayoutAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedFlowLayoutPropSet,
        authored_digest: u64,
        admission_digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            prop_set,
            authored_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn prop_set(&self) -> &WorthUiValidatedFlowLayoutPropSet {
        &self.prop_set
    }

    pub fn authored_digest(&self) -> u64 {
        self.authored_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiValidatedFlowLayoutPropSet {
    pub(crate) fn new(
        kind: WorthUiFlowLayoutKind,
        gap_token: impl Into<String>,
        gap_points: f32,
        padding_token: impl Into<String>,
        padding_edges: WorthUiBoxEdges,
        align: WorthUiFlowLayoutAlign,
        cross_align: WorthUiFlowLayoutCrossAlign,
        fit: WorthUiFlowLayoutFit,
        fill: WorthUiFlowLayoutFill,
    ) -> Self {
        Self {
            kind,
            gap_token: gap_token.into(),
            gap_points,
            padding_token: padding_token.into(),
            padding_edges,
            align,
            cross_align,
            fit,
            fill,
        }
    }

    pub fn kind(&self) -> WorthUiFlowLayoutKind {
        self.kind
    }

    pub fn gap_points(&self) -> f32 {
        self.gap_points
    }

    pub fn gap_token(&self) -> &str {
        &self.gap_token
    }

    pub fn padding_points(&self) -> f32 {
        self.padding_edges.max_axis_point()
    }

    pub fn padding_edges(&self) -> WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn align(&self) -> WorthUiFlowLayoutAlign {
        self.align
    }

    pub fn cross_align(&self) -> WorthUiFlowLayoutCrossAlign {
        self.cross_align
    }

    pub fn fit(&self) -> WorthUiFlowLayoutFit {
        self.fit
    }

    pub fn fill(&self) -> WorthUiFlowLayoutFill {
        self.fill
    }
}

impl WorthUiFlowLayoutAdmissionCounters {
    pub(crate) fn new(
        schema_count: usize,
        authored_props_seen: usize,
        defaults_applied: usize,
        values_validated: usize,
        denials_emitted: usize,
    ) -> Self {
        Self {
            schema_count,
            authored_props_seen,
            defaults_applied,
            values_validated,
            denials_emitted,
        }
    }

    pub fn schema_count(self) -> usize {
        self.schema_count
    }

    pub fn authored_props_seen(self) -> usize {
        self.authored_props_seen
    }

    pub fn defaults_applied(self) -> usize {
        self.defaults_applied
    }

    pub fn values_validated(self) -> usize {
        self.values_validated
    }

    pub fn denials_emitted(self) -> usize {
        self.denials_emitted
    }
}

impl WorthUiFlowLayoutValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiFlowLayoutValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        assert!(!denials.is_empty(), "flow layout denial set is non-empty");
        Self {
            surface_id: surface_id.into(),
            denials,
            denial_set_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn denials(&self) -> &[WorthUiFlowLayoutValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

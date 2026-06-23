use super::super::WorthUiBoxEdges;
use super::receipt::{
    WorthUiPrimitiveEventContainment, WorthUiPrimitiveEventCursor, WorthUiPrimitiveHitArea,
    WorthUiPrimitivePointerCapture,
};
use super::WorthUiEventGeometryValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiEventGeometryAdmissionReport {
    surface_id: String,
    status: WorthUiEventGeometryAdmissionStatus,
    counters: WorthUiEventGeometryAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiEventGeometryAdmissionStatus {
    Accepted(WorthUiEventGeometryAdmissionReceipt),
    Rejected(WorthUiEventGeometryValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiEventGeometryAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedEventGeometryPropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedEventGeometryPropSet {
    cursor: WorthUiPrimitiveEventCursor,
    hit_area: WorthUiPrimitiveHitArea,
    hit_slop_token: String,
    hit_slop_edges: WorthUiBoxEdges,
    containment: WorthUiPrimitiveEventContainment,
    capture: WorthUiPrimitivePointerCapture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiEventGeometryAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiEventGeometryValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiEventGeometryValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiEventGeometryAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiEventGeometryAdmissionReceipt,
        counters: WorthUiEventGeometryAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiEventGeometryAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiEventGeometryValueDenialSet,
        counters: WorthUiEventGeometryAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiEventGeometryAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiEventGeometryAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiEventGeometryAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiEventGeometryAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiEventGeometryAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiEventGeometryValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiEventGeometryAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedEventGeometryPropSet,
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

    pub fn prop_set(&self) -> &WorthUiValidatedEventGeometryPropSet {
        &self.prop_set
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiValidatedEventGeometryPropSet {
    pub(crate) fn new(
        cursor: WorthUiPrimitiveEventCursor,
        hit_area: WorthUiPrimitiveHitArea,
        hit_slop_token: impl Into<String>,
        hit_slop_edges: WorthUiBoxEdges,
        containment: WorthUiPrimitiveEventContainment,
        capture: WorthUiPrimitivePointerCapture,
    ) -> Self {
        Self {
            cursor,
            hit_area,
            hit_slop_token: hit_slop_token.into(),
            hit_slop_edges,
            containment,
            capture,
        }
    }

    pub fn cursor(&self) -> WorthUiPrimitiveEventCursor {
        self.cursor
    }

    pub fn hit_area(&self) -> WorthUiPrimitiveHitArea {
        self.hit_area
    }

    pub fn hit_slop_token(&self) -> &str {
        &self.hit_slop_token
    }

    pub fn hit_slop_edges(&self) -> WorthUiBoxEdges {
        self.hit_slop_edges
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.containment
    }

    pub fn capture(&self) -> WorthUiPrimitivePointerCapture {
        self.capture
    }
}

impl WorthUiEventGeometryAdmissionCounters {
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

    pub fn denials_emitted(self) -> usize {
        self.denials_emitted
    }
}

impl WorthUiEventGeometryValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiEventGeometryValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        assert!(
            !denials.is_empty(),
            "event geometry denial set is non-empty"
        );
        Self {
            surface_id: surface_id.into(),
            denials,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiEventGeometryValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

use crate::capability::IconId;

use super::receipt::WorthUiPrimitiveContentItemKind;
use super::value::WorthUiPrimitiveContentKind;
use super::WorthUiPrimitiveContentValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentAdmissionReport {
    surface_id: String,
    status: WorthUiPrimitiveContentAdmissionStatus,
    counters: WorthUiPrimitiveContentAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitiveContentAdmissionStatus {
    Accepted(WorthUiPrimitiveContentAdmissionReceipt),
    Rejected(WorthUiPrimitiveContentValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedPrimitiveContentPropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedPrimitiveContentPropSet {
    kind: WorthUiPrimitiveContentKind,
    order: Vec<WorthUiPrimitiveContentItemKind>,
    text: String,
    icon_id: Option<IconId>,
    text_size_token: String,
    text_size_points: f32,
    icon_size_token: String,
    icon_size_points: f32,
    icon_stroke_token: String,
    icon_stroke_width_points: f32,
    spacer_size_token: String,
    spacer_size_points: f32,
    badge_text: Option<String>,
    divider_thickness_token: String,
    divider_thickness_points: f32,
    accessibility_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveContentAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
    items_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContentValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiPrimitiveContentValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiPrimitiveContentAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiPrimitiveContentAdmissionReceipt,
        counters: WorthUiPrimitiveContentAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiPrimitiveContentAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiPrimitiveContentValueDenialSet,
        counters: WorthUiPrimitiveContentAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiPrimitiveContentAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiPrimitiveContentAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiPrimitiveContentAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiPrimitiveContentAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiPrimitiveContentAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiPrimitiveContentValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiPrimitiveContentAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedPrimitiveContentPropSet,
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

    pub fn prop_set(&self) -> &WorthUiValidatedPrimitiveContentPropSet {
        &self.prop_set
    }

    pub fn authored_digest(&self) -> u64 {
        self.authored_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiValidatedPrimitiveContentPropSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: WorthUiPrimitiveContentKind,
        order: Vec<WorthUiPrimitiveContentItemKind>,
        text: impl Into<String>,
        icon_id: Option<IconId>,
        text_size_token: impl Into<String>,
        text_size_points: f32,
        icon_size_token: impl Into<String>,
        icon_size_points: f32,
        icon_stroke_token: impl Into<String>,
        icon_stroke_width_points: f32,
        spacer_size_token: impl Into<String>,
        spacer_size_points: f32,
        badge_text: Option<String>,
        divider_thickness_token: impl Into<String>,
        divider_thickness_points: f32,
        accessibility_name: Option<String>,
    ) -> Self {
        Self {
            kind,
            order,
            text: text.into(),
            icon_id,
            text_size_token: text_size_token.into(),
            text_size_points,
            icon_size_token: icon_size_token.into(),
            icon_size_points,
            icon_stroke_token: icon_stroke_token.into(),
            icon_stroke_width_points,
            spacer_size_token: spacer_size_token.into(),
            spacer_size_points,
            badge_text,
            divider_thickness_token: divider_thickness_token.into(),
            divider_thickness_points,
            accessibility_name,
        }
    }

    pub fn kind(&self) -> WorthUiPrimitiveContentKind {
        self.kind
    }

    pub fn order(&self) -> &[WorthUiPrimitiveContentItemKind] {
        &self.order
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn icon_id(&self) -> Option<&IconId> {
        self.icon_id.as_ref()
    }

    pub fn text_size_token(&self) -> &str {
        &self.text_size_token
    }

    pub fn text_size_points(&self) -> f32 {
        self.text_size_points
    }

    pub fn icon_size_token(&self) -> &str {
        &self.icon_size_token
    }

    pub fn icon_size_points(&self) -> f32 {
        self.icon_size_points
    }

    pub fn icon_stroke_token(&self) -> &str {
        &self.icon_stroke_token
    }

    pub fn icon_stroke_width_points(&self) -> f32 {
        self.icon_stroke_width_points
    }

    pub fn spacer_size_token(&self) -> &str {
        &self.spacer_size_token
    }

    pub fn spacer_size_points(&self) -> f32 {
        self.spacer_size_points
    }

    pub fn badge_text(&self) -> Option<&str> {
        self.badge_text.as_deref()
    }

    pub fn divider_thickness_token(&self) -> &str {
        &self.divider_thickness_token
    }

    pub fn divider_thickness_points(&self) -> f32 {
        self.divider_thickness_points
    }

    pub fn accessibility_name(&self) -> Option<&str> {
        self.accessibility_name.as_deref()
    }
}

impl WorthUiPrimitiveContentAdmissionCounters {
    pub(crate) fn new(
        schema_count: usize,
        authored_props_seen: usize,
        defaults_applied: usize,
        values_validated: usize,
        denials_emitted: usize,
        items_emitted: usize,
    ) -> Self {
        Self {
            schema_count,
            authored_props_seen,
            defaults_applied,
            values_validated,
            denials_emitted,
            items_emitted,
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

    pub fn items_emitted(self) -> usize {
        self.items_emitted
    }
}

impl WorthUiPrimitiveContentValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiPrimitiveContentValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        assert!(!denials.is_empty(), "content denial set is non-empty");
        Self {
            surface_id: surface_id.into(),
            denials,
            denial_set_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn denials(&self) -> &[WorthUiPrimitiveContentValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

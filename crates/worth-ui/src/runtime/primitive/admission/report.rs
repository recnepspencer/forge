use crate::runtime::WorthUiPrimitiveValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePropAdmissionReport {
    surface_id: String,
    status: WorthUiPrimitivePropAdmissionStatus,
    counters: WorthUiPrimitivePropAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitivePropAdmissionStatus {
    Accepted(WorthUiPrimitivePropAdmissionReceipt),
    Rejected(WorthUiPrimitiveValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePropAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedPrimitivePropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedPrimitivePropSet {
    text: String,
    align: crate::runtime::WorthUiPrimitiveAlign,
    padding_token: String,
    radius_token: String,
    background_color: crate::runtime::WorthUiPrimitiveColor,
    foreground_color: crate::runtime::WorthUiPrimitiveColor,
    interaction_kind: crate::runtime::WorthUiPrimitiveInteractionKind,
    cursor: crate::runtime::WorthUiPrimitiveCursorPosture,
    focus: crate::runtime::WorthUiPrimitiveFocusPosture,
    disabled: bool,
    selected: bool,
    interaction_id: String,
    submit_payload: String,
    motion_kind: crate::runtime::WorthUiPrimitiveMotionKind,
    motion_target: crate::runtime::WorthUiPrimitiveMotionTarget,
    motion_duration_token: String,
    motion_easing: crate::runtime::WorthUiPrimitiveMotionEasing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitivePropAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiPrimitiveValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiPrimitivePropAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiPrimitivePropAdmissionReceipt,
        counters: WorthUiPrimitivePropAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let surface_id = surface_id.into();
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id,
            status: WorthUiPrimitivePropAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiPrimitiveValueDenialSet,
        counters: WorthUiPrimitivePropAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let surface_id = surface_id.into();
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id,
            status: WorthUiPrimitivePropAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiPrimitivePropAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiPrimitivePropAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiPrimitivePropAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiPrimitivePropAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiPrimitiveValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiPrimitivePropAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedPrimitivePropSet,
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

    pub fn prop_set(&self) -> &WorthUiValidatedPrimitivePropSet {
        &self.prop_set
    }

    pub fn authored_digest(&self) -> u64 {
        self.authored_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiValidatedPrimitivePropSet {
    pub(crate) fn new(
        text: String,
        align: crate::runtime::WorthUiPrimitiveAlign,
        padding_token: String,
        radius_token: String,
        background_color: crate::runtime::WorthUiPrimitiveColor,
        foreground_color: crate::runtime::WorthUiPrimitiveColor,
        interaction_kind: crate::runtime::WorthUiPrimitiveInteractionKind,
        cursor: crate::runtime::WorthUiPrimitiveCursorPosture,
        focus: crate::runtime::WorthUiPrimitiveFocusPosture,
        disabled: bool,
        selected: bool,
        interaction_id: String,
        submit_payload: String,
        motion_kind: crate::runtime::WorthUiPrimitiveMotionKind,
        motion_target: crate::runtime::WorthUiPrimitiveMotionTarget,
        motion_duration_token: String,
        motion_easing: crate::runtime::WorthUiPrimitiveMotionEasing,
    ) -> Self {
        Self {
            text,
            align,
            padding_token,
            radius_token,
            background_color,
            foreground_color,
            interaction_kind,
            cursor,
            focus,
            disabled,
            selected,
            interaction_id,
            submit_payload,
            motion_kind,
            motion_target,
            motion_duration_token,
            motion_easing,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn align(&self) -> crate::runtime::WorthUiPrimitiveAlign {
        self.align
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn radius_token(&self) -> &str {
        &self.radius_token
    }

    pub fn background_color(&self) -> crate::runtime::WorthUiPrimitiveColor {
        self.background_color
    }

    pub fn foreground_color(&self) -> crate::runtime::WorthUiPrimitiveColor {
        self.foreground_color
    }

    pub fn interaction_kind(&self) -> crate::runtime::WorthUiPrimitiveInteractionKind {
        self.interaction_kind
    }

    pub fn cursor(&self) -> crate::runtime::WorthUiPrimitiveCursorPosture {
        self.cursor
    }

    pub fn focus(&self) -> crate::runtime::WorthUiPrimitiveFocusPosture {
        self.focus
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn submit_payload(&self) -> &str {
        &self.submit_payload
    }

    pub fn motion_kind(&self) -> crate::runtime::WorthUiPrimitiveMotionKind {
        self.motion_kind
    }

    pub fn motion_target(&self) -> crate::runtime::WorthUiPrimitiveMotionTarget {
        self.motion_target
    }

    pub fn motion_duration_token(&self) -> &str {
        &self.motion_duration_token
    }

    pub fn motion_easing(&self) -> crate::runtime::WorthUiPrimitiveMotionEasing {
        self.motion_easing
    }
}

impl WorthUiPrimitivePropAdmissionCounters {
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

impl WorthUiPrimitiveValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiPrimitiveValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        assert!(
            !denials.is_empty(),
            "primitive value denial set must be non-empty"
        );
        Self {
            surface_id: surface_id.into(),
            denials,
            denial_set_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn denials(&self) -> &[WorthUiPrimitiveValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

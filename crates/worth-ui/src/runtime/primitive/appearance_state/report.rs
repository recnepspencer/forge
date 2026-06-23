use super::digest::state_digest_basis;
use super::receipt::{WorthUiAppearanceStateFieldSet, WorthUiStatefulAppearanceRecipeReceipt};
use super::WorthUiAppearanceStateValueDenialReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateAdmissionReport {
    surface_id: String,
    status: WorthUiAppearanceStateAdmissionStatus,
    counters: WorthUiAppearanceStateAdmissionCounters,
    schema_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiAppearanceStateAdmissionStatus {
    Accepted(WorthUiAppearanceStateAdmissionReceipt),
    Rejected(WorthUiAppearanceStateValueDenialSet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateAdmissionReceipt {
    surface_id: String,
    prop_set: WorthUiValidatedAppearanceStatePropSet,
    authored_digest: u64,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiValidatedAppearanceStatePropSet {
    rest: WorthUiAppearanceStateFieldSet,
    hover: WorthUiAppearanceStateFieldSet,
    pressed: WorthUiAppearanceStateFieldSet,
    focus: WorthUiAppearanceStateFieldSet,
    disabled: WorthUiAppearanceStateFieldSet,
    selected: WorthUiAppearanceStateFieldSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceStateAdmissionCounters {
    schema_count: usize,
    authored_props_seen: usize,
    defaults_applied: usize,
    values_validated: usize,
    denials_emitted: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAppearanceStateValueDenialSet {
    surface_id: String,
    denials: Vec<WorthUiAppearanceStateValueDenialReceipt>,
    denial_set_digest: u64,
}

impl WorthUiAppearanceStateAdmissionReport {
    pub(crate) fn accepted(
        surface_id: impl Into<String>,
        receipt: WorthUiAppearanceStateAdmissionReceipt,
        counters: WorthUiAppearanceStateAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = receipt.admission_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiAppearanceStateAdmissionStatus::Accepted(receipt),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub(crate) fn rejected(
        surface_id: impl Into<String>,
        denial_set: WorthUiAppearanceStateValueDenialSet,
        counters: WorthUiAppearanceStateAdmissionCounters,
        schema_digest: u64,
    ) -> Self {
        let admission_digest = denial_set.denial_set_digest();
        Self {
            surface_id: surface_id.into(),
            status: WorthUiAppearanceStateAdmissionStatus::Rejected(denial_set),
            counters,
            schema_digest,
            admission_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn status(&self) -> &WorthUiAppearanceStateAdmissionStatus {
        &self.status
    }

    pub fn counters(&self) -> WorthUiAppearanceStateAdmissionCounters {
        self.counters
    }

    pub fn schema_digest(&self) -> u64 {
        self.schema_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiAppearanceStateAdmissionStatus {
    pub fn accepted_receipt(&self) -> Option<&WorthUiAppearanceStateAdmissionReceipt> {
        match self {
            Self::Accepted(receipt) => Some(receipt),
            Self::Rejected(_) => None,
        }
    }

    pub fn denial_set(&self) -> Option<&WorthUiAppearanceStateValueDenialSet> {
        match self {
            Self::Accepted(_) => None,
            Self::Rejected(denial_set) => Some(denial_set),
        }
    }
}

impl WorthUiAppearanceStateAdmissionReceipt {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        prop_set: WorthUiValidatedAppearanceStatePropSet,
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

    pub fn prop_set(&self) -> &WorthUiValidatedAppearanceStatePropSet {
        &self.prop_set
    }

    pub fn authored_digest(&self) -> u64 {
        self.authored_digest
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiValidatedAppearanceStatePropSet {
    pub(crate) fn new(
        rest: WorthUiAppearanceStateFieldSet,
        hover: WorthUiAppearanceStateFieldSet,
        pressed: WorthUiAppearanceStateFieldSet,
        focus: WorthUiAppearanceStateFieldSet,
        disabled: WorthUiAppearanceStateFieldSet,
        selected: WorthUiAppearanceStateFieldSet,
    ) -> Self {
        Self {
            rest,
            hover,
            pressed,
            focus,
            disabled,
            selected,
        }
    }

    pub(crate) fn into_recipe(self, receipt_digest: u64) -> WorthUiStatefulAppearanceRecipeReceipt {
        WorthUiStatefulAppearanceRecipeReceipt::new(
            self.rest,
            self.hover,
            self.pressed,
            self.focus,
            self.disabled,
            self.selected,
            receipt_digest,
        )
    }

    pub fn digest_basis(&self) -> String {
        [
            state_digest_basis("rest", &self.rest),
            state_digest_basis("hover", &self.hover),
            state_digest_basis("pressed", &self.pressed),
            state_digest_basis("focus", &self.focus),
            state_digest_basis("disabled", &self.disabled),
            state_digest_basis("selected", &self.selected),
        ]
        .join("|")
    }
}

impl WorthUiAppearanceStateAdmissionCounters {
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

impl WorthUiAppearanceStateValueDenialSet {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        denials: Vec<WorthUiAppearanceStateValueDenialReceipt>,
        denial_set_digest: u64,
    ) -> Self {
        assert!(
            !denials.is_empty(),
            "appearance state denial set is non-empty"
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

    pub fn denials(&self) -> &[WorthUiAppearanceStateValueDenialReceipt] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

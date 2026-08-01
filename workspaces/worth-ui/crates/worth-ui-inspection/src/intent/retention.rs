use super::UiIntentEvidenceReference;

pub const UI_INTENT_INTERACTION_EVIDENCE_ENTRY_CAPACITY: usize = 64;
const _: () = assert!(UI_INTENT_INTERACTION_EVIDENCE_ENTRY_CAPACITY <= u8::MAX as usize + 1);
pub const UI_INTENT_CAUSAL_TRACE_EVIDENCE_BYTE_CAPACITY: usize =
    UI_INTENT_INTERACTION_EVIDENCE_ENTRY_CAPACITY
        * core::mem::size_of::<super::UiIntentCausalTraceEvidence>();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentEvidenceRetentionOmission {
    IdentityExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentEvidenceRetentionOutcome {
    Retained(UiIntentEvidenceReference),
    Replaced {
        retained: UiIntentEvidenceReference,
        expired: UiIntentEvidenceReference,
    },
    Omitted(UiIntentEvidenceRetentionOmission),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentEvidenceRetirementCause {
    RuntimeWithoutApplication,
    ApplicationReplacement,
    ApplicationShutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentEvidenceRetirementReport {
    cause: UiIntentEvidenceRetirementCause,
    disposed_references: usize,
    disposed_bytes: usize,
    replacements: u64,
    omissions: u64,
    active_after: usize,
}

impl UiIntentEvidenceRetirementReport {
    #[doc(hidden)]
    pub const fn new(
        cause: UiIntentEvidenceRetirementCause,
        disposed_references: usize,
        disposed_bytes: usize,
        replacements: u64,
        omissions: u64,
    ) -> Self {
        Self {
            cause,
            disposed_references,
            disposed_bytes,
            replacements,
            omissions,
            active_after: 0,
        }
    }

    pub const fn cause(self) -> UiIntentEvidenceRetirementCause {
        self.cause
    }

    pub const fn disposed_references(self) -> usize {
        self.disposed_references
    }

    pub const fn disposed_bytes(self) -> usize {
        self.disposed_bytes
    }

    pub const fn replacements(self) -> u64 {
        self.replacements
    }

    pub const fn omissions(self) -> u64 {
        self.omissions
    }

    pub const fn active_after(self) -> usize {
        self.active_after
    }
}

impl Default for UiIntentEvidenceRetirementReport {
    fn default() -> Self {
        Self::new(
            UiIntentEvidenceRetirementCause::RuntimeWithoutApplication,
            0,
            0,
            0,
            0,
        )
    }
}

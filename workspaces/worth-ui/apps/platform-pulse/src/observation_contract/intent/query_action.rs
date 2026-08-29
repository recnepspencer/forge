use serde::{Deserialize, Serialize};

use super::PlatformPulseIntentAttemptObservationReference;

/// Audience-safe projection of the exact binding-local precondition that stopped
/// a Query-backed product action before any Query work was submitted. It is not
/// a Query-owned admission denial and must not be read as one.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseQueryActionPreconditionDenial {
    SourceRevisionMismatch,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "posture", content = "evidence")]
pub enum PlatformPulseQueryActionObservation {
    Executed {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        query_source_revision: u64,
        status: String,
        query_receipt_digest: String,
        affected_live_view_ids: Vec<String>,
    },
    Denied {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        denial: PlatformPulseQueryActionPreconditionDenial,
        active_query_source_revision: u64,
        submitted_query_source_revision: u64,
    },
    Indeterminate {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
        detail: String,
    },
    CancelledBeforeEffect {
        reference: PlatformPulseIntentAttemptObservationReference,
        action_input_revision: u64,
    },
}

impl PlatformPulseQueryActionPreconditionDenial {
    pub fn from_projection(
        denial: worth_ui::facade::query_binding::WorthUiScalarProjectionActionPreconditionDenial,
    ) -> Self {
        match denial {
            worth_ui::facade::query_binding::WorthUiScalarProjectionActionPreconditionDenial::SourceRevisionMismatch => {
                Self::SourceRevisionMismatch
            }
        }
    }
}

impl PlatformPulseQueryActionObservation {
    pub fn executed(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        evidence: &worth_ui::facade::query_binding::WorthUiScalarProjectionActionEvidence,
    ) -> Self {
        Self::Executed {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            query_source_revision: evidence.source_revision(),
            status: evidence.status().to_owned(),
            query_receipt_digest: evidence.query_receipt_digest().to_owned(),
            affected_live_view_ids: evidence.affected_live_view_ids().to_vec(),
        }
    }

    pub fn denied(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        denial: worth_ui::facade::query_binding::WorthUiScalarProjectionActionPreconditionDenial,
        active_query_source_revision: u64,
        submitted_query_source_revision: u64,
    ) -> Self {
        Self::Denied {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            denial: PlatformPulseQueryActionPreconditionDenial::from_projection(denial),
            active_query_source_revision,
            submitted_query_source_revision,
        }
    }

    pub fn indeterminate(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
        detail: String,
    ) -> Self {
        Self::Indeterminate {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
            detail,
        }
    }

    pub fn cancelled_before_effect(
        reference: crate::intent::PlatformPulseActionAttemptReference,
        action_input_revision: crate::intent::PlatformPulseActionInputRevision,
    ) -> Self {
        Self::CancelledBeforeEffect {
            reference: PlatformPulseIntentAttemptObservationReference::from_product(reference),
            action_input_revision: action_input_revision.value(),
        }
    }
}

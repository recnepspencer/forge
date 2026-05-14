use crate::diagnostics::outcomes::FoundationalDiagnosticOutcomeKind;
use crate::diagnostics::primitives::{
    FoundationalDiagnosticBreachClass, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticScopeId, FoundationalDiagnosticSeverity,
};
use crate::diagnostics::rows::labels::{
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticSemanticLabelSet,
    FoundationalDiagnosticSupportEvidencePosture, FoundationalDiagnosticWidenedFalloutPosture,
};
use crate::diagnostics::subjects::{FoundationalDiagnosticLocator, FoundationalDiagnosticSubject};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FoundationalDiagnosticRowCommon {
    pub(crate) code: FoundationalDiagnosticCodeId,
    pub(crate) scope: FoundationalDiagnosticScopeId,
    pub(crate) severity: FoundationalDiagnosticSeverity,
    pub(crate) subject: FoundationalDiagnosticSubject,
    pub(crate) locator: FoundationalDiagnosticLocator,
    pub(crate) outcome_kind: FoundationalDiagnosticOutcomeKind,
    pub(crate) semantic_labels: FoundationalDiagnosticSemanticLabelSet,
}

impl FoundationalDiagnosticRowCommon {}

macro_rules! common_accessors {
    () => {
        pub fn code(&self) -> &FoundationalDiagnosticCodeId {
            &self.common.code
        }

        pub fn scope(&self) -> &FoundationalDiagnosticScopeId {
            &self.common.scope
        }

        pub const fn severity(&self) -> FoundationalDiagnosticSeverity {
            self.common.severity
        }

        pub fn subject(&self) -> &FoundationalDiagnosticSubject {
            &self.common.subject
        }

        pub fn locator(&self) -> &FoundationalDiagnosticLocator {
            &self.common.locator
        }

        pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
            self.common.outcome_kind
        }

        pub fn semantic_labels(&self) -> &FoundationalDiagnosticSemanticLabelSet {
            &self.common.semantic_labels
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticDecisionRow {
    pub(crate) common: FoundationalDiagnosticRowCommon,
    denial_class: Option<FoundationalDiagnosticDenialClass>,
    locality_claim: FoundationalDiagnosticLocalityClaim,
    widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
}

impl FoundationalDiagnosticDecisionRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: FoundationalDiagnosticCodeId,
        scope: FoundationalDiagnosticScopeId,
        severity: FoundationalDiagnosticSeverity,
        subject: FoundationalDiagnosticSubject,
        locator: FoundationalDiagnosticLocator,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        semantic_labels: FoundationalDiagnosticSemanticLabelSet,
        denial_class: Option<FoundationalDiagnosticDenialClass>,
        locality_claim: FoundationalDiagnosticLocalityClaim,
        widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
    ) -> Self {
        Self {
            common: FoundationalDiagnosticRowCommon {
                code,
                scope,
                severity,
                subject,
                locator,
                outcome_kind,
                semantic_labels,
            },
            denial_class,
            locality_claim,
            widened_fallout_posture,
        }
    }

    common_accessors!();

    pub const fn denial_class(&self) -> Option<FoundationalDiagnosticDenialClass> {
        self.denial_class
    }

    pub const fn locality_claim(&self) -> FoundationalDiagnosticLocalityClaim {
        self.locality_claim
    }

    pub const fn widened_fallout_posture(&self) -> FoundationalDiagnosticWidenedFalloutPosture {
        self.widened_fallout_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticFailureRow {
    pub(crate) common: FoundationalDiagnosticRowCommon,
    breach_class: FoundationalDiagnosticBreachClass,
    locality_claim: FoundationalDiagnosticLocalityClaim,
    widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
}

impl FoundationalDiagnosticFailureRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: FoundationalDiagnosticCodeId,
        scope: FoundationalDiagnosticScopeId,
        severity: FoundationalDiagnosticSeverity,
        subject: FoundationalDiagnosticSubject,
        locator: FoundationalDiagnosticLocator,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        semantic_labels: FoundationalDiagnosticSemanticLabelSet,
        breach_class: FoundationalDiagnosticBreachClass,
        locality_claim: FoundationalDiagnosticLocalityClaim,
        widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
    ) -> Self {
        Self {
            common: FoundationalDiagnosticRowCommon {
                code,
                scope,
                severity,
                subject,
                locator,
                outcome_kind,
                semantic_labels,
            },
            breach_class,
            locality_claim,
            widened_fallout_posture,
        }
    }

    common_accessors!();

    pub const fn breach_class(&self) -> FoundationalDiagnosticBreachClass {
        self.breach_class
    }

    pub const fn locality_claim(&self) -> FoundationalDiagnosticLocalityClaim {
        self.locality_claim
    }

    pub const fn widened_fallout_posture(&self) -> FoundationalDiagnosticWidenedFalloutPosture {
        self.widened_fallout_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticComparisonRow {
    pub(crate) common: FoundationalDiagnosticRowCommon,
    mismatch_locator: Option<FoundationalDiagnosticLocator>,
    evidence_posture: FoundationalDiagnosticEvidencePosture,
}

impl FoundationalDiagnosticComparisonRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: FoundationalDiagnosticCodeId,
        scope: FoundationalDiagnosticScopeId,
        severity: FoundationalDiagnosticSeverity,
        subject: FoundationalDiagnosticSubject,
        locator: FoundationalDiagnosticLocator,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        semantic_labels: FoundationalDiagnosticSemanticLabelSet,
        mismatch_locator: Option<FoundationalDiagnosticLocator>,
        evidence_posture: FoundationalDiagnosticEvidencePosture,
    ) -> Self {
        Self {
            common: FoundationalDiagnosticRowCommon {
                code,
                scope,
                severity,
                subject,
                locator,
                outcome_kind,
                semantic_labels,
            },
            mismatch_locator,
            evidence_posture,
        }
    }

    common_accessors!();

    pub fn mismatch_locator(&self) -> Option<&FoundationalDiagnosticLocator> {
        self.mismatch_locator.as_ref()
    }

    pub const fn evidence_posture(&self) -> FoundationalDiagnosticEvidencePosture {
        self.evidence_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticSupportRow {
    pub(crate) common: FoundationalDiagnosticRowCommon,
    evidence_posture: FoundationalDiagnosticSupportEvidencePosture,
    locality_claim: FoundationalDiagnosticLocalityClaim,
    widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
}

impl FoundationalDiagnosticSupportRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: FoundationalDiagnosticCodeId,
        scope: FoundationalDiagnosticScopeId,
        severity: FoundationalDiagnosticSeverity,
        subject: FoundationalDiagnosticSubject,
        locator: FoundationalDiagnosticLocator,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        semantic_labels: FoundationalDiagnosticSemanticLabelSet,
        evidence_posture: FoundationalDiagnosticSupportEvidencePosture,
        locality_claim: FoundationalDiagnosticLocalityClaim,
        widened_fallout_posture: FoundationalDiagnosticWidenedFalloutPosture,
    ) -> Self {
        Self {
            common: FoundationalDiagnosticRowCommon {
                code,
                scope,
                severity,
                subject,
                locator,
                outcome_kind,
                semantic_labels,
            },
            evidence_posture,
            locality_claim,
            widened_fallout_posture,
        }
    }

    common_accessors!();

    pub fn evidence_posture(&self) -> &FoundationalDiagnosticSupportEvidencePosture {
        &self.evidence_posture
    }

    pub const fn locality_claim(&self) -> FoundationalDiagnosticLocalityClaim {
        self.locality_claim
    }

    pub const fn widened_fallout_posture(&self) -> FoundationalDiagnosticWidenedFalloutPosture {
        self.widened_fallout_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticProvenanceReadyRow {
    pub(crate) common: FoundationalDiagnosticRowCommon,
    evidence_origin_locator: FoundationalDiagnosticLocator,
    evidence_posture: FoundationalDiagnosticEvidencePosture,
}

impl FoundationalDiagnosticProvenanceReadyRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: FoundationalDiagnosticCodeId,
        scope: FoundationalDiagnosticScopeId,
        severity: FoundationalDiagnosticSeverity,
        subject: FoundationalDiagnosticSubject,
        locator: FoundationalDiagnosticLocator,
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        semantic_labels: FoundationalDiagnosticSemanticLabelSet,
        evidence_origin_locator: FoundationalDiagnosticLocator,
        evidence_posture: FoundationalDiagnosticEvidencePosture,
    ) -> Self {
        Self {
            common: FoundationalDiagnosticRowCommon {
                code,
                scope,
                severity,
                subject,
                locator,
                outcome_kind,
                semantic_labels,
            },
            evidence_origin_locator,
            evidence_posture,
        }
    }

    common_accessors!();

    pub fn evidence_origin_locator(&self) -> &FoundationalDiagnosticLocator {
        &self.evidence_origin_locator
    }

    pub const fn evidence_posture(&self) -> FoundationalDiagnosticEvidencePosture {
        self.evidence_posture
    }
}

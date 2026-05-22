use forge_foundational::FoundationalDiagnosticOutcomeKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilityCategory {
    Admission,
    SupportTraceability,
    InvariantCapability,
    WorkflowPreview,
    ContinuityLineage,
    ConsequenceAftermath,
    ExplanationInspection,
}

impl ForgeQueryDomainCapabilityCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::SupportTraceability => "support-traceability",
            Self::InvariantCapability => "invariant-capability",
            Self::WorkflowPreview => "workflow-preview",
            Self::ContinuityLineage => "continuity-lineage",
            Self::ConsequenceAftermath => "consequence-aftermath",
            Self::ExplanationInspection => "explanation-inspection",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDomainCapabilitySemanticPosture {
    AdmissionAdvisory,
    AdmissionViolation,
    AdmissionSupportOnly,
    SupportDeclarationSupport,
    SupportDeclarationTraceability,
    SupportNarrowedSupport,
    InvariantCapabilityGap,
    InvariantDenial,
    InvariantSupportSummary,
    InvariantRegistration,
    WorkflowPreviewOnly,
    WorkflowPromotionEligible,
    WorkflowConfirmationRequired,
    WorkflowDiscardRequired,
    ContinuityPreserved,
    ContinuitySplit,
    ContinuityReplaced,
    ContinuityCorrespondenceOnly,
    AftermathEstablishesFact,
    AftermathConsumesFact,
    AftermathDeclaresResidue,
    ExplanationRequiresContext,
    ExplanationExplainsFallback,
    ExplanationExplainsAmbiguity,
}

impl ForgeQueryDomainCapabilitySemanticPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmissionAdvisory => "advisory",
            Self::AdmissionViolation => "violation",
            Self::AdmissionSupportOnly => "support-only",
            Self::SupportDeclarationSupport => "declaration-support",
            Self::SupportDeclarationTraceability => "declaration-traceability",
            Self::SupportNarrowedSupport => "narrowed-support",
            Self::InvariantCapabilityGap => "capability-gap",
            Self::InvariantDenial => "invariant-denial",
            Self::InvariantSupportSummary => "support-summary",
            Self::InvariantRegistration => "invariant-registration",
            Self::WorkflowPreviewOnly => "preview-only",
            Self::WorkflowPromotionEligible => "promotion-eligible",
            Self::WorkflowConfirmationRequired => "confirmation-required",
            Self::WorkflowDiscardRequired => "discard-required",
            Self::ContinuityPreserved => "preserved",
            Self::ContinuitySplit => "split",
            Self::ContinuityReplaced => "replaced",
            Self::ContinuityCorrespondenceOnly => "correspondence-only",
            Self::AftermathEstablishesFact => "establishes-fact",
            Self::AftermathConsumesFact => "consumes-fact",
            Self::AftermathDeclaresResidue => "declares-residue",
            Self::ExplanationRequiresContext => "requires-context",
            Self::ExplanationExplainsFallback => "explains-fallback",
            Self::ExplanationExplainsAmbiguity => "explains-ambiguity",
        }
    }

    pub const fn outcome_kind(self) -> FoundationalDiagnosticOutcomeKind {
        match self {
            Self::AdmissionAdvisory => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::AdmissionViolation => FoundationalDiagnosticOutcomeKind::Violation,
            Self::AdmissionSupportOnly => FoundationalDiagnosticOutcomeKind::Partial,
            Self::SupportDeclarationSupport => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::SupportDeclarationTraceability => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::SupportNarrowedSupport => FoundationalDiagnosticOutcomeKind::Partial,
            Self::InvariantCapabilityGap => FoundationalDiagnosticOutcomeKind::Unsupported,
            Self::InvariantDenial => FoundationalDiagnosticOutcomeKind::Violation,
            Self::InvariantSupportSummary => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::InvariantRegistration => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::WorkflowPreviewOnly => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::WorkflowPromotionEligible => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::WorkflowConfirmationRequired => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::WorkflowDiscardRequired => FoundationalDiagnosticOutcomeKind::Denied,
            Self::ContinuityPreserved => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::ContinuitySplit => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::ContinuityReplaced => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::ContinuityCorrespondenceOnly => FoundationalDiagnosticOutcomeKind::Partial,
            Self::AftermathEstablishesFact => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::AftermathConsumesFact => FoundationalDiagnosticOutcomeKind::Accepted,
            Self::AftermathDeclaresResidue => FoundationalDiagnosticOutcomeKind::Partial,
            Self::ExplanationRequiresContext => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::ExplanationExplainsFallback => FoundationalDiagnosticOutcomeKind::Advisory,
            Self::ExplanationExplainsAmbiguity => FoundationalDiagnosticOutcomeKind::Advisory,
        }
    }

    pub const fn is_policy_or_inferred(self) -> bool {
        !matches!(
            self,
            Self::AdmissionAdvisory
                | Self::AdmissionViolation
                | Self::AdmissionSupportOnly
                | Self::SupportDeclarationSupport
                | Self::InvariantRegistration
        )
    }
}

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed as SealedPayload;

pub trait ForgeQueryDomainCapabilityPayload: Clone + SealedPayload {
    fn category(&self) -> ForgeQueryDomainCapabilityCategory;
    fn posture_label(&self) -> &'static str;
    fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture;
    fn semantic_code(&self) -> &str;
    fn detail(&self) -> &str;
    fn payload_digest(&self) -> &str;
}

macro_rules! define_payload_family {
    (
        $posture:ident,
        $payload:ident,
        $category:expr,
        { $($variant:ident => $label:literal),+ $(,)? }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $posture {
            $($variant),+
        }

        impl $posture {
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $label),+
                }
            }
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $payload {
            posture: $posture,
            semantic_code: String,
            detail: String,
            payload_digest: String,
        }

        impl $payload {
            pub fn new(
                posture: $posture,
                semantic_code: impl Into<String>,
                detail: impl Into<String>,
            ) -> Self {
                let semantic_code = semantic_code.into();
                let detail = detail.into();
                let payload_digest = $crate::identity::hash_parts(&[
                    "forge_query_domain_capability_payload_v1".to_string(),
                    format!("category:{}", $category.as_str()),
                    format!("posture:{}", posture.as_str()),
                    format!("semantic_code:{semantic_code}"),
                    format!("detail:{detail}"),
                ]);
                Self {
                    posture,
                    semantic_code,
                    detail,
                    payload_digest,
                }
            }

            pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
                $category
            }

            pub fn posture(&self) -> $posture {
                self.posture
            }

            pub fn semantic_code(&self) -> &str {
                &self.semantic_code
            }

            pub fn detail(&self) -> &str {
                &self.detail
            }

            pub fn payload_digest(&self) -> &str {
                &self.payload_digest
            }
        }

        impl $crate::domain_capabilities::payloads::common::SealedPayload for $payload {}

        impl $crate::domain_capabilities::payloads::common::ForgeQueryDomainCapabilityPayload
            for $payload
        {
            fn category(&self) -> ForgeQueryDomainCapabilityCategory {
                self.category()
            }

            fn posture_label(&self) -> &'static str {
                self.posture().as_str()
            }

            fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture {
                self.posture().semantic_posture()
            }

            fn semantic_code(&self) -> &str {
                self.semantic_code()
            }

            fn detail(&self) -> &str {
                self.detail()
            }

            fn payload_digest(&self) -> &str {
                self.payload_digest()
            }
        }
    };
}

pub(crate) use define_payload_family;

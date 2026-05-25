use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryContributionCategoryFamily {
    Admission,
    SupportTraceability,
    ExplanationInspection,
    WorkflowPreview,
    ContinuityLineage,
    ConsequenceAftermath,
}

impl ForgeQueryDeclarationEntryContributionCategoryFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admission => "admission",
            Self::SupportTraceability => "support_traceability",
            Self::ExplanationInspection => "explanation_inspection",
            Self::WorkflowPreview => "workflow_preview",
            Self::ContinuityLineage => "continuity_lineage",
            Self::ConsequenceAftermath => "consequence_aftermath",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryContributionTargetFamily {
    DeclarationBound,
    AdmittedPlanBound,
    LowerRuntimeBound,
}

impl ForgeQueryDeclarationEntryContributionTargetFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationBound => "declaration_bound",
            Self::AdmittedPlanBound => "admitted_plan_bound",
            Self::LowerRuntimeBound => "lower_runtime_bound",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryContributionEvidence {
    Admission(ForgeQueryDeclarationEntryContributionEvidenceRecord),
    SupportTraceability(ForgeQueryDeclarationEntryContributionEvidenceRecord),
    ExplanationInspection(ForgeQueryDeclarationEntryContributionEvidenceRecord),
    WorkflowPreview(ForgeQueryDeclarationEntryContributionEvidenceRecord),
    ContinuityLineage(ForgeQueryDeclarationEntryContributionEvidenceRecord),
    ConsequenceAftermath(ForgeQueryDeclarationEntryContributionEvidenceRecord),
}

impl ForgeQueryDeclarationEntryContributionEvidence {
    pub fn category_family(&self) -> ForgeQueryDeclarationEntryContributionCategoryFamily {
        match self {
            Self::Admission(_) => ForgeQueryDeclarationEntryContributionCategoryFamily::Admission,
            Self::SupportTraceability(_) => {
                ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
            }
            Self::ExplanationInspection(_) => {
                ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
            }
            Self::WorkflowPreview(_) => {
                ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
            }
            Self::ContinuityLineage(_) => {
                ForgeQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
            }
            Self::ConsequenceAftermath(_) => {
                ForgeQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
            }
        }
    }

    pub fn target_family(&self) -> ForgeQueryDeclarationEntryContributionTargetFamily {
        self.record().target_family
    }

    pub fn target_digest(&self) -> &str {
        &self.record().target_digest
    }

    pub fn target_binding_digest(&self) -> &str {
        &self.record().target_binding_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.record().evidence_digest
    }

    pub fn posture_label(&self) -> &str {
        &self.record().posture_label
    }

    pub fn semantic_code(&self) -> &str {
        &self.record().semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.record().detail
    }

    pub fn decision_stage(&self) -> Option<&str> {
        self.record().decision_stage.as_deref()
    }

    fn record(&self) -> &ForgeQueryDeclarationEntryContributionEvidenceRecord {
        match self {
            Self::Admission(record)
            | Self::SupportTraceability(record)
            | Self::ExplanationInspection(record)
            | Self::WorkflowPreview(record)
            | Self::ContinuityLineage(record)
            | Self::ConsequenceAftermath(record) => record,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryContributionEvidenceRecord {
    pub(crate) target_family: ForgeQueryDeclarationEntryContributionTargetFamily,
    pub(crate) target_digest: String,
    pub(crate) target_binding_digest: String,
    pub(crate) evidence_digest: String,
    pub(crate) posture_label: String,
    pub(crate) semantic_code: String,
    pub(crate) detail: String,
    pub(crate) decision_stage: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryContributionEvidenceSet {
    evidence: Vec<ForgeQueryDeclarationEntryContributionEvidence>,
}

impl ForgeQueryDeclarationEntryContributionEvidenceSet {
    pub fn new(evidence: Vec<ForgeQueryDeclarationEntryContributionEvidence>) -> Self {
        Self { evidence }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn evidence(&self) -> &[ForgeQueryDeclarationEntryContributionEvidence] {
        &self.evidence
    }

    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryContributionComposition {
    evidence: Vec<ForgeQueryDeclarationEntryContributionEvidence>,
    contribution_digest: String,
    composed_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
    rejected_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
}

impl ForgeQueryDeclarationEntryContributionComposition {
    pub(crate) fn new(evidence: Vec<ForgeQueryDeclarationEntryContributionEvidence>) -> Self {
        let contribution_digest = crate::identity::hash_parts(
            &evidence
                .iter()
                .map(|value| value.evidence_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let composed_category_families = evidence
            .iter()
            .map(ForgeQueryDeclarationEntryContributionEvidence::category_family)
            .collect::<Vec<_>>();
        Self {
            evidence,
            contribution_digest,
            composed_category_families,
            rejected_category_families: Vec::new(),
        }
    }

    pub fn evidence(&self) -> &[ForgeQueryDeclarationEntryContributionEvidence] {
        &self.evidence
    }

    pub fn contribution_digest(&self) -> &str {
        &self.contribution_digest
    }

    pub fn composed_category_families(
        &self,
    ) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.composed_category_families
    }

    pub fn rejected_category_families(
        &self,
    ) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.rejected_category_families
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryContributionCompositionFailureClass {
    RetainedSubjectMismatch,
    TargetDigestMismatch,
    TargetFamilyTooStrong,
    CategoryNotComposableForRetainedSeam,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryContributionCompositionError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    failure_class: ForgeQueryDeclarationEntryContributionCompositionFailureClass,
    rejected_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
    reason: &'static str,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryContributionCompositionError<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        failure_class: ForgeQueryDeclarationEntryContributionCompositionFailureClass,
        rejected_category_families: Vec<ForgeQueryDeclarationEntryContributionCategoryFamily>,
        reason: &'static str,
    ) -> Self {
        Self {
            declaration_family_key,
            failure_class,
            rejected_category_families,
            reason,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn failure_class(&self) -> ForgeQueryDeclarationEntryContributionCompositionFailureClass {
        self.failure_class
    }

    pub fn rejected_category_families(
        &self,
    ) -> &[ForgeQueryDeclarationEntryContributionCategoryFamily] {
        &self.rejected_category_families
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum ForgeQueryDeclarationEntryRetainedSubjectStrength {
    Envelope,
    Relational,
    Bridge,
    Signal,
}

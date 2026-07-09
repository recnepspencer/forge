use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

mod adapter;
mod reconciliation;
mod scope;

pub(crate) use reconciliation::{
    reconcile_contribution_evidence, WorthQueryDeclarationEntryContributionReconciliationContext,
};
pub(crate) use scope::WorthQueryDeclarationEntryContributionProofScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryContributionCategoryFamily {
    Admission,
    SupportTraceability,
    ExplanationInspection,
    WorkflowPreview,
    ContinuityLineage,
    ConsequenceAftermath,
}

impl WorthQueryDeclarationEntryContributionCategoryFamily {
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
pub enum WorthQueryDeclarationEntryContributionTargetFamily {
    DeclarationBound,
    AdmittedPlanBound,
    LowerRuntimeBound,
}

impl WorthQueryDeclarationEntryContributionTargetFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationBound => "declaration_bound",
            Self::AdmittedPlanBound => "admitted_plan_bound",
            Self::LowerRuntimeBound => "lower_runtime_bound",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryContributionEvidence {
    Admission(WorthQueryDeclarationEntryContributionEvidenceRecord),
    SupportTraceability(WorthQueryDeclarationEntryContributionEvidenceRecord),
    ExplanationInspection(WorthQueryDeclarationEntryContributionEvidenceRecord),
    WorkflowPreview(WorthQueryDeclarationEntryContributionEvidenceRecord),
    ContinuityLineage(WorthQueryDeclarationEntryContributionEvidenceRecord),
    ConsequenceAftermath(WorthQueryDeclarationEntryContributionEvidenceRecord),
}

impl WorthQueryDeclarationEntryContributionEvidence {
    pub fn category_family(&self) -> WorthQueryDeclarationEntryContributionCategoryFamily {
        match self {
            Self::Admission(_) => WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
            Self::SupportTraceability(_) => {
                WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability
            }
            Self::ExplanationInspection(_) => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection
            }
            Self::WorkflowPreview(_) => {
                WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview
            }
            Self::ContinuityLineage(_) => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage
            }
            Self::ConsequenceAftermath(_) => {
                WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath
            }
        }
    }

    pub fn target_family(&self) -> WorthQueryDeclarationEntryContributionTargetFamily {
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

    fn record(&self) -> &WorthQueryDeclarationEntryContributionEvidenceRecord {
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
pub struct WorthQueryDeclarationEntryContributionEvidenceRecord {
    pub(crate) target_family: WorthQueryDeclarationEntryContributionTargetFamily,
    pub(crate) target_digest: String,
    pub(crate) target_binding_digest: String,
    pub(crate) evidence_digest: String,
    pub(crate) posture_label: String,
    pub(crate) semantic_code: String,
    pub(crate) detail: String,
    pub(crate) decision_stage: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryContributionEvidenceSet {
    evidence: Vec<WorthQueryDeclarationEntryContributionEvidence>,
}

impl WorthQueryDeclarationEntryContributionEvidenceSet {
    pub fn new(evidence: Vec<WorthQueryDeclarationEntryContributionEvidence>) -> Self {
        Self { evidence }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn evidence(&self) -> &[WorthQueryDeclarationEntryContributionEvidence] {
        &self.evidence
    }

    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryContributionComposition {
    evidence: Vec<WorthQueryDeclarationEntryContributionEvidence>,
    contribution_digest: String,
    composed_category_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    rejected_category_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
}

impl WorthQueryDeclarationEntryContributionComposition {
    pub(crate) fn new(
        evidence: Vec<WorthQueryDeclarationEntryContributionEvidence>,
        rejected_category_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    ) -> Self {
        let contribution_digest = crate::identity::hash_parts(
            &evidence
                .iter()
                .map(|value| value.evidence_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let composed_category_families = evidence
            .iter()
            .map(WorthQueryDeclarationEntryContributionEvidence::category_family)
            .collect::<Vec<_>>();
        Self {
            evidence,
            contribution_digest,
            composed_category_families,
            rejected_category_families,
        }
    }

    pub fn evidence(&self) -> &[WorthQueryDeclarationEntryContributionEvidence] {
        &self.evidence
    }

    pub fn contribution_digest(&self) -> &str {
        &self.contribution_digest
    }

    pub fn composed_category_families(
        &self,
    ) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.composed_category_families
    }

    pub fn rejected_category_families(
        &self,
    ) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.rejected_category_families
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationEntryContributionCompositionFailureClass {
    RetainedSubjectMismatch,
    TargetDigestMismatch,
    TargetFamilyTooStrong,
    CategoryNotComposableForRetainedSeam,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEntryContributionCompositionError<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    failure_class: WorthQueryDeclarationEntryContributionCompositionFailureClass,
    rejected_category_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    reason: &'static str,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryContributionCompositionError<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        failure_class: WorthQueryDeclarationEntryContributionCompositionFailureClass,
        rejected_category_families: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
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

    pub fn failure_class(&self) -> WorthQueryDeclarationEntryContributionCompositionFailureClass {
        self.failure_class
    }

    pub fn rejected_category_families(
        &self,
    ) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.rejected_category_families
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthQueryDeclarationEntryRetainedSubjectStrength {
    Envelope,
    Relational,
    Bridge,
    Signal,
}

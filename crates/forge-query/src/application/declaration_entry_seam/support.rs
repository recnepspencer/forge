use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAspectPublication,
    ForgeQueryDeclarationBridgeAuthorityAspectSummary, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRelationalAuthorityAspectSummary,
    ForgeQueryDeclarationSignalAuthorityAspectSummary, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

use super::{
    contribution::{
        reconcile_contribution_evidence, ForgeQueryDeclarationEntryContributionComposition,
        ForgeQueryDeclarationEntryContributionCompositionError,
        ForgeQueryDeclarationEntryContributionEvidenceSet,
        ForgeQueryDeclarationEntryContributionProofScope,
        ForgeQueryDeclarationEntryContributionReconciliationContext,
        ForgeQueryDeclarationEntryRetainedSubjectStrength,
    },
    digest::derive_readiness_digest,
    inspection::ForgeQueryDeclarationEntryRetainedSubjectInput,
    readiness_projection::readiness_row_for_crossing,
    retained_subject::{readiness_reconciliation_context, ReadinessRetainedPosture},
    row::{crossing_rows_for_family, ForgeQueryDeclarationEntryCrossingRow},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationEntryReadinessStatus {
    Admitted,
    Deferred,
    Unsupported,
    InvalidBasis,
}

impl ForgeQueryDeclarationEntryReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
            Self::InvalidBasis => "invalid_basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryReadinessRow {
    crossing_row: ForgeQueryDeclarationEntryCrossingRow,
    status: ForgeQueryDeclarationEntryReadinessStatus,
    reason: &'static str,
    envelope_aspect_publication: Option<ForgeQueryDeclarationAspectPublication>,
    relational_authority_summary: Option<ForgeQueryDeclarationRelationalAuthorityAspectSummary>,
    bridge_authority_summary: Option<ForgeQueryDeclarationBridgeAuthorityAspectSummary>,
    signal_authority_summary: Option<ForgeQueryDeclarationSignalAuthorityAspectSummary>,
    readiness_digest: String,
}

impl ForgeQueryDeclarationEntryReadinessRow {
    pub(crate) fn new(
        crossing_row: ForgeQueryDeclarationEntryCrossingRow,
        status: ForgeQueryDeclarationEntryReadinessStatus,
        reason: &'static str,
        envelope_aspect_publication: Option<ForgeQueryDeclarationAspectPublication>,
        relational_authority_summary: Option<ForgeQueryDeclarationRelationalAuthorityAspectSummary>,
        bridge_authority_summary: Option<ForgeQueryDeclarationBridgeAuthorityAspectSummary>,
        signal_authority_summary: Option<ForgeQueryDeclarationSignalAuthorityAspectSummary>,
    ) -> Self {
        let readiness_digest = derive_readiness_digest(&[
            format!("row:{}", crossing_row.row_digest()),
            format!("status:{}", status.as_str()),
            format!("reason:{reason}"),
            format!("envelope_publication:{envelope_aspect_publication:?}"),
            format!("relational_summary:{relational_authority_summary:?}"),
            format!("bridge_summary:{bridge_authority_summary:?}"),
            format!("signal_summary:{signal_authority_summary:?}"),
        ]);
        Self {
            crossing_row,
            status,
            reason,
            envelope_aspect_publication,
            relational_authority_summary,
            bridge_authority_summary,
            signal_authority_summary,
            readiness_digest,
        }
    }

    pub fn crossing_row(&self) -> &ForgeQueryDeclarationEntryCrossingRow {
        &self.crossing_row
    }
    pub fn status(&self) -> ForgeQueryDeclarationEntryReadinessStatus {
        self.status
    }
    pub fn reason(&self) -> &'static str {
        self.reason
    }
    pub fn envelope_aspect_publication(&self) -> Option<&ForgeQueryDeclarationAspectPublication> {
        self.envelope_aspect_publication.as_ref()
    }
    pub fn relational_authority_summary(
        &self,
    ) -> Option<&ForgeQueryDeclarationRelationalAuthorityAspectSummary> {
        self.relational_authority_summary.as_ref()
    }
    pub fn bridge_authority_summary(
        &self,
    ) -> Option<&ForgeQueryDeclarationBridgeAuthorityAspectSummary> {
        self.bridge_authority_summary.as_ref()
    }
    pub fn signal_authority_summary(
        &self,
    ) -> Option<&ForgeQueryDeclarationSignalAuthorityAspectSummary> {
        self.signal_authority_summary.as_ref()
    }
    pub fn readiness_digest(&self) -> &str {
        &self.readiness_digest
    }
}

pub struct ForgeQueryDeclarationEntryReadinessRequest<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    contribution_evidence: Option<ForgeQueryDeclarationEntryContributionEvidenceSet>,
    retained_subject: Option<ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>>,
    contribution_scope: ForgeQueryDeclarationEntryContributionProofScope,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> Default
    for ForgeQueryDeclarationEntryReadinessRequest<D, I>
{
    fn default() -> Self {
        Self {
            contribution_evidence: None,
            retained_subject: None,
            contribution_scope: ForgeQueryDeclarationEntryContributionProofScope::default(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryReadinessRequest<D, I>
{
    pub fn base() -> Self {
        Self::default()
    }

    pub fn with_contribution_evidence(
        mut self,
        evidence: ForgeQueryDeclarationEntryContributionEvidenceSet,
    ) -> Self {
        self.contribution_evidence = Some(evidence);
        self
    }

    pub fn for_retained_subject(
        mut self,
        subject: ForgeQueryDeclarationEntryRetainedSubjectInput<D, I>,
    ) -> Self {
        self.retained_subject = Some(subject);
        self
    }

    pub fn with_admitted_plan_scope(
        mut self,
        plan: crate::runtime::ForgeQueryAdmittedIntentPlan,
    ) -> Self {
        self.contribution_scope = self.contribution_scope.with_admitted_plan(plan);
        self
    }

    pub fn with_lower_runtime_boundary_scope(
        mut self,
        envelope: crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        self.contribution_scope = self
            .contribution_scope
            .with_lower_runtime_boundary(envelope);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryReadinessReport<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    rows: Vec<ForgeQueryDeclarationEntryReadinessRow>,
    contribution_composition: Option<ForgeQueryDeclarationEntryContributionComposition>,
    readiness_digest: String,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryReadinessReport<D, I>
{
    pub(crate) fn new(
        declaration_family_key: &'static str,
        rows: Vec<ForgeQueryDeclarationEntryReadinessRow>,
        contribution_composition: Option<ForgeQueryDeclarationEntryContributionComposition>,
        readiness_digest: String,
    ) -> Self {
        Self {
            declaration_family_key,
            rows,
            contribution_composition,
            readiness_digest,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn rows(&self) -> &[ForgeQueryDeclarationEntryReadinessRow] {
        &self.rows
    }
    pub fn contribution_composition(
        &self,
    ) -> Option<&ForgeQueryDeclarationEntryContributionComposition> {
        self.contribution_composition.as_ref()
    }
    pub fn readiness_digest(&self) -> &str {
        &self.readiness_digest
    }
}

pub(crate) fn forge_query_declaration_entry_readiness_report<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
) -> ForgeQueryDeclarationEntryReadinessReport<D, I> {
    match forge_query_declaration_entry_readiness_report_with_request::<D, C, I>(
        handle,
        ForgeQueryDeclarationEntryReadinessRequest::base(),
    ) {
        Ok(report) => report,
        Err(_) => {
            panic!("base readiness request should not fail contribution composition")
        }
    }
}

pub(crate) fn forge_query_declaration_entry_readiness_report_with_request<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryDeclarationEntryReadinessRequest<D, I>,
) -> Result<
    ForgeQueryDeclarationEntryReadinessReport<D, I>,
    ForgeQueryDeclarationEntryContributionCompositionError<D, I>,
> {
    let ForgeQueryDeclarationEntryReadinessRequest {
        contribution_evidence,
        retained_subject,
        contribution_scope,
        _marker: _,
    } = request;
    let reconciliation = readiness_reconciliation_context(handle, retained_subject)?;
    forge_query_declaration_entry_readiness_report_with_context::<D, C, I>(
        handle,
        contribution_evidence.as_ref(),
        reconciliation.declaration_digest.as_deref(),
        reconciliation.subject_strength,
        reconciliation.retained_posture.as_ref(),
        &contribution_scope,
    )
}

pub(crate) fn forge_query_declaration_entry_readiness_report_with_context<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    contribution_evidence: Option<&ForgeQueryDeclarationEntryContributionEvidenceSet>,
    declaration_digest: Option<&str>,
    subject_strength: ForgeQueryDeclarationEntryRetainedSubjectStrength,
    retained_posture: Option<&ReadinessRetainedPosture>,
    contribution_scope: &ForgeQueryDeclarationEntryContributionProofScope,
) -> Result<
    ForgeQueryDeclarationEntryReadinessReport<D, I>,
    ForgeQueryDeclarationEntryContributionCompositionError<D, I>,
> {
    let rows = crossing_rows_for_family::<D, C, I>(handle)
        .into_iter()
        .map(|row| readiness_row_for_crossing::<D, C, I>(row, handle, retained_posture))
        .collect::<Vec<_>>();
    let contribution_composition = reconcile_contribution_evidence::<D, I>(
        ForgeQueryDeclarationEntryContributionReconciliationContext {
            declaration_family_key: I::Family::semantic_family_key(),
            declaration_digest,
            subject_strength,
            admitted_plan_digest: contribution_scope.admitted_plan_digest(),
            lower_runtime_boundary_digest: contribution_scope.lower_runtime_boundary_digest(),
        },
        contribution_evidence,
    )?;
    let mut digest_parts = rows
        .iter()
        .map(|row| row.readiness_digest().to_string())
        .collect::<Vec<_>>();
    if let Some(subject_digest) = declaration_digest {
        digest_parts.push(format!("subject:{subject_digest}"));
    }
    if let Some(plan_digest) = contribution_scope.admitted_plan_digest() {
        digest_parts.push(format!("plan:{plan_digest}"));
    }
    if let Some(lower_runtime_digest) = contribution_scope.lower_runtime_boundary_digest() {
        digest_parts.push(format!("lower_runtime:{lower_runtime_digest}"));
    }
    if let Some(composition) = contribution_composition.as_ref() {
        digest_parts.push(format!(
            "contribution:{}",
            composition.contribution_digest()
        ));
    }
    let readiness_digest = derive_readiness_digest(&digest_parts);
    Ok(ForgeQueryDeclarationEntryReadinessReport::new(
        I::Family::semantic_family_key(),
        rows,
        contribution_composition,
        readiness_digest,
    ))
}

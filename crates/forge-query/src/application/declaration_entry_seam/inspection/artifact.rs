use crate::application::{
    ForgeQueryDeclarationBridgeContinuationFamily, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeRoutingClass, ForgeQueryDeclarationBridgeRoutingDenialCause,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationEnvelopeClass,
    ForgeQueryDeclarationEnvelopeEvidenceOrigin, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRelationalAuthorityFamily,
    ForgeQueryDeclarationRelationalRoutingClass, ForgeQueryDeclarationRelationalRoutingDenialCause,
    ForgeQueryDeclarationRelationalTruthClaim, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQueryDeclarationSignalCompatibilityClass,
    ForgeQueryDeclarationSignalCompatibilityDenialCause,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
};
use crate::basis_lifecycle::BasisFamily;

use super::super::{
    contribution::{
        ForgeQueryDeclarationEntryContributionComposition,
        ForgeQueryDeclarationEntryContributionCompositionError,
    },
    support::ForgeQueryDeclarationEntryReadinessReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryInspectionRelationalPosture {
    pub(crate) class: ForgeQueryDeclarationRelationalRoutingClass,
    pub(crate) truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    pub(crate) authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    pub(crate) routing_digest: String,
    pub(crate) denial_cause: Option<ForgeQueryDeclarationRelationalRoutingDenialCause>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryInspectionBridgePosture {
    pub(crate) class: ForgeQueryDeclarationBridgeRoutingClass,
    pub(crate) continuation_mode: ForgeQueryDeclarationBridgeContinuationMode,
    pub(crate) truth_context: ForgeQueryDeclarationBridgeTruthContext,
    pub(crate) continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
    pub(crate) routing_digest: String,
    pub(crate) denial_cause: Option<ForgeQueryDeclarationBridgeRoutingDenialCause>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEntryInspectionSignalPosture {
    pub(crate) class: ForgeQueryDeclarationSignalCompatibilityClass,
    pub(crate) execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    pub(crate) basis_families: Vec<BasisFamily>,
    pub(crate) compatibility_digest: String,
    pub(crate) denial_cause: Option<ForgeQueryDeclarationSignalCompatibilityDenialCause>,
}

pub struct ForgeQueryDeclarationEntryInspection<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) declaration_family_key: &'static str,
    pub(crate) handle_identity_digest: String,
    pub(crate) operating_context_identity_digest: String,
    pub(crate) declaration_digest: String,
    pub(crate) progression_digest: Option<String>,
    pub(crate) route_plan_digest: Option<String>,
    pub(crate) receipt_digest: Option<String>,
    pub(crate) envelope_digest: String,
    pub(crate) envelope_class: ForgeQueryDeclarationEnvelopeClass,
    pub(crate) evidence_origin: ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    pub(crate) route_denial_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    pub(crate) receipt_denial_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
    pub(crate) route_reason: Option<String>,
    pub(crate) receipt_reason: String,
    pub(crate) relational_posture: Option<ForgeQueryDeclarationEntryInspectionRelationalPosture>,
    pub(crate) bridge_posture: Option<ForgeQueryDeclarationEntryInspectionBridgePosture>,
    pub(crate) signal_posture: Option<ForgeQueryDeclarationEntryInspectionSignalPosture>,
    pub(crate) contribution_composition: Option<ForgeQueryDeclarationEntryContributionComposition>,
    pub(crate) matching_row_digests: Vec<String>,
    pub(crate) readiness: ForgeQueryDeclarationEntryReadinessReport<D, I>,
    pub(crate) inspection_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryInspection<D, I>
{
    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }
    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }
    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }
    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }
    pub fn receipt_digest(&self) -> Option<&str> {
        self.receipt_digest.as_deref()
    }
    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
    pub fn envelope_class(&self) -> ForgeQueryDeclarationEnvelopeClass {
        self.envelope_class
    }
    pub fn evidence_origin(&self) -> ForgeQueryDeclarationEnvelopeEvidenceOrigin {
        self.evidence_origin
    }
    pub fn route_denial_cause(&self) -> Option<ForgeQueryDeclarationRoutePlanDenialCause> {
        self.route_denial_cause
    }
    pub fn receipt_denial_cause(&self) -> Option<ForgeQueryDeclarationReceiptDenialCause> {
        self.receipt_denial_cause
    }
    pub fn route_reason(&self) -> Option<&str> {
        self.route_reason.as_deref()
    }
    pub fn receipt_reason(&self) -> &str {
        &self.receipt_reason
    }
    pub fn relational_posture(
        &self,
    ) -> Option<&ForgeQueryDeclarationEntryInspectionRelationalPosture> {
        self.relational_posture.as_ref()
    }
    pub fn bridge_posture(&self) -> Option<&ForgeQueryDeclarationEntryInspectionBridgePosture> {
        self.bridge_posture.as_ref()
    }
    pub fn signal_posture(&self) -> Option<&ForgeQueryDeclarationEntryInspectionSignalPosture> {
        self.signal_posture.as_ref()
    }
    pub fn contribution_composition(
        &self,
    ) -> Option<&ForgeQueryDeclarationEntryContributionComposition> {
        self.contribution_composition.as_ref()
    }
    pub fn matching_row_digests(&self) -> &[String] {
        &self.matching_row_digests
    }
    pub fn readiness(&self) -> &ForgeQueryDeclarationEntryReadinessReport<D, I> {
        &self.readiness
    }
    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

pub enum ForgeQueryDeclarationEntryInspectionError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    RetainedSubjectMismatch {
        declaration_family_key: &'static str,
        reason: &'static str,
        _marker: std::marker::PhantomData<(D, I)>,
    },
    ContributionComposition(ForgeQueryDeclarationEntryContributionCompositionError<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryInspectionError<D, I>
{
    pub(crate) fn new(declaration_family_key: &'static str, reason: &'static str) -> Self {
        Self::RetainedSubjectMismatch {
            declaration_family_key,
            reason,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        match self {
            Self::RetainedSubjectMismatch {
                declaration_family_key,
                ..
            } => declaration_family_key,
            Self::ContributionComposition(error) => error.declaration_family_key(),
        }
    }
    pub fn reason(&self) -> &'static str {
        match self {
            Self::RetainedSubjectMismatch { reason, .. } => reason,
            Self::ContributionComposition(error) => error.reason(),
        }
    }
    pub fn contribution_composition_error(
        &self,
    ) -> Option<&ForgeQueryDeclarationEntryContributionCompositionError<D, I>> {
        match self {
            Self::ContributionComposition(error) => Some(error),
            Self::RetainedSubjectMismatch { .. } => None,
        }
    }
}

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use super::inventory::CausalEvidenceFamily;
use super::observation_identity::{
    CausalEvidenceReferenceDigest, CausalEvidenceReferenceInput, CausalObservationBasisIdentity,
    CausalObservationQueryIdentity, CausalObservationReceiptIdentity,
    CausalObservationTargetHandle, CausalQueryObservationReceiptIdentity,
    CausalResultShapeContextHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryObservationReceiptFamily {
    WriteReceipt,
    IntentReceipt,
    IntentDenial,
    BranchIntentReceipt,
    PreviewOutcome,
    ReadReceipt,
    Fixture,
}

impl QueryObservationReceiptFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WriteReceipt => "write_receipt",
            Self::IntentReceipt => "intent_receipt",
            Self::IntentDenial => "intent_denial",
            Self::BranchIntentReceipt => "branch_intent_receipt",
            Self::PreviewOutcome => "preview_outcome",
            Self::ReadReceipt => "read_receipt",
            Self::Fixture => "fixture",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalObservationOutcome {
    Changed,
    Suppressed,
    Denied,
    BranchPreview,
    Replayed,
}

impl CausalObservationOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Changed => "changed",
            Self::Suppressed => "suppressed",
            Self::Denied => "denied",
            Self::BranchPreview => "branch_preview",
            Self::Replayed => "replayed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionReason {
    ChangedResult,
    SuppressedResult,
    DeniedResult,
    BranchPreviewResult,
    HistoricalReplayResult,
}

impl CausalInspectionReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChangedResult => "changed_result",
            Self::SuppressedResult => "suppressed_result",
            Self::DeniedResult => "denied_result",
            Self::BranchPreviewResult => "branch_preview_result",
            Self::HistoricalReplayResult => "historical_replay_result",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalObservationEvidenceIdentity {
    family: CausalEvidenceFamily,
    reference_digest: CausalEvidenceReferenceDigest,
    source_reference_was_empty: bool,
}

impl CausalObservationEvidenceIdentity {
    pub(in crate::runtime) fn new(
        family: CausalEvidenceFamily,
        reference_digest: impl Into<CausalEvidenceReferenceInput>,
    ) -> Self {
        let (reference_digest, source_reference_was_empty) =
            into_reference_digest(family, reference_digest.into());
        Self {
            family,
            reference_digest,
            source_reference_was_empty,
        }
    }

    pub fn family(&self) -> CausalEvidenceFamily {
        self.family
    }

    pub fn reference_digest(&self) -> &CausalEvidenceReferenceDigest {
        &self.reference_digest
    }

    pub fn source_reference_was_empty(&self) -> bool {
        self.source_reference_was_empty
    }
}

fn into_reference_digest(
    family: CausalEvidenceFamily,
    reference_input: CausalEvidenceReferenceInput,
) -> (CausalEvidenceReferenceDigest, bool) {
    match reference_input {
        CausalEvidenceReferenceInput::Typed(identity) => (identity, false),
        CausalEvidenceReferenceInput::Source(source_reference) => {
            let source_was_empty = source_reference.is_empty();
            let _ = family;
            (
                CausalEvidenceReferenceDigest::from(source_reference),
                source_was_empty,
            )
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryObservationReceipt {
    family: QueryObservationReceiptFamily,
    observation_receipt_identity: CausalObservationReceiptIdentity,
    query_identity: CausalObservationQueryIdentity,
    basis_posture: String,
    basis_identity: CausalObservationBasisIdentity,
    result_shape_context: CausalResultShapeContextHandle,
    observation_target: CausalObservationTargetHandle,
    outcome: CausalObservationOutcome,
    evidence_identities: Vec<CausalObservationEvidenceIdentity>,
    receipt_identity: CausalQueryObservationReceiptIdentity,
}

impl QueryObservationReceipt {
    pub(super) fn from_parts(parts: ObservationReceiptParts) -> Self {
        let receipt_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::CausalQueryObservationReceipt,
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), parts.family.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("observation"),
            parts.observation_receipt_identity.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("query"),
            parts.query_identity.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("basis_posture"),
            &parts.basis_posture,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("basis"),
            parts.basis_identity.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("result_shape_context"),
            parts.result_shape_context.identity().as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("observation_target"),
            parts.observation_target.identity().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("outcome"),
            parts.outcome.as_str(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("evidence"),
            parts.evidence_identities.iter().flat_map(|identity| {
                [
                    identity.family().as_str(),
                    identity.reference_digest().as_str(),
                ]
            }),
        )
        .seal()
        .into();
        Self {
            family: parts.family,
            observation_receipt_identity: parts.observation_receipt_identity,
            query_identity: parts.query_identity,
            basis_posture: parts.basis_posture,
            basis_identity: parts.basis_identity,
            result_shape_context: parts.result_shape_context,
            observation_target: parts.observation_target,
            outcome: parts.outcome,
            evidence_identities: parts.evidence_identities,
            receipt_identity,
        }
    }

    pub fn family(&self) -> QueryObservationReceiptFamily {
        self.family
    }

    pub fn observation_receipt_identity(&self) -> &CausalObservationReceiptIdentity {
        &self.observation_receipt_identity
    }

    pub fn query_identity(&self) -> &CausalObservationQueryIdentity {
        &self.query_identity
    }

    pub fn basis_posture(&self) -> &str {
        &self.basis_posture
    }

    pub fn basis_identity(&self) -> &CausalObservationBasisIdentity {
        &self.basis_identity
    }

    pub fn result_shape_context(&self) -> &CausalResultShapeContextHandle {
        &self.result_shape_context
    }

    pub fn observation_target(&self) -> &CausalObservationTargetHandle {
        &self.observation_target
    }

    pub fn outcome(&self) -> CausalObservationOutcome {
        self.outcome
    }

    pub fn evidence_identities(&self) -> &[CausalObservationEvidenceIdentity] {
        &self.evidence_identities
    }

    pub fn receipt_identity(&self) -> &CausalQueryObservationReceiptIdentity {
        &self.receipt_identity
    }
}

pub(super) struct ObservationReceiptParts {
    pub family: QueryObservationReceiptFamily,
    pub observation_receipt_identity: CausalObservationReceiptIdentity,
    pub query_identity: CausalObservationQueryIdentity,
    pub basis_posture: String,
    pub basis_identity: CausalObservationBasisIdentity,
    pub result_shape_context: CausalResultShapeContextHandle,
    pub observation_target: CausalObservationTargetHandle,
    pub outcome: CausalObservationOutcome,
    pub evidence_identities: Vec<CausalObservationEvidenceIdentity>,
}

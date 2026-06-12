use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::memory_workspace::ForgeQueryWorkspaceError;

use super::super::ForgeQueryAspectValue;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthAssertionMode {
    RetainedAuthoritativeAssertion,
    BackendVerifiedAssertion,
}

impl ForgeQueryExistingTruthAssertionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetainedAuthoritativeAssertion => "retained_authoritative_assertion",
            Self::BackendVerifiedAssertion => "backend_verified_assertion",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthAssertionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthAssertionDenialKind {
    BackendVerificationUnsupported,
    ClearAssertionUnsupported,
    MissingAssertedAspect,
    AssertedValueMismatch,
}

impl ForgeQueryExistingTruthAssertionDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendVerificationUnsupported => "backend_verification_unsupported",
            Self::ClearAssertionUnsupported => "clear_assertion_unsupported",
            Self::MissingAssertedAspect => "missing_asserted_aspect",
            Self::AssertedValueMismatch => "asserted_value_mismatch",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthAssertionDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthAssertionDenial {
    binding: crate::runtime::ForgeQueryExistingTruthTargetBinding,
    kind: ForgeQueryExistingTruthAssertionDenialKind,
    asserted_aspect_path: Option<String>,
    expected_external_value_json: Option<String>,
    found_external_value_json: Option<String>,
    message: String,
    denial_digest: String,
}

impl ForgeQueryExistingTruthAssertionDenial {
    pub fn new(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthAssertionDenialKind,
        asserted_aspect_path: Option<String>,
        expected_external_value_json: Option<String>,
        found_external_value_json: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-assertion-denial",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    binding.family().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authoritative"),
                    binding.authoritative_identity().evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    &binding.resolved_target_identity().evidence_identity(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("collection"),
                    binding.target_collection(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .optional_value(
                    ForgeQueryEvidenceTag::new("aspect"),
                    asserted_aspect_path.as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("expected"),
                    expected_external_value_json.as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("found"),
                    found_external_value_json.as_deref(),
                )
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
        Self {
            binding: binding.clone(),
            kind,
            asserted_aspect_path,
            expected_external_value_json,
            found_external_value_json,
            message,
            denial_digest,
        }
    }

    pub fn binding(&self) -> &crate::runtime::ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn kind(&self) -> ForgeQueryExistingTruthAssertionDenialKind {
        self.kind
    }

    pub fn asserted_aspect_path(&self) -> Option<&str> {
        self.asserted_aspect_path.as_deref()
    }

    pub fn expected_external_value_json(&self) -> Option<&str> {
        self.expected_external_value_json.as_deref()
    }

    pub fn found_external_value_json(&self) -> Option<&str> {
        self.found_external_value_json.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthAssertionDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "existing-truth assertion denied for authoritative `{}` during {}: {}",
            self.binding.authoritative_identity().as_str(),
            self.kind,
            self.message
        )
    }
}

impl std::error::Error for ForgeQueryExistingTruthAssertionDenial {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerifiedExistingTruthAssertion {
    asserted_aspect_count: usize,
    verification_digest: ForgeQueryEvidenceIdentity,
    verified_assumption_set: crate::runtime::ForgeQueryVerifiedAssumptionSet,
}

impl ForgeQueryVerifiedExistingTruthAssertion {
    pub fn from_snapshot_identity(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
        snapshot_identity: &ForgeQuerySnapshotIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(binding, aspects, snapshot_identity.clone())
    }

    pub(crate) fn new(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let asserted_aspect_count = aspects.len();
        let asserted_aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let cleared_assertion_count = aspects
            .iter()
            .filter(|aspect| aspect.clears_existing_value())
            .count();
        let aspect_evidence_rows = aspects
            .iter()
            .map(existing_truth_assertion_aspect_evidence)
            .collect::<Vec<_>>();
        let verified_assumption_set = crate::runtime::ForgeQueryVerifiedAssumptionSet::new(
            binding.binding_evidence_identity().clone(),
            asserted_aspect_paths,
            aspect_evidence_rows.clone(),
            cleared_assertion_count,
            snapshot_identity,
        );
        let verification_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-assertion-verification",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("binding"),
                    binding.binding_evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("assumption_set"),
                    verified_assumption_set.verified_assumption_evidence_digest(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("aspect"),
                    aspect_evidence_rows.iter(),
                )
                .seal();
        Ok(Self {
            asserted_aspect_count,
            verification_digest,
            verified_assumption_set,
        })
    }

    pub fn asserted_aspect_count(&self) -> usize {
        self.asserted_aspect_count
    }

    pub fn verification_digest(&self) -> &str {
        self.verification_digest.as_str()
    }

    pub fn verification_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.verification_digest
    }

    pub fn verified_assumption_set(&self) -> &crate::runtime::ForgeQueryVerifiedAssumptionSet {
        &self.verified_assumption_set
    }

    pub fn assumption_snapshot_digest(&self) -> &str {
        self.verified_assumption_set.assumption_snapshot_digest()
    }

    pub fn verified_precondition_digest(&self) -> &str {
        self.verified_assumption_set.verified_precondition_digest()
    }

    pub fn verification_read_set_breadth(
        &self,
    ) -> &crate::runtime::ForgeQueryVerificationReadSetBreadth {
        self.verified_assumption_set.verification_read_set_breadth()
    }
}

fn existing_truth_assertion_aspect_evidence(
    aspect: &ForgeQueryAspectValue,
) -> ForgeQueryEvidenceIdentity {
    let value_json =
        serde_json::to_string(aspect.value()).unwrap_or_else(|_| aspect.value().to_string());
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("role"),
            "existing-truth-assertion-aspect",
        )
        .field_value(
            ForgeQueryEvidenceTag::new("aspect_path"),
            aspect.aspect_path(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("clears_existing_value"),
            aspect.clears_existing_value(),
        )
        .field_value(ForgeQueryEvidenceTag::new("value"), value_json)
        .seal()
}

use crate::identity::hash_parts;
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
    expected_value_json: Option<String>,
    found_value_json: Option<String>,
    message: String,
    denial_digest: String,
}

impl ForgeQueryExistingTruthAssertionDenial {
    pub fn new(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthAssertionDenialKind,
        asserted_aspect_path: Option<String>,
        expected_value_json: Option<String>,
        found_value_json: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest = hash_parts(&[
            "forge_query_existing_truth_assertion_denial_v1".to_string(),
            format!("family:{}", binding.family()),
            format!("authoritative:{}", binding.authoritative_identity()),
            format!("resolved:{}", binding.resolved_target_identity()),
            format!("collection:{}", binding.target_collection().unwrap_or("")),
            format!("kind:{kind}"),
            format!(
                "aspect:{}",
                asserted_aspect_path.as_deref().unwrap_or("none")
            ),
            format!(
                "expected:{}",
                expected_value_json.as_deref().unwrap_or("none")
            ),
            format!("found:{}", found_value_json.as_deref().unwrap_or("none")),
            format!("message:{message}"),
        ]);
        Self {
            binding: binding.clone(),
            kind,
            asserted_aspect_path,
            expected_value_json,
            found_value_json,
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

    pub fn expected_value_json(&self) -> Option<&str> {
        self.expected_value_json.as_deref()
    }

    pub fn found_value_json(&self) -> Option<&str> {
        self.found_value_json.as_deref()
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
            self.binding.authoritative_identity(),
            self.kind,
            self.message
        )
    }
}

impl std::error::Error for ForgeQueryExistingTruthAssertionDenial {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryVerifiedExistingTruthAssertion {
    asserted_aspect_count: usize,
    verification_digest: String,
    verified_assumption_set: crate::runtime::ForgeQueryVerifiedAssumptionSet,
}

impl ForgeQueryVerifiedExistingTruthAssertion {
    pub(crate) fn new(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
        aspects: &[ForgeQueryAspectValue],
        snapshot_token: &str,
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
        let verified_assumption_set = crate::runtime::ForgeQueryVerifiedAssumptionSet::new(
            binding.binding_digest(),
            asserted_aspect_paths,
            aspects
                .iter()
                .map(|aspect| {
                    format!(
                        "{}:{}:{}",
                        aspect.aspect_path(),
                        aspect.clears_existing_value(),
                        serde_json::to_string(aspect.value())
                            .unwrap_or_else(|_| aspect.value().to_string())
                    )
                })
                .collect::<Vec<_>>(),
            cleared_assertion_count,
            snapshot_token,
        );
        let verification_digest = hash_parts(
            &std::iter::once("forge_query_existing_truth_assertion_verification_v1".to_string())
                .chain(std::iter::once(format!(
                    "binding:{}",
                    binding.binding_digest()
                )))
                .chain(std::iter::once(format!(
                    "assumption-set:{}",
                    verified_assumption_set.verified_assumption_digest()
                )))
                .chain(aspects.iter().map(|aspect| {
                    format!(
                        "{}:{}:{}",
                        aspect.aspect_path(),
                        aspect.clears_existing_value(),
                        serde_json::to_string(aspect.value())
                            .unwrap_or_else(|_| aspect.value().to_string())
                    )
                }))
                .collect::<Vec<_>>(),
        );
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

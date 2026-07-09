use sha2::{Digest, Sha256};

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct AuthorityBackedDigestLabel {
    label: String,
    source_identity: Option<WorthQueryEvidenceIdentity>,
}

impl PartialOrd for AuthorityBackedDigestLabel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AuthorityBackedDigestLabel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.label.cmp(&other.label)
    }
}

impl std::hash::Hash for AuthorityBackedDigestLabel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

impl AuthorityBackedDigestLabel {
    fn from_parts(parts: &[String]) -> Self {
        Self {
            label: digest_hash_parts(parts),
            source_identity: None,
        }
    }

    fn from_evidence_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self {
            label: identity.terminal_projection_for_reporting().to_string(),
            source_identity: Some(identity.clone()),
        }
    }

    pub fn from_domain_parts(parts: &[String]) -> Self {
        Self::from_parts(parts)
    }

    fn as_str(&self) -> &str {
        &self.label
    }

    fn evidence_identity(
        &self,
        scope: WorthQueryEvidenceScope,
        identity_family: &'static str,
        field_tag: &'static str,
    ) -> WorthQueryEvidenceIdentity {
        if let Some(source_identity) = &self.source_identity {
            return source_identity.clone();
        }
        worth_query_evidence_identity(scope)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                identity_family,
            )
            .field_value(WorthQueryEvidenceTag::new(field_tag), self.as_str())
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalQueryDigest(AuthorityBackedDigestLabel);

impl CanonicalQueryDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    pub fn from_domain_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_domain_parts(parts))
    }

    pub(crate) fn from_evidence_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self(AuthorityBackedDigestLabel::from_evidence_identity(identity))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::MutationEvidenceSourceDigest,
            "canonical_query_digest_v1",
            "canonical_query_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalResultShapeDigest(AuthorityBackedDigestLabel);

impl CanonicalResultShapeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    #[allow(dead_code)]
    pub(crate) fn from_evidence_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self(AuthorityBackedDigestLabel::from_evidence_identity(identity))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::MutationEvidenceSourceDigest,
            "canonical_result_shape_digest_v1",
            "result_shape_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SchemaBasisDigest(String);

impl SchemaBasisDigest {
    pub fn from_domain_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedQueryDigest(AuthorityBackedDigestLabel);

impl ValidatedQueryDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    pub(crate) fn from_evidence_identity(identity: &WorthQueryEvidenceIdentity) -> Self {
        Self(AuthorityBackedDigestLabel::from_evidence_identity(identity))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::MutationEvidenceSourceDigest,
            "validated_query_digest_v1",
            "validated_query_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedResultShapeDigest(String);

impl ValidatedResultShapeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlanDigest(AuthorityBackedDigestLabel);

impl PlanDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::MutationEvidenceSourceDigest,
            "execution_plan_digest_v1",
            "plan_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CollectionPlanDigest(AuthorityBackedDigestLabel);

impl CollectionPlanDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::MutationEvidenceSourceDigest,
            "collection_plan_digest_v1",
            "collection_plan_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BasisDigest(AuthorityBackedDigestLabel);

impl BasisDigest {
    #[cfg(test)]
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_parts(parts))
    }

    pub(crate) fn from_evidence_identity(
        identity: &crate::evidence_identity::WorthQueryEvidenceIdentity,
    ) -> Self {
        Self(AuthorityBackedDigestLabel::from_evidence_identity(identity))
    }

    pub fn from_domain_parts(parts: &[String]) -> Self {
        Self(AuthorityBackedDigestLabel::from_domain_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.0.evidence_identity(
            WorthQueryEvidenceScope::BasisDigest,
            "basis_digest_evidence_v1",
            "basis_digest",
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BindingFulfillmentDigest(String);

impl BindingFulfillmentDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResultDigest(String);

impl ResultDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineageDigest(String);

impl LineageDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CorrespondenceOutcomeDigest(String);

impl CorrespondenceOutcomeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CorrespondenceCostPostureDigest(String);

impl CorrespondenceCostPostureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HistoricalPathClassDigest(String);

impl HistoricalPathClassDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HistoricalCostPostureDigest(String);

impl HistoricalCostPostureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    #[allow(dead_code)]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FailureDigest(String);

impl FailureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CounterSnapshotDigest(String);

impl CounterSnapshotDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(digest_hash_parts(parts))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn hash_parts(parts: &[String]) -> String {
    digest_hash_parts(parts)
}

fn digest_hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}

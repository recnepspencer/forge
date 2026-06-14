use sha2::{Digest, Sha256};

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalQueryDigest(String);

impl CanonicalQueryDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub(crate) fn from_evidence_identity(identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self(identity.as_str().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "canonical_query_digest_v1",
            )
            .field_identity(ForgeQueryEvidenceTag::new("canonical_query_digest"), self.as_str())
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalResultShapeDigest(String);

impl CanonicalResultShapeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub(crate) fn from_evidence_identity(identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self(identity.as_str().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "canonical_result_shape_digest_v1",
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("result_shape_digest"),
                self.as_str(),
            )
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SchemaBasisDigest(String);

impl SchemaBasisDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedQueryDigest(String);

impl ValidatedQueryDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub(crate) fn from_evidence_identity(identity: &ForgeQueryEvidenceIdentity) -> Self {
        Self(identity.as_str().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "validated_query_digest_v1",
            )
            .field_identity(ForgeQueryEvidenceTag::new("validated_query_digest"), self.as_str())
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedResultShapeDigest(String);

impl ValidatedResultShapeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlanDigest(String);

impl PlanDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "execution_plan_digest_v1",
            )
            .field_identity(ForgeQueryEvidenceTag::new("plan_digest"), self.as_str())
            .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CollectionPlanDigest(String);

impl CollectionPlanDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BasisDigest(String);

impl BasisDigest {
    #[cfg(test)]
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub(crate) fn from_evidence_identity(
        identity: &crate::evidence_identity::ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self(identity.as_str().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn evidence_identity(&self) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
        crate::evidence_identity::forge_query_evidence_identity(
            crate::evidence_identity::ForgeQueryEvidenceScope::BasisDigest,
        )
        .field_identity(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("basis_digest"),
            self.as_str(),
        )
        .seal()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BindingFulfillmentDigest(String);

impl BindingFulfillmentDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResultDigest(String);

impl ResultDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LineageDigest(String);

impl LineageDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CorrespondenceOutcomeDigest(String);

impl CorrespondenceOutcomeDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CorrespondenceCostPostureDigest(String);

impl CorrespondenceCostPostureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HistoricalPathClassDigest(String);

impl HistoricalPathClassDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HistoricalCostPostureDigest(String);

impl HistoricalCostPostureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FailureDigest(String);

impl FailureDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CounterSnapshotDigest(String);

impl CounterSnapshotDigest {
    pub(crate) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}

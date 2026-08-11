use super::super::{
    classification_error, stable_digest, SubscriptionSupportActionOrigin,
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportRole,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportAffectedSet {
    pub(crate) fn from_retention_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support retention affected sets must not be empty",
            ));
        };
        if first.action_origin() != SubscriptionSupportActionOrigin::Retention {
            return Err(classification_error(
                "subscription-support retention affected sets require retention-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != SubscriptionSupportActionOrigin::Retention {
                return Err(classification_error(
                    "subscription-support retention affected sets cannot mix action origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support retention affected sets must be family-local",
                ));
            }
        }
        let affected_set_digest = SupportAffectedSetDigest::from_bases(&affected_bases)?;
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            affected_set_digest,
            affected_bases,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_bases.len() as u64
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub(crate) fn primary_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.affected_bases[0]
    }

    pub(crate) fn affected_artifact_ids(&self) -> Vec<SubscriptionSupportArtifactId> {
        self.affected_bases
            .iter()
            .map(|basis| basis.artifact_id().clone())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SupportAffectedSetDigest(String);

impl SupportAffectedSetDigest {
    pub(crate) fn from_bases(
        affected_bases: &[SubscriptionSupportOperationalBasis],
    ) -> Result<Self, StoreError> {
        Ok(Self(stable_digest(&affected_bases)?))
    }

    pub(crate) fn from_persisted(value: impl Into<String>) -> Result<Self, StoreError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(classification_error(
                "subscription-support affected-set digests must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

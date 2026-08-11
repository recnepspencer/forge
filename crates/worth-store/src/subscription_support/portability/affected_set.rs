use super::super::{
    classification_error, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportRole, SupportAffectedSetDigest,
};
use super::evidence_validation::validate_basis_artifact_ids;
use crate::failure::StoreError;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    action_origin: SubscriptionSupportActionOrigin,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportPortabilityAffectedSet {
    pub(crate) fn from_portability_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support portability affected sets must not be empty",
            ));
        };
        if !matches!(
            first.action_origin(),
            SubscriptionSupportActionOrigin::ReplicationExport
                | SubscriptionSupportActionOrigin::ReplicationImport
        ) {
            return Err(classification_error(
                "subscription-support portability affected sets require export/import-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != first.action_origin() {
                return Err(classification_error(
                    "subscription-support portability affected sets cannot mix export and import origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support portability affected sets must be family-local",
                ));
            }
        }
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            action_origin: first.action_origin(),
            affected_set_digest: SupportAffectedSetDigest::from_bases(&affected_bases)?,
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

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
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

    pub(crate) fn basis_digests_for_artifact_ids(
        &self,
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Vec<String>, StoreError> {
        validate_basis_artifact_ids(self, basis_artifact_ids, &[])?;
        let included = basis_artifact_ids.iter().collect::<BTreeSet<_>>();
        Ok(self
            .affected_bases
            .iter()
            .filter(|basis| included.contains(basis.artifact_id()))
            .map(|basis| basis.basis_digest().to_string())
            .collect())
    }

    pub(crate) fn all_artifacts_omitted(&self) -> Vec<SubscriptionSupportArtifactId> {
        self.affected_artifact_ids()
    }

    pub(crate) fn contains_artifact_id(&self, artifact_id: &SubscriptionSupportArtifactId) -> bool {
        self.affected_bases
            .iter()
            .any(|basis| basis.artifact_id() == artifact_id)
    }

    pub(crate) fn portability_digests(&self) -> Vec<String> {
        self.affected_bases
            .iter()
            .map(|basis| basis.portability_digest().to_string())
            .collect()
    }
}

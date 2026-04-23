use super::{
    admission_error, AdmittedSubscriptionSupportDeclaration, RawSubscriptionSupportDeclaration,
    SubscriptionSupportAuthority, SubscriptionSupportDensityClass, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportRole, SUBSCRIPTION_SUPPORT_FAMILY_VERSION,
};
use crate::failure::StoreError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportFamilyRecord {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    role: SubscriptionSupportRole,
    version: u16,
    density_class: SubscriptionSupportDensityClass,
}

impl SubscriptionSupportFamilyRecord {
    fn admits(&self, declaration: &RawSubscriptionSupportDeclaration) -> bool {
        self.family_id == declaration.family_id
            && self.family_kind == declaration.family_kind
            && self.role == declaration.role
            && self.version == declaration.family_version
    }

    pub fn density_class(&self) -> SubscriptionSupportDensityClass {
        self.density_class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubscriptionSupportAccessStructure {
    FamilyLookup,
    ArtifactLookupByFamilyAndArtifact,
    DeclarationLookup,
    BasisLookup,
    CursorLookup,
    CheckpointLookup,
    CompatibilityLookup,
    ClassificationLookup,
    RestartManifestSequence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccessStructureReport {
    required: Vec<SubscriptionSupportAccessStructure>,
    debted: Vec<SubscriptionSupportAccessStructure>,
}

impl SubscriptionSupportAccessStructureReport {
    pub fn required_first_ship() -> Self {
        Self {
            required: vec![
                SubscriptionSupportAccessStructure::FamilyLookup,
                SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact,
                SubscriptionSupportAccessStructure::DeclarationLookup,
                SubscriptionSupportAccessStructure::BasisLookup,
                SubscriptionSupportAccessStructure::CursorLookup,
                SubscriptionSupportAccessStructure::CheckpointLookup,
                SubscriptionSupportAccessStructure::CompatibilityLookup,
                SubscriptionSupportAccessStructure::ClassificationLookup,
                SubscriptionSupportAccessStructure::RestartManifestSequence,
            ],
            debted: Vec::new(),
        }
    }

    pub(crate) fn debt_for(mut debted: Vec<SubscriptionSupportAccessStructure>) -> Self {
        debted.sort();
        debted.dedup();
        Self {
            required: Self::required_first_ship().required,
            debted,
        }
    }

    pub fn required(&self) -> &[SubscriptionSupportAccessStructure] {
        &self.required
    }

    pub fn debted(&self) -> &[SubscriptionSupportAccessStructure] {
        &self.debted
    }

    pub fn has_debt(&self) -> bool {
        !self.debted.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCatalog {
    records: BTreeMap<SubscriptionSupportFamilyKind, SubscriptionSupportFamilyRecord>,
}

impl SubscriptionSupportCatalog {
    pub fn first_ship() -> Self {
        let mut records = BTreeMap::new();
        for (family_kind, id, role, density_class) in [
            (
                SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                "basis-bound-continuation-support",
                SubscriptionSupportRole::ExactContinuation,
                SubscriptionSupportDensityClass::SparseIdentityClassification,
            ),
            (
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                "materialized-narrowing-support",
                SubscriptionSupportRole::NarrowingMaterialization,
                SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
            ),
            (
                SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                "degraded-continuation-support",
                SubscriptionSupportRole::DegradedContinuation,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
            ),
        ] {
            records.insert(
                family_kind,
                SubscriptionSupportFamilyRecord {
                    family_id: SubscriptionSupportFamilyId(id.to_string()),
                    family_kind,
                    role,
                    version: SUBSCRIPTION_SUPPORT_FAMILY_VERSION,
                    density_class,
                },
            );
        }
        Self { records }
    }

    pub fn family_count(&self) -> usize {
        self.records.len()
    }

    pub fn access_structures(&self) -> SubscriptionSupportAccessStructureReport {
        SubscriptionSupportAccessStructureReport::required_first_ship()
    }

    pub(crate) fn density_for(
        &self,
        family_kind: SubscriptionSupportFamilyKind,
    ) -> Option<SubscriptionSupportDensityClass> {
        self.records
            .get(&family_kind)
            .map(SubscriptionSupportFamilyRecord::density_class)
    }

    pub fn admit(
        &self,
        declaration: RawSubscriptionSupportDeclaration,
    ) -> Result<AdmittedSubscriptionSupportDeclaration, StoreError> {
        if matches!(
            declaration.authority,
            SubscriptionSupportAuthority::Unadmitted(_)
        ) {
            return Err(admission_error(
                "subscription-support declarations require an admitted upstream authority",
            ));
        }
        if declaration.compatibility_binding.trim().is_empty() {
            return Err(admission_error(
                "subscription-support declarations require a compatibility binding",
            ));
        }
        let Some(record) = self.records.get(&declaration.family_kind) else {
            return Err(admission_error(
                "subscription-support family kind is not in the first-ship catalog",
            ));
        };
        if !record.admits(&declaration) {
            return Err(admission_error(
                "subscription-support declaration does not match the catalog family identity, role, or version",
            ));
        }
        AdmittedSubscriptionSupportDeclaration::new(declaration)
    }
}

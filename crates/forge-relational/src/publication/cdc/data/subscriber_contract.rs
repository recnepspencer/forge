use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::schema::data::{
    DescriptorSemanticsVersion, SchemaBoundaryFingerprint, SchemaContinuationClassification,
    SchemaStratum,
};

pub const MAX_NORMALIZED_CONTINUATION_BOUNDARIES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberStrataSet(BTreeSet<SchemaStratum>);

impl SubscriberStrataSet {
    pub fn new(strata: impl IntoIterator<Item = SchemaStratum>) -> Self {
        Self(strata.into_iter().collect())
    }

    pub fn contains(&self, stratum: &SchemaStratum) -> bool {
        self.0.contains(stratum)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SchemaStratum> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberContinuationClassSet(BTreeSet<SchemaContinuationClassification>);

impl SubscriberContinuationClassSet {
    pub fn new(
        classes: impl IntoIterator<Item = SchemaContinuationClassification>,
    ) -> Self {
        Self(classes.into_iter().collect())
    }

    pub fn contains(
        &self,
        classification: &SchemaContinuationClassification,
    ) -> bool {
        self.0.contains(classification)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SchemaContinuationClassification> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberContractDeclaration {
    pub contract_id: String,
    pub consumable_strata: SubscriberStrataSet,
    pub accepted_continuation_classes: SubscriberContinuationClassSet,
    pub accepted_upgrade_classes: SubscriberContinuationClassSet,
}

impl Default for SubscriberContractDeclaration {
    fn default() -> Self {
        Self {
            contract_id: "default.subscriber.contract".to_string(),
            consumable_strata: SubscriberStrataSet::new([
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
                SchemaStratum::SubscriberContract,
            ]),
            accepted_continuation_classes: SubscriberContinuationClassSet::new([
                SchemaContinuationClassification::ContinueUnchanged,
                SchemaContinuationClassification::ContinueWithTransparentBridge,
                SchemaContinuationClassification::ContinueWithVisibleBridge,
            ]),
            accepted_upgrade_classes: SubscriberContinuationClassSet::new([
                SchemaContinuationClassification::ContinueWithContractUpgrade,
            ]),
        }
    }
}

impl SubscriberContractDeclaration {
    pub fn consumes_any_strata(
        &self,
        changed_strata: &[SchemaStratum],
    ) -> bool {
        changed_strata
            .iter()
            .any(|stratum| self.consumable_strata.contains(stratum))
    }

    pub fn accepts_continuation(
        &self,
        classification: SchemaContinuationClassification,
    ) -> bool {
        self.accepted_continuation_classes.contains(&classification)
    }

    pub fn accepts_upgrade(
        &self,
        classification: SchemaContinuationClassification,
    ) -> bool {
        self.accepted_upgrade_classes.contains(&classification)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedContinuationProof {
    boundary_fingerprints: Vec<SchemaBoundaryFingerprint>,
    descriptor_semantics_version: DescriptorSemanticsVersion,
    normalized_boundary_count: usize,
}

impl NormalizedContinuationProof {
    pub fn new(
        boundary_fingerprints: Vec<SchemaBoundaryFingerprint>,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        debug_assert_eq!(boundary_fingerprints.len(), {
            let mut deduped = BTreeSet::new();
            for fingerprint in &boundary_fingerprints {
                deduped.insert(*fingerprint);
            }
            deduped.len()
        });
        debug_assert!(boundary_fingerprints.len() <= MAX_NORMALIZED_CONTINUATION_BOUNDARIES);
        Self {
            normalized_boundary_count: boundary_fingerprints.len(),
            boundary_fingerprints,
            descriptor_semantics_version,
        }
    }

    pub fn boundary_fingerprints(&self) -> &[SchemaBoundaryFingerprint] {
        &self.boundary_fingerprints
    }

    pub fn descriptor_semantics_version(&self) -> DescriptorSemanticsVersion {
        self.descriptor_semantics_version
    }

    pub fn normalized_boundary_count(&self) -> usize {
        self.normalized_boundary_count
    }

    #[cfg(test)]
    pub(crate) fn from_raw_parts_for_test(
        boundary_fingerprints: Vec<SchemaBoundaryFingerprint>,
        descriptor_semantics_version: DescriptorSemanticsVersion,
        normalized_boundary_count: usize,
    ) -> Self {
        Self {
            boundary_fingerprints,
            descriptor_semantics_version,
            normalized_boundary_count,
        }
    }
}

impl Default for NormalizedContinuationProof {
    fn default() -> Self {
        Self::new(Vec::new(), DescriptorSemanticsVersion::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberContinuationSummary {
    pub contract_id: String,
    pub continuation_outcome: SchemaContinuationClassification,
    pub crossed_boundary_count: usize,
    pub normalized_boundary_count: usize,
    pub descriptor_semantics_version: DescriptorSemanticsVersion,
    pub contract_upgrade_applied: bool,
}

impl SubscriberContinuationSummary {
    pub fn unchanged(
        contract_id: String,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self {
            contract_id,
            continuation_outcome: SchemaContinuationClassification::ContinueUnchanged,
            crossed_boundary_count: 0,
            normalized_boundary_count: 0,
            descriptor_semantics_version,
            contract_upgrade_applied: false,
        }
    }

    pub fn new(
        contract_id: String,
        continuation_outcome: SchemaContinuationClassification,
        crossed_boundary_count: usize,
        normalized_boundary_count: usize,
        descriptor_semantics_version: DescriptorSemanticsVersion,
        contract_upgrade_applied: bool,
    ) -> Self {
        Self {
            contract_id,
            continuation_outcome,
            crossed_boundary_count,
            normalized_boundary_count,
            descriptor_semantics_version,
            contract_upgrade_applied,
        }
    }
}

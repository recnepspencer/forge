use std::collections::BTreeMap;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::consumer::SpatialCompiledProductConsumer;
use super::declaration::SpatialCompiledProductFamilyDeclaration;
use super::error::SpatialCompiledProductFamilyError;
use super::error::SpatialCompiledProductFamilyErrorKind;
use super::family_identity::SpatialCompiledProductFamilyIdentity;
use super::posture::{
    SpatialEquivalencePolicyPosture, SpatialEvidenceSupportRolePosture,
    SpatialLocalityFootprintBasisPosture, SpatialPriorProofRolePosture,
    SpatialSourceAuthorityDigestBasisPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyCatalogCounters {
    family_count: usize,
    declared_family_count: usize,
    supported_consumer_count: usize,
}

impl SpatialCompiledProductFamilyCatalogCounters {
    const fn new(
        family_count: usize,
        declared_family_count: usize,
        supported_consumer_count: usize,
    ) -> Self {
        Self {
            family_count,
            declared_family_count,
            supported_consumer_count,
        }
    }

    pub const fn family_count(&self) -> usize {
        self.family_count
    }

    pub const fn declared_family_count(&self) -> usize {
        self.declared_family_count
    }

    pub const fn supported_consumer_count(&self) -> usize {
        self.supported_consumer_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialCompiledProductFamilyCatalog {
    families: Vec<SpatialCompiledProductFamilyDeclaration>,
    counters: SpatialCompiledProductFamilyCatalogCounters,
    catalog_digest: String,
}

impl SpatialCompiledProductFamilyCatalog {
    pub(crate) fn new(
        mut families: Vec<SpatialCompiledProductFamilyDeclaration>,
    ) -> Result<Self, SpatialCompiledProductFamilyError> {
        families.sort_by_key(SpatialCompiledProductFamilyDeclaration::identity);
        reject_duplicate_consumer_coverage(&families)?;
        let supported_consumer_count = families
            .iter()
            .map(|family| family.supported_consumers().len())
            .sum();
        let counters = SpatialCompiledProductFamilyCatalogCounters::new(
            families.len(),
            SpatialCompiledProductFamilyIdentity::REQUIRED.len(),
            supported_consumer_count,
        );
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:compiled-product-family-catalog:v1".to_string(),
                format!("family-count:{}", counters.family_count),
                format!("declared-family-count:{}", counters.declared_family_count),
                format!(
                    "supported-consumer-count:{}",
                    counters.supported_consumer_count
                ),
                families
                    .iter()
                    .map(|family| format!("family-digest:{}", family.family_digest()))
                    .collect::<Vec<_>>()
                    .join("|"),
            ],
        );
        Ok(Self {
            families,
            counters,
            catalog_digest,
        })
    }

    pub fn family(
        &self,
        identity: SpatialCompiledProductFamilyIdentity,
    ) -> Option<&SpatialCompiledProductFamilyDeclaration> {
        self.families
            .iter()
            .find(|family| family.identity() == identity)
    }

    pub fn family_for_consumer(
        &self,
        consumer: SpatialCompiledProductConsumer,
    ) -> Option<&SpatialCompiledProductFamilyDeclaration> {
        self.families
            .iter()
            .find(|family| family.supports(consumer))
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub const fn counters(&self) -> SpatialCompiledProductFamilyCatalogCounters {
        self.counters
    }
}

pub fn current_spatial_compiled_product_family_catalog() -> SpatialCompiledProductFamilyCatalog {
    SpatialCompiledProductFamilyCatalog::new(vec![
        SpatialCompiledProductFamilyDeclaration::new(
            SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport,
            vec![
                SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
                SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
            ],
            SpatialSourceAuthorityDigestBasisPosture::EvidenceLookupLedgerBasisWithStageReceiptCoordinate,
            SpatialLocalityFootprintBasisPosture::SpatialTouchDigest,
            SpatialPriorProofRolePosture::SelectedPlanTopologyAndQuerySupportBasis,
            SpatialEvidenceSupportRolePosture::QueryAndTopologySupportEvidence,
            SpatialEquivalencePolicyPosture::EvidenceLookupIndexSemanticParity,
            "spatial-evidence-lookup-derived-support",
            &[
                "compiled-product-identity",
                "selected-plan",
                "spatial-touch",
                "stage-receipt",
                "topology-support",
                "query-support",
            ],
        ),
        SpatialCompiledProductFamilyDeclaration::new(
            SpatialCompiledProductFamilyIdentity::RetainedCancellationDerivedSupport,
            vec![SpatialCompiledProductConsumer::RetainedCancellationChain],
            SpatialSourceAuthorityDigestBasisPosture::RetainedCancellationChainAuthorityDigest,
            SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest,
            SpatialPriorProofRolePosture::RetainedCancellationCheckpointHistoryBasis,
            SpatialEvidenceSupportRolePosture::RetainedCancellationProjectionEvidence,
            SpatialEquivalencePolicyPosture::RetainedCancellationSemanticParity,
            "spatial-retained-cancellation-derived-support",
            &[
                "compiled-product-identity",
                "retained-cancellation-workload",
                "retained-basis",
                "projection-consumed",
                "checkpoint-history",
            ],
        ),
        SpatialCompiledProductFamilyDeclaration::new(
            SpatialCompiledProductFamilyIdentity::RetainedReplayDerivedSupport,
            vec![SpatialCompiledProductConsumer::RetainedReplayParity],
            SpatialSourceAuthorityDigestBasisPosture::RetainedPlanarHistoricalInspectionDigest,
            SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest,
            SpatialPriorProofRolePosture::NotRequired,
            SpatialEvidenceSupportRolePosture::RetainedReplayProjectionEvidence,
            SpatialEquivalencePolicyPosture::RetainedReplaySemanticParity,
            "spatial-retained-replay-derived-support",
            &[
                "compiled-product-identity",
                "retained-planar-facts",
                "projection-consumed-facts",
            ],
        ),
    ])
    .expect("spatial compiled-product family catalog declares each consumer exactly once")
}

#[cfg(test)]
pub(crate) fn catalog_from_declarations(
    declarations: Vec<SpatialCompiledProductFamilyDeclaration>,
) -> Result<SpatialCompiledProductFamilyCatalog, SpatialCompiledProductFamilyError> {
    SpatialCompiledProductFamilyCatalog::new(declarations)
}

fn reject_duplicate_consumer_coverage(
    families: &[SpatialCompiledProductFamilyDeclaration],
) -> Result<(), SpatialCompiledProductFamilyError> {
    let mut seen =
        BTreeMap::<SpatialCompiledProductConsumer, SpatialCompiledProductFamilyIdentity>::new();
    for family in families {
        for consumer in family.supported_consumers() {
            if let Some(existing_family) = seen.insert(*consumer, family.identity()) {
                return Err(SpatialCompiledProductFamilyError::new(
                    SpatialCompiledProductFamilyErrorKind::DuplicateConsumerCoverage,
                    format!(
                        "spatial compiled-product consumer `{}` was declared by both `{}` and `{}`",
                        consumer.as_str(),
                        existing_family.as_str(),
                        family.identity().as_str()
                    ),
                ));
            }
        }
    }
    Ok(())
}

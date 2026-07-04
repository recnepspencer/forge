use serde::Serialize;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::consumer::TopologyCompiledProductConsumer;
use super::declaration::TopologyCompiledProductFamilyDeclaration;
use super::family_identity::TopologyCompiledProductFamilyIdentity;
use super::posture::{
    TopologyAuthorityBasisPosture, TopologyEquivalencePolicyPosture,
    TopologyLocalityFootprintPosture, TopologyPriorProofPosture, TopologyStageIdentityPosture,
    TopologyValidatorEvidenceRolePosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductFamilyCatalogCounters {
    family_count: usize,
    declared_family_count: usize,
    supported_consumer_count: usize,
}

impl TopologyCompiledProductFamilyCatalogCounters {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductFamilyCatalog {
    families: Vec<TopologyCompiledProductFamilyDeclaration>,
    counters: TopologyCompiledProductFamilyCatalogCounters,
    catalog_digest: String,
}

impl TopologyCompiledProductFamilyCatalog {
    fn new(mut families: Vec<TopologyCompiledProductFamilyDeclaration>) -> Self {
        families.sort_by_key(TopologyCompiledProductFamilyDeclaration::identity);
        let supported_consumer_count = families
            .iter()
            .map(|family| family.supported_consumers().len())
            .sum();
        let counters = TopologyCompiledProductFamilyCatalogCounters::new(
            families.len(),
            TopologyCompiledProductFamilyIdentity::REQUIRED.len(),
            supported_consumer_count,
        );
        let catalog_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:compiled-product-family-catalog:v1".to_string(),
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
        Self {
            families,
            counters,
            catalog_digest,
        }
    }

    pub fn family(
        &self,
        identity: TopologyCompiledProductFamilyIdentity,
    ) -> Option<&TopologyCompiledProductFamilyDeclaration> {
        self.families
            .iter()
            .find(|family| family.identity() == identity)
    }

    pub fn family_for_consumer(
        &self,
        consumer: TopologyCompiledProductConsumer,
    ) -> Option<&TopologyCompiledProductFamilyDeclaration> {
        self.families
            .iter()
            .find(|family| family.supports(consumer))
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub const fn counters(&self) -> TopologyCompiledProductFamilyCatalogCounters {
        self.counters
    }
}

pub fn current_topology_compiled_product_family_catalog() -> TopologyCompiledProductFamilyCatalog {
    TopologyCompiledProductFamilyCatalog::new(vec![TopologyCompiledProductFamilyDeclaration::new(
        TopologyCompiledProductFamilyIdentity::DerivedTopologyEquivalenceContract,
        vec![
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            TopologyCompiledProductConsumer::DerivedEquivalenceCertificationParity,
        ],
        TopologyAuthorityBasisPosture::DerivedTopologyTruthBasis,
        TopologyLocalityFootprintPosture::InvalidationClosure,
        TopologyPriorProofPosture::NotRequired,
        TopologyStageIdentityPosture::NotRequired,
        TopologyValidatorEvidenceRolePosture::DerivedValidationDigestEquivalenceDimension,
        TopologyEquivalencePolicyPosture::DerivedTopologySemanticParity,
        "topology-derived-equivalence",
        &[
            "compiled-product-identity",
            "materialized-topology",
            "interpreted-topology",
            "derived-validation",
        ],
    )])
}

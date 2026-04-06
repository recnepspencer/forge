use std::sync::Arc;

use crate::clone_budget::CheapClone;
use crate::input::envelope::{BridgeCommittedPatchDigest, BridgeProducerMetadata};
use crate::routing::context::BridgeMappingContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteContractProof {
    producer_metadata: BridgeProducerMetadata,
    mapping_context: BridgeMappingContext,
    source_digest: BridgeCommittedPatchDigest,
    planning_provenance_digest: Arc<str>,
    planning_summary_digest: Arc<str>,
    lowering_provenance_digest: Arc<str>,
    lowering_summary_digest: Arc<str>,
}

impl BridgeRouteContractProof {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        producer_metadata: BridgeProducerMetadata,
        mapping_context: BridgeMappingContext,
        source_digest: BridgeCommittedPatchDigest,
        planning_provenance_digest: impl Into<Arc<str>>,
        planning_summary_digest: impl Into<Arc<str>>,
        lowering_provenance_digest: impl Into<Arc<str>>,
        lowering_summary_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            producer_metadata,
            mapping_context,
            source_digest,
            planning_provenance_digest: planning_provenance_digest.into(),
            planning_summary_digest: planning_summary_digest.into(),
            lowering_provenance_digest: lowering_provenance_digest.into(),
            lowering_summary_digest: lowering_summary_digest.into(),
        }
    }

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        &self.producer_metadata
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        &self.mapping_context
    }

    pub fn mapping_context_digest(&self) -> &str {
        self.mapping_context.digest()
    }

    pub fn source_digest(&self) -> &BridgeCommittedPatchDigest {
        &self.source_digest
    }

    pub fn planning_provenance_digest(&self) -> &str {
        &self.planning_provenance_digest
    }

    pub fn planning_summary_digest(&self) -> &str {
        &self.planning_summary_digest
    }

    pub fn lowering_provenance_digest(&self) -> &str {
        &self.lowering_provenance_digest
    }

    pub fn lowering_summary_digest(&self) -> &str {
        &self.lowering_summary_digest
    }
}

impl CheapClone for BridgeRouteContractProof {}

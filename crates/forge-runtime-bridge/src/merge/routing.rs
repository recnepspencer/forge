use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeMergeCounters, BridgeMergeRoutingOutcomeClass, LoweredMergeHistoryPacketSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedMergeRoutingArtifact {
    lowered_packet_set: LoweredMergeHistoryPacketSet,
    outcome_class: BridgeMergeRoutingOutcomeClass,
    counters: BridgeMergeCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ReducedMergeRoutingArtifact {
    pub(crate) fn from_lowered_packet_set(
        lowered_packet_set: LoweredMergeHistoryPacketSet,
    ) -> Self {
        let outcome_class = if lowered_packet_set.structural_contradiction() {
            BridgeMergeRoutingOutcomeClass::StructuralContradiction
        } else if lowered_packet_set.denial_class().is_some() {
            BridgeMergeRoutingOutcomeClass::Denied
        } else {
            BridgeMergeRoutingOutcomeClass::ContinuityCandidate
        };

        let mut counters = *lowered_packet_set.counters();
        counters = counters.with_routing_result();
        let canonical_basis = Arc::<str>::from(format!(
            "reduced-merge-routing-artifact|lowered={}|outcome:{outcome_class:?}|blocked-stage={:?}|denial={:?}",
            lowered_packet_set.digest(),
            lowered_packet_set.blocked_stage(),
            lowered_packet_set.denial_class(),
        ));
        counters = counters.with_digest(canonical_basis.len());
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            lowered_packet_set,
            outcome_class,
            counters,
            canonical_basis,
            digest: Arc::from(format!("reduced-merge-routing-artifact:sha256:{digest:x}")),
        }
    }

    pub fn lowered_packet_set(&self) -> &LoweredMergeHistoryPacketSet {
        &self.lowered_packet_set
    }

    pub fn outcome_class(&self) -> BridgeMergeRoutingOutcomeClass {
        self.outcome_class
    }

    pub fn counters(&self) -> &BridgeMergeCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub(crate) fn with_counters(mut self, counters: BridgeMergeCounters) -> Self {
        self.counters = counters;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ReducedMergeRoutingArtifact;

    use crate::merge::{
        AdmittedMergeRegistry, BridgeMergeAuthorityBasis, BridgeMergeAuthorityBasisKind,
        BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface,
        BridgeMergeParentOrderProof, BridgeMergeRoutingOutcomeClass, MergeHistoryDeclaration,
        MergeHistoryDeclarationIdentity,
    };

    #[test]
    fn reduced_merge_routing_artifact_tracks_continuity_candidate() {
        let declaration = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:test"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
            BridgeMergeAuthorityBasis::new(
                BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
                "merge-artifact:test",
                "rel-merge-v1",
                "schema-policy-v1",
                BridgeMergeParentOrderProof::new(vec![
                    crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                    crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
                ]),
            ),
        );
        let contract = AdmittedMergeRegistry::freeze(vec![declaration])
            .expect("merge registry should freeze")
            .contracts()[0]
            .clone();
        let lowered = crate::merge::LoweredMergeHistoryPacketSet::from_contract(&contract);
        let reduced = ReducedMergeRoutingArtifact::from_lowered_packet_set(lowered);

        assert_eq!(
            reduced.outcome_class(),
            BridgeMergeRoutingOutcomeClass::ContinuityCandidate
        );
        assert_eq!(reduced.counters().merge_routing_result_count(), 1);
    }
}

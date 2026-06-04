use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeMergeRoutingOutcomeClass, BridgeMergeStructuralAdvisoryDisposition,
    LoweredMergeHistoryPacketSet, ReducedMergeRoutingArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedMergeContinuityArtifact {
    reduced_routing_artifact: ReducedMergeRoutingArtifact,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PublishedMergeContinuityArtifact {
    pub(crate) fn from_reduced_routing_artifact(
        reduced_routing_artifact: ReducedMergeRoutingArtifact,
    ) -> Option<Self> {
        if reduced_routing_artifact.outcome_class()
            != BridgeMergeRoutingOutcomeClass::ContinuityCandidate
        {
            return None;
        }

        let canonical_basis = Arc::<str>::from(format!(
            "published-merge-continuity-artifact|reduced={}|blocked-stage={:?}|denial={:?}",
            reduced_routing_artifact.digest(),
            reduced_routing_artifact
                .lowered_packet_set()
                .blocked_stage(),
            reduced_routing_artifact.lowered_packet_set().denial_class(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Some(Self {
            reduced_routing_artifact,
            canonical_basis,
            digest: Arc::from(format!(
                "published-merge-continuity-artifact:sha256:{digest:x}"
            )),
        })
    }

    pub fn reduced_routing_artifact(&self) -> &ReducedMergeRoutingArtifact {
        &self.reduced_routing_artifact
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedMergeRemapArtifact {
    reduced_routing_artifact: ReducedMergeRoutingArtifact,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PublishedMergeRemapArtifact {
    pub(crate) fn from_reduced_routing_artifact(
        reduced_routing_artifact: ReducedMergeRoutingArtifact,
    ) -> Option<Self> {
        let lowered: &LoweredMergeHistoryPacketSet = reduced_routing_artifact.lowered_packet_set();
        if reduced_routing_artifact.outcome_class()
            != BridgeMergeRoutingOutcomeClass::ContinuityCandidate
            || lowered
                .contract()
                .validated_declaration()
                .declaration()
                .structural_advisory()
                != BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent
        {
            return None;
        }

        let canonical_basis = Arc::<str>::from(format!(
            "published-merge-remap-artifact|reduced={}|structural:{:?}",
            reduced_routing_artifact.digest(),
            lowered
                .contract()
                .validated_declaration()
                .declaration()
                .structural_advisory(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Some(Self {
            reduced_routing_artifact,
            canonical_basis,
            digest: Arc::from(format!("published-merge-remap-artifact:sha256:{digest:x}")),
        })
    }

    pub fn reduced_routing_artifact(&self) -> &ReducedMergeRoutingArtifact {
        &self.reduced_routing_artifact
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{PublishedMergeContinuityArtifact, PublishedMergeRemapArtifact};

    use crate::merge::{
        AdmittedMergeRegistry, BridgeMergeAuthorityBasis, BridgeMergeAuthorityBasisKind,
        BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface,
        BridgeMergeParentOrderProof, BridgeMergeStructuralAdvisoryDisposition,
        LoweredMergeHistoryPacketSet, MergeHistoryDeclaration, MergeHistoryDeclarationIdentity,
        ReducedMergeRoutingArtifact,
    };

    fn reduced(
        structural: BridgeMergeStructuralAdvisoryDisposition,
    ) -> ReducedMergeRoutingArtifact {
        let declaration = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::new("merge:test"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
            BridgeMergeAuthorityBasis::new(
                BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
                "merge-artifact:test",
                "rel-merge-v1",
                "schema-policy-v1",
                BridgeMergeParentOrderProof::new(vec![
                    crate::facade::TruthCommitIdentity::new("parent-a"),
                    crate::facade::TruthCommitIdentity::new("parent-b"),
                ]),
            ),
        )
        .with_structural_advisory(structural);
        let contract = AdmittedMergeRegistry::freeze(vec![declaration])
            .expect("merge registry should freeze")
            .contracts()[0]
            .clone();
        ReducedMergeRoutingArtifact::from_lowered_packet_set(
            LoweredMergeHistoryPacketSet::from_contract(&contract),
        )
    }

    #[test]
    fn merge_continuity_publication_accepts_continuity_candidate() {
        let reduced = reduced(BridgeMergeStructuralAdvisoryDisposition::NotConsulted);
        let artifact =
            PublishedMergeContinuityArtifact::from_reduced_routing_artifact(reduced.clone())
                .expect("continuity candidate should publish");
        assert_eq!(artifact.reduced_routing_artifact(), &reduced);
        assert_eq!(
            artifact.reduced_routing_artifact().outcome_class(),
            crate::merge::BridgeMergeRoutingOutcomeClass::ContinuityCandidate
        );
    }

    #[test]
    fn merge_remap_publication_requires_consistent_structural_advisory() {
        assert!(
            PublishedMergeRemapArtifact::from_reduced_routing_artifact(reduced(
                BridgeMergeStructuralAdvisoryDisposition::NotConsulted,
            ))
            .is_none()
        );
        assert!(
            PublishedMergeRemapArtifact::from_reduced_routing_artifact(reduced(
                BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent,
            ))
            .is_some()
        );
    }
}

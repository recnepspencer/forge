use worth_proof::TransitionOutcome;

use super::primitives::{
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
};
use super::provenance::{
    FoundationalBoundaryEvidenceAuthorityPath, FoundationalBoundaryEvidenceCanonicalDigestBasis,
    FoundationalBoundaryEvidenceComparisonBasis, FoundationalBoundaryEvidenceProfileBasis,
    FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalBoundaryEvidenceStrategyBasis,
    FoundationalBoundaryEvidenceSupportContextAttachment,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceProvenanceFrontDoor;

impl FoundationalBoundaryEvidenceProvenanceFrontDoor {
    pub fn current(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::Current,
            source_basis,
        )
    }

    pub fn branch_local(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::BranchLocal,
            source_basis,
        )
    }

    pub fn historical(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::Historical,
            source_basis,
        )
    }

    pub fn comparison_paired(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::ComparisonPaired,
            source_basis,
        )
    }

    pub fn snapshot_bound(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::SnapshotBound,
            source_basis,
        )
    }

    pub fn replay_derived(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::ReplayDerived,
            source_basis,
        )
    }

    pub fn restored_readmitted(
        self,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> FoundationalBoundaryEvidenceProvenanceLocalityStep {
        FoundationalBoundaryEvidenceProvenanceLocalityStep::new(
            FoundationalBoundaryEvidenceLocality::RestoredReadmitted,
            source_basis,
        )
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceProvenanceLocalityStep {
    locality: FoundationalBoundaryEvidenceLocality,
    source_basis: FoundationalBoundaryEvidenceSourceBasis,
    authority_path: Option<FoundationalBoundaryEvidenceAuthorityPath>,
    strategy_basis: Option<FoundationalBoundaryEvidenceStrategyBasis>,
    profile_basis: Option<FoundationalBoundaryEvidenceProfileBasis>,
    comparison_basis: Option<FoundationalBoundaryEvidenceComparisonBasis>,
    canonical_digest_basis: Option<FoundationalBoundaryEvidenceCanonicalDigestBasis>,
    support_context_attachments: Vec<FoundationalBoundaryEvidenceSupportContextAttachment>,
}

impl FoundationalBoundaryEvidenceProvenanceLocalityStep {
    fn new(
        locality: FoundationalBoundaryEvidenceLocality,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
    ) -> Self {
        Self {
            locality,
            source_basis,
            authority_path: None,
            strategy_basis: None,
            profile_basis: None,
            comparison_basis: None,
            canonical_digest_basis: None,
            support_context_attachments: Vec::new(),
        }
    }

    pub fn authority_path(mut self, path: FoundationalBoundaryEvidenceAuthorityPath) -> Self {
        self.authority_path = Some(path);
        self
    }

    pub fn strategy_basis(mut self, basis: FoundationalBoundaryEvidenceStrategyBasis) -> Self {
        self.strategy_basis = Some(basis);
        self
    }

    pub fn profile_basis(mut self, basis: FoundationalBoundaryEvidenceProfileBasis) -> Self {
        self.profile_basis = Some(basis);
        self
    }

    pub fn comparison_basis(mut self, basis: FoundationalBoundaryEvidenceComparisonBasis) -> Self {
        self.comparison_basis = Some(basis);
        self
    }

    pub fn canonical_digest_basis(
        mut self,
        basis: FoundationalBoundaryEvidenceCanonicalDigestBasis,
    ) -> Self {
        self.canonical_digest_basis = Some(basis);
        self
    }

    pub fn attach_support_context(
        mut self,
        attachment: FoundationalBoundaryEvidenceSupportContextAttachment,
    ) -> Self {
        self.support_context_attachments.push(attachment);
        self
    }

    pub fn with_freshness(
        self,
        freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidenceProvenanceArtifact,
        FoundationalBoundaryEvidenceProvenanceConstructionDenial,
    > {
        let denial = classify_provenance_freshness_denial(self.locality, freshness_posture);

        if let Some(denial) = denial {
            return TransitionOutcome::denied(denial);
        }

        TransitionOutcome::success(materialize_provenance_artifact(self, freshness_posture))
    }
}

fn classify_provenance_freshness_denial(
    locality: FoundationalBoundaryEvidenceLocality,
    freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
) -> Option<FoundationalBoundaryEvidenceProvenanceConstructionDenial> {
    match (locality, freshness_posture) {
        (
            FoundationalBoundaryEvidenceLocality::ReplayDerived,
            FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay,
        ) => None,
        (FoundationalBoundaryEvidenceLocality::ReplayDerived, _) => Some(
            FoundationalBoundaryEvidenceProvenanceConstructionDenial::ReplayDerivedLocalityRequiresReplayFreshness,
        ),
        (
            FoundationalBoundaryEvidenceLocality::RestoredReadmitted,
            FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint,
        ) => None,
        (FoundationalBoundaryEvidenceLocality::RestoredReadmitted, _) => Some(
            FoundationalBoundaryEvidenceProvenanceConstructionDenial::RestoredReadmittedLocalityRequiresRestoredFreshness,
        ),
        (
            FoundationalBoundaryEvidenceLocality::Current
            | FoundationalBoundaryEvidenceLocality::BranchLocal,
            FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay,
        ) => Some(
            FoundationalBoundaryEvidenceProvenanceConstructionDenial::CurrentOrBranchLocalLocalityMustNotUseReplayFreshness,
        ),
        (
            FoundationalBoundaryEvidenceLocality::Current
            | FoundationalBoundaryEvidenceLocality::BranchLocal,
            FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint,
        ) => Some(
            FoundationalBoundaryEvidenceProvenanceConstructionDenial::CurrentOrBranchLocalLocalityMustNotUseRestoredFreshness,
        ),
        _ => None,
    }
}

fn materialize_provenance_artifact(
    mut locality_step: FoundationalBoundaryEvidenceProvenanceLocalityStep,
    freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
) -> FoundationalBoundaryEvidenceProvenanceArtifact {
    canonicalize_support_context_attachments(&mut locality_step.support_context_attachments);

    FoundationalBoundaryEvidenceProvenanceArtifact::new(
        locality_step.locality,
        freshness_posture,
        locality_step.source_basis,
        locality_step.authority_path,
        locality_step.strategy_basis,
        locality_step.profile_basis,
        locality_step.comparison_basis,
        locality_step.canonical_digest_basis,
        locality_step.support_context_attachments,
    )
}

fn canonicalize_support_context_attachments(
    attachments: &mut Vec<FoundationalBoundaryEvidenceSupportContextAttachment>,
) {
    attachments.sort();
    attachments.dedup();
}

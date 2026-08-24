use worth_store_physical_format::{
    BoundedFreeSpaceMembershipBlockDecodeDenial, BoundedRootRoutingBlockDecodeDenial,
    BoundedSegmentMembershipBlockDecodeDenial, RecordArtifactFile,
};

use super::artifact_generation;
use crate::entry::PhysicalRecoverySuccessorCandidateDenial;
use crate::orchestration::planning::manifest_entry_budget::ManifestEntryBudget;

pub(super) fn root_denial(
    denial: BoundedRootRoutingBlockDecodeDenial,
    artifact: RecordArtifactFile,
    budget: &ManifestEntryBudget,
) -> PhysicalRecoverySuccessorCandidateDenial {
    match denial {
        BoundedRootRoutingBlockDecodeDenial::LeafEntries { observed, admitted }
        | BoundedRootRoutingBlockDecodeDenial::BranchChildren { observed, admitted } => {
            debug_assert_eq!(admitted, budget.remaining());
            let (observed, admitted) = budget.crossing_evidence(observed);
            limit(artifact, observed, admitted)
        }
        BoundedRootRoutingBlockDecodeDenial::Format(_) => invalid(artifact),
    }
}

pub(super) fn segment_denial(
    denial: BoundedSegmentMembershipBlockDecodeDenial,
    artifact: RecordArtifactFile,
    budget: &ManifestEntryBudget,
) -> PhysicalRecoverySuccessorCandidateDenial {
    match denial {
        BoundedSegmentMembershipBlockDecodeDenial::LeafEntries { observed, admitted }
        | BoundedSegmentMembershipBlockDecodeDenial::BranchChildren { observed, admitted } => {
            debug_assert_eq!(admitted, budget.remaining());
            let (observed, admitted) = budget.crossing_evidence(observed);
            limit(artifact, observed, admitted)
        }
        BoundedSegmentMembershipBlockDecodeDenial::Format(_) => invalid(artifact),
    }
}

pub(super) fn free_denial(
    denial: BoundedFreeSpaceMembershipBlockDecodeDenial,
    artifact: RecordArtifactFile,
    budget: &ManifestEntryBudget,
) -> PhysicalRecoverySuccessorCandidateDenial {
    match denial {
        BoundedFreeSpaceMembershipBlockDecodeDenial::LeafEntries { observed, admitted }
        | BoundedFreeSpaceMembershipBlockDecodeDenial::BranchChildren { observed, admitted } => {
            debug_assert_eq!(admitted, budget.remaining());
            let (observed, admitted) = budget.crossing_evidence(observed);
            limit(artifact, observed, admitted)
        }
        BoundedFreeSpaceMembershipBlockDecodeDenial::Format(_) => invalid(artifact),
    }
}

pub(super) const fn invalid(
    artifact: RecordArtifactFile,
) -> PhysicalRecoverySuccessorCandidateDenial {
    PhysicalRecoverySuccessorCandidateDenial::InvalidArtifact {
        artifact,
        generation: artifact_generation(artifact),
    }
}

const fn limit(
    artifact: RecordArtifactFile,
    observed: u64,
    admitted: u64,
) -> PhysicalRecoverySuccessorCandidateDenial {
    PhysicalRecoverySuccessorCandidateDenial::ManifestEntryLimit {
        artifact,
        generation: artifact_generation(artifact),
        observed,
        admitted,
    }
}

pub(super) fn admit_successor_read(
    budget: &ManifestEntryBudget,
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    budget
        .successor_read_evidence()
        .map_err(|(observed, admitted)| limit(artifact, observed, admitted))
}

pub(super) fn consume_successor(
    budget: &mut ManifestEntryBudget,
    entries: usize,
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    budget
        .consume_with_evidence(entries)
        .map_err(|(observed, admitted)| limit(artifact, observed, admitted))
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::{BoundedRootRoutingBlockDecodeDenial, RecordArtifactFile};

    use super::{root_denial, ManifestEntryBudget, PhysicalRecoverySuccessorCandidateDenial};

    #[test]
    fn decoder_crossing_uses_global_manifest_budget_coordinates() {
        let artifact = RecordArtifactFile::RootRoutingBlock {
            generation: 9,
            block: 4,
        };
        let budget = ManifestEntryBudget::new(10, 7);
        let denial = root_denial(
            BoundedRootRoutingBlockDecodeDenial::LeafEntries {
                observed: 4,
                admitted: 3,
            },
            artifact,
            &budget,
        );
        assert_eq!(
            denial,
            PhysicalRecoverySuccessorCandidateDenial::ManifestEntryLimit {
                artifact,
                generation: 9,
                observed: 11,
                admitted: 10,
            }
        );
    }
}

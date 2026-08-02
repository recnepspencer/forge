use worth_proof::NonEmpty;
use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::durability::{
    PhysicalRootPublicationIdentity, PhysicalRootPublicationMemberIdentity,
    PhysicalRootPublicationTransition,
};
use crate::physical_runtime::{
    PhysicalDurabilityGroupBasis, PreparedPhysicalRootCandidate,
    RootPublicationPhysicalMutationMember, SettledPhysicalWork,
};

pub struct RootPublicationPreparedPhysicalMutationMembers {
    core: RootPublicationPreparedCore,
}

pub(in crate::physical_runtime) struct RootPublicationPreparedCore {
    identity: PhysicalRootPublicationIdentity,
    group: PhysicalDurabilityGroupBasis,
    member_identities: Box<[PhysicalRootPublicationMemberIdentity]>,
    members: NonEmpty<RootPublicationPhysicalMutationMember>,
    candidate: PreparedPhysicalRootCandidate,
    candidate_synchronizations: Box<[SettledPhysicalWork]>,
    transition: PhysicalRootPublicationTransition,
}

impl RootPublicationPreparedPhysicalMutationMembers {
    pub(in crate::physical_runtime) fn new(
        identity: PhysicalRootPublicationIdentity,
        group: PhysicalDurabilityGroupBasis,
        members: NonEmpty<RootPublicationPhysicalMutationMember>,
        candidate: PreparedPhysicalRootCandidate,
        candidate_synchronizations: Box<[SettledPhysicalWork]>,
        transition: PhysicalRootPublicationTransition,
    ) -> Self {
        let member_identities = members
            .as_slice()
            .iter()
            .map(RootPublicationPhysicalMutationMember::identity)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            core: RootPublicationPreparedCore {
                identity,
                group,
                member_identities,
                members,
                candidate,
                candidate_synchronizations,
                transition,
            },
        }
    }

    pub const fn group_basis(&self) -> PhysicalDurabilityGroupBasis {
        self.core.group
    }

    pub fn members(&self) -> &[PhysicalRootPublicationMemberIdentity] {
        &self.core.member_identities
    }

    pub fn settled_members(&self) -> &[RootPublicationPhysicalMutationMember] {
        self.core.members.as_slice()
    }

    pub fn candidate_artifacts(&self) -> &[RecordArtifactFile] {
        self.core.candidate.artifacts()
    }

    pub const fn catalog_candidate(&self) -> RecordArtifactFile {
        self.core.candidate.catalog_candidate()
    }

    pub fn source_root_generation(&self) -> u64 {
        self.core.candidate.source_root().generation()
    }

    pub fn candidate_root_generation(&self) -> u64 {
        self.core.candidate.successor_root().generation()
    }

    pub fn candidate_synchronization_count(&self) -> usize {
        self.core.candidate_synchronizations.len()
    }

    pub fn publication_identity_digest(&self) -> [u8; 32] {
        self.core.identity.stable_digest()
    }

    pub(in crate::physical_runtime) const fn identity(&self) -> PhysicalRootPublicationIdentity {
        self.core.identity()
    }

    pub(in crate::physical_runtime) fn into_core(self) -> RootPublicationPreparedCore {
        self.core
    }
}

impl RootPublicationPreparedCore {
    pub(in crate::physical_runtime) const fn identity(&self) -> PhysicalRootPublicationIdentity {
        self.identity
    }

    pub(in crate::physical_runtime) const fn group(&self) -> PhysicalDurabilityGroupBasis {
        self.group
    }

    pub(in crate::physical_runtime) fn members(&self) -> &[PhysicalRootPublicationMemberIdentity] {
        &self.member_identities
    }

    pub(in crate::physical_runtime) fn settled_members(
        &self,
    ) -> &[RootPublicationPhysicalMutationMember] {
        self.members.as_slice()
    }

    pub(in crate::physical_runtime) const fn candidate(&self) -> &PreparedPhysicalRootCandidate {
        &self.candidate
    }

    pub(in crate::physical_runtime) fn require_inspection(&mut self) {
        self.transition.require_inspection();
    }

    pub(in crate::physical_runtime) fn transition_matches(&self) -> bool {
        self.transition.identity() == self.identity
            && self.transition.source_root() == self.candidate.source_root()
    }

    pub(in crate::physical_runtime) fn release_transition(
        self,
    ) -> (
        PreparedPhysicalRootCandidate,
        NonEmpty<RootPublicationPhysicalMutationMember>,
    ) {
        self.transition.release();
        (self.candidate, self.members)
    }
}

mod identity_digest;
mod invariant_family_identity;
mod validator_family_identity;

pub(super) use identity_digest::legality_family_identity_digest;
pub use identity_digest::WorthTopologyLegalityFamilyIdentityDigest;
pub use invariant_family_identity::WorthTopologyInvariantFamilyIdentity;
use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictOverlapIdentityInput,
    ConflictPriorProofInput, ConflictRoutingPosture, ConflictRoutingVocabularyError,
};
pub use validator_family_identity::WorthTopologyValidatorFamilyIdentity;

use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
use crate::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictFamilyApplicability,
    TopologyConflictFamilyIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthTopologyLegalityFamilyIdentity {
    Validator(WorthTopologyValidatorFamilyIdentity),
    Invariant(WorthTopologyInvariantFamilyIdentity),
}

impl WorthTopologyLegalityFamilyIdentity {
    pub fn stable_key(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.stable_key(),
            Self::Invariant(identity) => identity.stable_key(),
        }
    }

    pub fn identity_digest(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.identity_digest(),
            Self::Invariant(identity) => identity.identity_digest(),
        }
    }

    pub fn authority_digest(&self) -> &WorthTopologyLegalityFamilyIdentityDigest {
        match self {
            Self::Validator(identity) => identity.authority_digest(),
            Self::Invariant(identity) => identity.authority_digest(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Validator(identity) => identity.name(),
            Self::Invariant(identity) => identity.name(),
        }
    }

    pub fn conflict_participant_identity(
        &self,
    ) -> Result<
        schema::facade::platform::authority::touched_graph_conflict::ConflictParticipantIdentity,
        schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingVocabularyError,
    > {
        match self {
            Self::Validator(identity) => identity.conflict_participant_identity(),
            Self::Invariant(identity) => identity.conflict_participant_identity(),
        }
    }

    pub(crate) fn matching_conflict_family_identities(
        &self,
        touched_closure: &DerivedInvalidationTouchedClosure,
    ) -> Result<Vec<TopologyConflictFamilyIdentity>, ConflictRoutingVocabularyError> {
        let locality = touched_closure.conflict_locality_identity()?;
        let participant = self.conflict_participant_identity()?;
        let contract = admit_conflict_routing_contract(
            admit_conflict_overlap_identity(ConflictOverlapIdentityInput::validator(
                locality,
                vec![participant],
            ))?,
            ConflictPriorProofInput::none(),
            ConflictRoutingPosture::RequiresFamilySelection,
        );
        let closeout = current_topology_conflict_family_catalog_closeout()
            .expect("topology conflict family catalog closes");
        Ok(closeout
            .catalog()
            .matching_families(
                &contract,
                TopologyConflictFamilyApplicability::Validator {
                    touched_closure,
                    identity: self,
                },
            )
            .into_iter()
            .map(|declaration| declaration.identity())
            .collect())
    }

    pub(crate) fn matching_conflict_family_identities_for_contract(
        &self,
        touched_closure: &DerivedInvalidationTouchedClosure,
        contract: &schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract,
    ) -> Vec<TopologyConflictFamilyIdentity> {
        let closeout = current_topology_conflict_family_catalog_closeout()
            .expect("topology conflict family catalog closes");
        closeout
            .catalog()
            .matching_families(
                contract,
                TopologyConflictFamilyApplicability::Validator {
                    touched_closure,
                    identity: self,
                },
            )
            .into_iter()
            .map(|declaration| declaration.identity())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use schema::facade::platform::authority::touched_graph_conflict::ConflictParticipantAuthority;

    use super::{WorthTopologyInvariantFamilyIdentity, WorthTopologyValidatorFamilyIdentity};
    use crate::validation::ownership_rule;

    #[test]
    fn validator_family_identity_admits_shared_validator_participant_identity() {
        let identity = WorthTopologyValidatorFamilyIdentity::from_registered_rule(ownership_rule());

        let participant = identity
            .conflict_participant_identity()
            .expect("validator participant admits");

        assert_eq!(
            participant.authority(),
            ConflictParticipantAuthority::Validator
        );
        assert_eq!(participant.digest(), identity.identity_digest());
    }

    #[test]
    fn invariant_family_identity_admits_shared_validator_participant_identity() {
        let identity = WorthTopologyInvariantFamilyIdentity::registered("loop-closure", "v1");

        let participant = identity
            .conflict_participant_identity()
            .expect("invariant participant admits");

        assert_eq!(
            participant.authority(),
            ConflictParticipantAuthority::Validator
        );
        assert_eq!(participant.digest(), identity.identity_digest());
    }
}

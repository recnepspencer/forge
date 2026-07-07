#[cfg(test)]
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

#[cfg(test)]
use super::applicability::matches_declaration;
use super::family_declaration::TopologyConflictFamilyDeclaration;
use super::family_identity::TopologyConflictFamilyIdentity;
#[cfg(test)]
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
#[cfg(test)]
use crate::validator_invariant_catalog::WorthTopologyLegalityFamilyIdentity;

#[cfg(test)]
#[derive(Debug)]
pub enum TopologyConflictFamilyApplicability<'a> {
    AspectLocality {
        touched_closure: &'a DerivedInvalidationTouchedClosure,
    },
    Validator {
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        identity: &'a WorthTopologyLegalityFamilyIdentity,
    },
    ReplayBoundary {
        touched_closure: &'a DerivedInvalidationTouchedClosure,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyCatalog {
    declarations: Vec<TopologyConflictFamilyDeclaration>,
}

impl TopologyConflictFamilyCatalog {
    pub fn new(declarations: Vec<TopologyConflictFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[TopologyConflictFamilyDeclaration] {
        &self.declarations
    }

    pub fn family(
        &self,
        identity: TopologyConflictFamilyIdentity,
    ) -> Option<&TopologyConflictFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }

    #[cfg(test)]
    pub(crate) fn matching_families(
        &self,
        contract: &ConflictRoutingContract,
        applicability: TopologyConflictFamilyApplicability<'_>,
    ) -> Vec<&TopologyConflictFamilyDeclaration> {
        match applicability {
            TopologyConflictFamilyApplicability::AspectLocality { touched_closure } => {
                self.declarations
                    .iter()
                    .filter(|declaration| {
                        declaration.admits_aspect_locality_selection()
                            && matches_declaration(declaration, contract)
                            && contract.overlap_identity().locality_identity().is_some_and(
                                |locality| {
                                    locality.authority_digest() == touched_closure.closure_digest()
                                },
                            )
                    })
                    .collect()
            }
            TopologyConflictFamilyApplicability::Validator {
                touched_closure,
                identity,
            } => {
                let participant = identity.conflict_participant_identity().ok();
                self.declarations
                    .iter()
                    .filter(|declaration| {
                        declaration.admits_validator_selection()
                            && matches_declaration(declaration, contract)
                            && contract.overlap_identity().locality_identity().is_some_and(
                                |locality| {
                                    locality.authority_digest() == touched_closure.closure_digest()
                                },
                            )
                            && participant.as_ref().is_some_and(|participant| {
                                contract
                                    .overlap_identity()
                                    .participants()
                                    .iter()
                                    .any(|candidate| candidate.digest() == participant.digest())
                            })
                    })
                    .collect()
            }
            TopologyConflictFamilyApplicability::ReplayBoundary { touched_closure } => {
                self.declarations
                    .iter()
                    .filter(|declaration| {
                        declaration.admits_replay_boundary_selection()
                            && matches_declaration(declaration, contract)
                            && contract.overlap_identity().locality_identity().is_some_and(
                                |locality| {
                                    locality.authority_digest() == touched_closure.closure_digest()
                                },
                            )
                    })
                    .collect()
            }
        }
    }
}

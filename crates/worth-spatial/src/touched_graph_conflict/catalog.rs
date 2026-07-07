#[cfg(test)]
use schema::facade::platform::authority::touched_graph_conflict::ConflictRoutingContract;

#[cfg(test)]
use super::applicability::matches_declaration;
use super::family_declaration::SpatialConflictFamilyDeclaration;
use super::family_identity::SpatialConflictFamilyIdentity;
#[cfg(test)]
use crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority;

#[cfg(test)]
#[derive(Debug)]
pub enum SpatialConflictFamilyApplicability<'a> {
    EvidenceLookup {
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
    },
    ReplayBoundary {
        authority: &'a SpatialGeometryEvidenceTouchAuthority,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyCatalog {
    declarations: Vec<SpatialConflictFamilyDeclaration>,
}

impl SpatialConflictFamilyCatalog {
    pub fn new(declarations: Vec<SpatialConflictFamilyDeclaration>) -> Self {
        Self { declarations }
    }

    pub fn declarations(&self) -> &[SpatialConflictFamilyDeclaration] {
        &self.declarations
    }

    pub fn family(
        &self,
        identity: SpatialConflictFamilyIdentity,
    ) -> Option<&SpatialConflictFamilyDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.identity() == identity)
    }

    #[cfg(test)]
    pub(crate) fn matching_families(
        &self,
        contract: &ConflictRoutingContract,
        applicability: SpatialConflictFamilyApplicability<'_>,
    ) -> Vec<&SpatialConflictFamilyDeclaration> {
        match applicability {
            SpatialConflictFamilyApplicability::EvidenceLookup { authority } => {
                let authority_participant = authority.conflict_participant_identity().ok();
                self.declarations
                    .iter()
                    .filter(|declaration_row| {
                        declaration_row.admits_evidence_selection()
                            && matches_declaration(declaration_row, contract)
                            && contract.overlap_identity().locality_identity().is_some_and(
                                |locality| {
                                    locality.authority_digest() == authority.digest().as_str()
                                },
                            )
                            && authority_participant.as_ref().is_some_and(|participant| {
                                contract
                                    .overlap_identity()
                                    .participants()
                                    .iter()
                                    .any(|candidate| candidate.digest() == participant.digest())
                            })
                    })
                    .collect()
            }
            SpatialConflictFamilyApplicability::ReplayBoundary { authority } => {
                self.declarations
                    .iter()
                    .filter(|declaration| {
                        declaration.admits_replay_boundary_selection()
                            && matches_declaration(declaration, contract)
                            && contract.overlap_identity().locality_identity().is_some_and(
                                |locality| {
                                    locality.authority_digest() == authority.digest().as_str()
                                },
                            )
                    })
                    .collect()
            }
        }
    }
}

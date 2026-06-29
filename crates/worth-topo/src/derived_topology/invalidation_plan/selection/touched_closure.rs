use forge_query::facade::ForgeQueryGraphTouchDescriptor;
use schema::facade::platform::authority::touched_graph_conflict::{
    admit_conflict_overlap_identity, admit_conflict_routing_contract, ConflictLocalityIdentity,
    ConflictOverlapIdentityInput, ConflictRoutingContract, ConflictRoutingPosture,
    ConflictRoutingVocabularyError,
};
use schema::facade::platform::authority::touched_graph_conflict_internal::admit_conflict_topology_touched_closure_locality_identity_from_digest;

use crate::topology_operators::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphBasis, TopologyTouchedGraphCounters,
};
use crate::touched_graph_conflict::{
    current_topology_conflict_family_catalog_closeout, TopologyConflictFamilyApplicability,
    TopologyConflictFamilyIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationTouchedClosureDigest(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationTouchedClosure {
    semantic_family_key: &'static str,
    basis: TopologyTouchedGraphBasis,
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    closure_digest: DerivedInvalidationTouchedClosureDigest,
}

impl DerivedInvalidationTouchedClosure {
    pub fn from_declared_touch(proof: &TopologyDeclaredTouchedGraphBasisProof) -> Self {
        let basis = proof.basis().clone();
        let touch_descriptor = proof.touch_descriptor().clone();
        let closure_digest = closure_digest(basis.digest(), touch_descriptor.descriptor_digest());
        Self {
            semantic_family_key: proof.semantic_family_key(),
            basis,
            touch_descriptor,
            closure_digest,
        }
    }

    pub fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub fn basis(&self) -> &TopologyTouchedGraphBasis {
        &self.basis
    }

    pub fn basis_digest(&self) -> &str {
        self.basis.digest()
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.touch_descriptor.descriptor_digest()
    }

    pub const fn counters(&self) -> TopologyTouchedGraphCounters {
        self.basis.counters()
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_digest.as_str()
    }

    pub fn authority_digest(&self) -> &DerivedInvalidationTouchedClosureDigest {
        &self.closure_digest
    }

    pub fn conflict_locality_identity(
        &self,
    ) -> Result<ConflictLocalityIdentity, ConflictRoutingVocabularyError> {
        admit_conflict_topology_touched_closure_locality_identity_from_digest(
            self.authority_digest(),
        )
    }

    pub(crate) fn conflict_routing_contract(
        &self,
    ) -> Result<ConflictRoutingContract, ConflictRoutingVocabularyError> {
        let locality = self.conflict_locality_identity()?;
        let overlap =
            admit_conflict_overlap_identity(ConflictOverlapIdentityInput::locality(locality))?;
        Ok(admit_conflict_routing_contract(
            overlap,
            schema::facade::platform::authority::touched_graph_conflict::ConflictPriorProofInput::none(),
            ConflictRoutingPosture::RequiresFamilySelection,
        ))
    }

    pub(crate) fn matching_conflict_family_identities(
        &self,
    ) -> Result<Vec<TopologyConflictFamilyIdentity>, ConflictRoutingVocabularyError> {
        let contract = self.conflict_routing_contract()?;
        Ok(self.matching_aspect_or_locality_conflict_family_identities_for_contract(&contract))
    }

    pub(crate) fn matching_aspect_or_locality_conflict_family_identities_for_contract(
        &self,
        contract: &ConflictRoutingContract,
    ) -> Vec<TopologyConflictFamilyIdentity> {
        let closeout = current_topology_conflict_family_catalog_closeout()
            .expect("topology conflict family catalog closes");
        closeout
            .catalog()
            .matching_families(
                contract,
                TopologyConflictFamilyApplicability::AspectLocality {
                    touched_closure: self,
                },
            )
            .into_iter()
            .map(|declaration| declaration.identity())
            .collect()
    }

    pub(crate) fn matching_conflict_family_identities_for_contract(
        &self,
        contract: &ConflictRoutingContract,
    ) -> Vec<TopologyConflictFamilyIdentity> {
        let closeout = current_topology_conflict_family_catalog_closeout()
            .expect("topology conflict family catalog closes");
        closeout
            .catalog()
            .matching_families(
                contract,
                TopologyConflictFamilyApplicability::ReplayBoundary {
                    touched_closure: self,
                },
            )
            .into_iter()
            .map(|declaration| declaration.identity())
            .collect()
    }
}

fn closure_digest(
    basis_digest: &str,
    touch_descriptor_digest: &str,
) -> DerivedInvalidationTouchedClosureDigest {
    DerivedInvalidationTouchedClosureDigest(super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-touched-closure:v1".to_string(),
        format!("basis:{basis_digest}"),
        format!("query-touch:{touch_descriptor_digest}"),
    ]))
}

impl DerivedInvalidationTouchedClosureDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DerivedInvalidationTouchedClosureDigest {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

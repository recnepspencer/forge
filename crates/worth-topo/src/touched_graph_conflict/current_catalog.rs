use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::catalog::TopologyConflictFamilyCatalog;
use super::declaration_admission::{
    admit_topology_conflict_family_declaration, TopologyConflictFamilyDeclarationInput,
};
use super::family_declaration::{
    TopologyConflictDiagnosticWitness, TopologyConflictLocalityAuthorityRequirement,
    TopologyConflictPriorProofPosture, TopologyConflictSelectionProductPosture,
};
use super::family_identity::{
    admit_topology_conflict_family_identity, TopologyConflictFamilyIdentityAuthority,
};

pub(crate) fn current_topology_conflict_family_catalog() -> TopologyConflictFamilyCatalog {
    TopologyConflictFamilyCatalog::new(vec![
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::aspect_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Aspect,
            secondary_overlap_category: Some(ConflictOverlapCategory::Locality),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::TouchedClosureDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::validator_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::Validator,
            secondary_overlap_category: None,
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: TopologyConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ValidatorFamilyDigest,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_topology_conflict_family_declaration(TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture:
                TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            diagnostic_witness: TopologyConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ])
}

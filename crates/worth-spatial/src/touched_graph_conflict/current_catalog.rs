use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};

use super::catalog::SpatialConflictFamilyCatalog;
use super::declaration_admission::{
    admit_spatial_conflict_family_declaration, SpatialConflictFamilyDeclarationInput,
};
use super::family_declaration::{
    SpatialConflictDiagnosticWitness, SpatialConflictLocalityAuthorityRequirement,
    SpatialConflictPriorProofPosture, SpatialConflictSelectionProductPosture,
};
use super::family_identity::{
    admit_spatial_conflict_family_identity, SpatialConflictFamilyIdentityAuthority,
};

pub(crate) fn current_spatial_conflict_family_catalog() -> SpatialConflictFamilyCatalog {
    SpatialConflictFamilyCatalog::new(vec![
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::evidence_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::Evidence,
            secondary_overlap_category: None,
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: SpatialConflictPriorProofPosture::NoPriorProofRequired,
            diagnostic_witness: SpatialConflictDiagnosticWitness::EvidenceFamilyDigest,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
        admit_spatial_conflict_family_declaration(SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            locality_authority_requirement:
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            routing_posture: ConflictRoutingPosture::RequiresFamilySelection,
            prior_proof_posture: SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            diagnostic_witness: SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
            selection_product_posture:
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
        }),
    ])
}

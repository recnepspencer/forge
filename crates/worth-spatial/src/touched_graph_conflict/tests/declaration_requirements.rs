use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::touched_graph_conflict::{
    admit_spatial_conflict_family_declaration, admit_spatial_conflict_family_identity,
    current_spatial_conflict_family_catalog_closeout, SpatialConflictDiagnosticWitness,
    SpatialConflictFamilyDeclaration, SpatialConflictFamilyDeclarationInput,
    SpatialConflictFamilyIdentityAuthority, SpatialConflictLocalityAuthorityRequirement,
    SpatialConflictPriorProofPosture, SpatialConflictSelectionProductPosture,
};

#[test]
fn declarations_encode_explicit_requirement_digest_basis() {
    let closeout = current_spatial_conflict_family_catalog_closeout()
        .expect("spatial conflict family catalog closes");

    for declaration in closeout.catalog().declarations() {
        assert_eq!(
            declaration.declaration_digest(),
            expected_digest(declaration),
            "declaration digest should encode the full explicit spatial family basis",
        );
    }
}

#[test]
fn declaration_digest_changes_when_each_variable_field_changes() {
    let baseline = baseline_input();
    let baseline_digest = admit_spatial_conflict_family_declaration(baseline)
        .declaration_digest()
        .to_string();

    let variants = [
        SpatialConflictFamilyDeclarationInput {
            identity: admit_spatial_conflict_family_identity(
                SpatialConflictFamilyIdentityAuthority::replay_boundary_selection(),
            ),
            ..baseline
        },
        SpatialConflictFamilyDeclarationInput {
            primary_overlap_category: ConflictOverlapCategory::ReplayUndo,
            ..baseline
        },
        SpatialConflictFamilyDeclarationInput {
            secondary_overlap_category: Some(ConflictOverlapCategory::Transaction),
            ..baseline
        },
        SpatialConflictFamilyDeclarationInput {
            routing_posture: ConflictRoutingPosture::SerializableOnly,
            ..baseline
        },
        SpatialConflictFamilyDeclarationInput {
            prior_proof_posture: SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            ..baseline
        },
        SpatialConflictFamilyDeclarationInput {
            diagnostic_witness: SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
            ..baseline
        },
    ];

    for variant in variants {
        let changed = admit_spatial_conflict_family_declaration(variant);
        assert_ne!(baseline_digest, changed.declaration_digest());
    }
}

fn baseline_input() -> SpatialConflictFamilyDeclarationInput {
    SpatialConflictFamilyDeclarationInput {
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
    }
}

fn expected_digest(declaration: &SpatialConflictFamilyDeclaration) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:conflict-family-declaration:v1".to_string(),
            declaration.identity().digest(),
            format!(
                "locality:{:?}",
                declaration.locality_authority_requirement()
            ),
            format!("overlap:{:?}", declaration.primary_overlap_category()),
            format!(
                "overlap-secondary:{:?}",
                declaration.secondary_overlap_category()
            ),
            format!("routing:{:?}", declaration.routing_posture()),
            format!("prior-proof:{:?}", declaration.prior_proof_posture()),
            format!("diagnostic:{:?}", declaration.diagnostic_witness()),
            format!("selection:{:?}", declaration.selection_product_posture()),
        ],
    )
}

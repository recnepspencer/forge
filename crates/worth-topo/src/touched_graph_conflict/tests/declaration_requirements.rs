use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::touched_graph_conflict::{
    admit_topology_conflict_family_declaration, admit_topology_conflict_family_identity,
    current_topology_conflict_family_catalog_closeout, TopologyConflictDiagnosticWitness,
    TopologyConflictFamilyDeclaration, TopologyConflictFamilyDeclarationInput,
    TopologyConflictFamilyIdentityAuthority, TopologyConflictLocalityAuthorityRequirement,
    TopologyConflictPriorProofPosture, TopologyConflictSelectionProductPosture,
};

#[test]
fn declarations_encode_explicit_requirement_digest_basis() {
    let closeout = current_topology_conflict_family_catalog_closeout()
        .expect("topology conflict family catalog closes");

    for declaration in closeout.catalog().declarations() {
        assert_eq!(
            declaration.declaration_digest(),
            expected_digest(declaration),
            "declaration digest should encode the full explicit topology family basis",
        );
    }
}

#[test]
fn declaration_digest_changes_when_each_variable_field_changes() {
    let baseline = baseline_input();
    let baseline_digest = admit_topology_conflict_family_declaration(baseline)
        .declaration_digest()
        .to_string();

    let variants = [
        TopologyConflictFamilyDeclarationInput {
            identity: admit_topology_conflict_family_identity(
                TopologyConflictFamilyIdentityAuthority::validator_selection(),
            ),
            ..baseline
        },
        TopologyConflictFamilyDeclarationInput {
            primary_overlap_category: ConflictOverlapCategory::Validator,
            ..baseline
        },
        TopologyConflictFamilyDeclarationInput {
            secondary_overlap_category: None,
            ..baseline
        },
        TopologyConflictFamilyDeclarationInput {
            routing_posture: ConflictRoutingPosture::SerializableOnly,
            ..baseline
        },
        TopologyConflictFamilyDeclarationInput {
            prior_proof_posture: TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
            ..baseline
        },
        TopologyConflictFamilyDeclarationInput {
            diagnostic_witness: TopologyConflictDiagnosticWitness::ValidatorFamilyDigest,
            ..baseline
        },
    ];

    for variant in variants {
        let changed = admit_topology_conflict_family_declaration(variant);
        assert_ne!(baseline_digest, changed.declaration_digest());
    }
}

fn baseline_input() -> TopologyConflictFamilyDeclarationInput {
    TopologyConflictFamilyDeclarationInput {
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
    }
}

fn expected_digest(declaration: &TopologyConflictFamilyDeclaration) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:conflict-family-declaration:v1".to_string(),
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

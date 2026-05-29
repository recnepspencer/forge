use forge_proof::{ProofOutcomeKind, RecipeStageKind};

use crate::application::ForgeQueryDeclarationLegalityContract;

mod fixtures;
mod outcomes;

use fixtures::{admitted_handle, legal, progressed, AdmittedFamily, Declaration};

#[test]
fn admitted_progression_yields_proof_bearing_artifact() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .progress_declaration(legal(
            &handle,
            Declaration::<AdmittedFamily>::new("edge:42"),
        ))
        .unwrap_or_else(|_| panic!("progression should admit"));

    assert_eq!(progressed.declaration_family_key(), "split-edge");
    assert_eq!(progressed.outcome().kind(), ProofOutcomeKind::Success);
    assert_eq!(progressed.stage(), RecipeStageKind::Admitted);
    assert_eq!(
        progressed.legality_contract(),
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    );
}

#[test]
fn recipe_lane_matches_convenience_lane() {
    let handle = admitted_handle("collaborative");
    let legal = legal(&handle, Declaration::<AdmittedFamily>::new("edge:42"));
    let recipe = handle.declaration_progression_recipe(legal);
    assert_eq!(recipe.stage(), RecipeStageKind::Unresolved);

    let progressed_from_recipe = handle
        .progress_declaration_recipe(recipe)
        .unwrap_or_else(|_| panic!("recipe progression should admit"));
    let progressed_from_convenience =
        progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));

    assert_eq!(
        progressed_from_recipe.progression_digest(),
        progressed_from_convenience.progression_digest()
    );
}

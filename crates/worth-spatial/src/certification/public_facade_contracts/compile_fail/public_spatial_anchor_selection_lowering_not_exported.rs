use worth_spatial::facade::anchor_selection::{
    AuthorSpatialAnchorSelectionIntent, SpatialAnchorSelectionDeclarationEntry, SpatialMoveSpec,
};
use worth_spatial::facade::placement::SpatialPlacementSpec;
use worth_spatial::facade::refs::EmptySpatialWitnessCatalog;

fn main() {
    let declaration = SpatialAnchorSelectionDeclarationEntry::from_author_intent_with_catalog(
        AuthorSpatialAnchorSelectionIntent::Move(
            SpatialMoveSpec::shape_origin().to([1.0, 2.0, 3.0]),
        ),
        &EmptySpatialWitnessCatalog,
    );
    let _ = declaration.apply_to_placement_with_catalog(
        SpatialPlacementSpec::world(),
        &EmptySpatialWitnessCatalog,
    );
}

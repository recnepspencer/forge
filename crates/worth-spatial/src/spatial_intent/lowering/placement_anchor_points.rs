use crate::spatial_intent::refs::SpatialGeometricTagFailureClass;
use crate::spatial_intent::resolution::SpatialFrameError;
use crate::spatial_intent::resolution::SpatialWitnessFailureClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialPlacementPointAnchorError {
    UnsupportedAnchor,
    InvalidReferenceFrame(SpatialFrameError),
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    AnchorTagFailure(SpatialGeometricTagFailureClass),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialPlacementPointAnchorKind {
    SubjectOwnedPoint,
    ExternalReferencePoint,
    FeatureOwnedPoint,
    GeometricTagPoint,
}

#[cfg(test)]
mod tests {
    use super::SpatialPlacementPointAnchorKind;
    use crate::facade::{SpatialAnchorRef, SpatialFrameRef, SpatialPlacementSpec};
    use crate::spatial_intent::lowering::placement_anchor_progression::{
        lower_supported_subject_anchor_with_catalog, lower_supported_translation_anchor,
    };
    use crate::spatial_intent::refs::{
        SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass,
    };
    use crate::test_support::SpatialFixtureWitnessCatalog;

    #[test]
    fn translation_anchor_lowering_preserves_point_anchor_semantics() {
        let world = SpatialPlacementSpec::world();
        let shape_origin =
            lower_supported_translation_anchor(&world, &SpatialAnchorRef::shape_origin())
                .expect("shape-origin translation anchor");
        let world_origin =
            lower_supported_translation_anchor(&world, &SpatialAnchorRef::world_origin())
                .expect("world-origin translation anchor");
        let frame_origin = lower_supported_translation_anchor(
            &world,
            &SpatialAnchorRef::frame_origin(SpatialFrameRef::workplane(
                "wp-kind",
                [2.0, 3.0, 4.0],
                [0.0, 0.0, 1.0],
            )),
        )
        .expect("frame-origin translation anchor");

        assert_eq!(
            shape_origin.payload().kind(),
            SpatialPlacementPointAnchorKind::SubjectOwnedPoint
        );
        assert_eq!(
            world_origin.payload().kind(),
            SpatialPlacementPointAnchorKind::ExternalReferencePoint
        );
        assert_eq!(
            frame_origin.payload().kind(),
            SpatialPlacementPointAnchorKind::ExternalReferencePoint
        );
        assert_eq!(shape_origin.payload().world_point(), [0.0, 0.0, 0.0]);
        assert_eq!(world_origin.payload().world_point(), [0.0, 0.0, 0.0]);
        assert_eq!(frame_origin.payload().world_point(), [2.0, 3.0, 4.0]);
    }

    #[test]
    fn subject_anchor_lowering_preserves_feature_and_tag_semantics_with_catalog() {
        let world = SpatialPlacementSpec::world();
        let catalog = SpatialFixtureWitnessCatalog::new()
            .with_feature_owned_point(
                "feature-anchor",
                crate::facade::SpatialCarrierPointRole::Anchor,
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [1.0, 2.0, 3.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            )
            .with_geometric_tag_point(
                "tag-anchor",
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [4.0, 5.0, 6.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            );
        let feature_owned = lower_supported_subject_anchor_with_catalog(
            &world,
            &SpatialAnchorRef::feature_owned("feature-anchor"),
            &catalog,
        )
        .expect("feature-owned subject anchor");
        let geometric_tag = lower_supported_subject_anchor_with_catalog(
            &world,
            &SpatialAnchorRef::geometric_tag("tag-anchor"),
            &catalog,
        )
        .expect("geometric-tag subject anchor");

        assert_eq!(
            feature_owned.payload().kind(),
            SpatialPlacementPointAnchorKind::FeatureOwnedPoint
        );
        assert_eq!(
            geometric_tag.payload().kind(),
            SpatialPlacementPointAnchorKind::GeometricTagPoint
        );
        assert_eq!(feature_owned.payload().world_point(), [1.0, 2.0, 3.0]);
        assert_eq!(geometric_tag.payload().world_point(), [4.0, 5.0, 6.0]);
    }
}

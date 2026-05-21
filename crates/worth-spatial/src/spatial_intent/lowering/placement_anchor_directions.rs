use crate::spatial_intent::resolution::SpatialWitnessFailureClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialPlacementDirectionalAnchorError {
    UnsupportedAnchor,
    AmbiguousAnchorMeaning,
    AnchorWitnessFailure(SpatialWitnessFailureClass),
    InvalidExistingPlacement,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SpatialPlacementReorientAnchorMode {
    PointLike,
    Directional([f64; 3]),
}

pub(crate) fn lowered_reorient_facing_from_directional_anchor(
    placement: &crate::spatial_intent::lowering::SpatialPlacementSpec,
    source_world_direction: [f64; 3],
    target_world_direction: [f64; 3],
) -> Result<[f64; 3], SpatialPlacementDirectionalAnchorError> {
    let admitted = crate::spatial_intent::lowering::admit_spatial_placement(placement.clone())
        .map_err(|_| SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?;
    rotate_vector_to_align_source(
        admitted.facing_vector(),
        source_world_direction,
        target_world_direction,
    )
}

fn rotate_vector_to_align_source(
    vector: [f64; 3],
    source: [f64; 3],
    target: [f64; 3],
) -> Result<[f64; 3], SpatialPlacementDirectionalAnchorError> {
    let source = normalize(source)
        .ok_or(SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?;
    let target = normalize(target)
        .ok_or(SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?;
    let dot = linalg::dot(source, target).clamp(-1.0, 1.0);
    if dot >= 1.0 - 1.0e-12 {
        return Ok(vector);
    }
    let cross = linalg::cross(source, target);
    let axis = if norm_sq(cross) <= f64::MIN_POSITIVE {
        fallback_perpendicular(source)
            .ok_or(SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?
    } else {
        normalize(cross).ok_or(SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)?
    };
    let angle = if norm_sq(cross) <= f64::MIN_POSITIVE {
        std::f64::consts::PI
    } else {
        dot.acos()
    };
    normalize(rotate_vector(vector, axis, angle))
        .ok_or(SpatialPlacementDirectionalAnchorError::InvalidExistingPlacement)
}

fn rotate_vector(vector: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let cos = angle.cos();
    let sin = angle.sin();
    let cross = linalg::cross(axis, vector);
    let dot = linalg::dot(axis, vector);
    [
        vector[0] * cos + cross[0] * sin + axis[0] * dot * (1.0 - cos),
        vector[1] * cos + cross[1] * sin + axis[1] * dot * (1.0 - cos),
        vector[2] * cos + cross[2] * sin + axis[2] * dot * (1.0 - cos),
    ]
}

fn fallback_perpendicular(parallel: [f64; 3]) -> Option<[f64; 3]> {
    let reference = if parallel[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    normalize(linalg::cross(parallel, reference))
}

fn normalize(vector: [f64; 3]) -> Option<[f64; 3]> {
    let magnitude_sq = norm_sq(vector);
    if !magnitude_sq.is_finite() || magnitude_sq <= f64::MIN_POSITIVE {
        return None;
    }
    let magnitude = magnitude_sq.sqrt();
    Some([
        vector[0] / magnitude,
        vector[1] / magnitude,
        vector[2] / magnitude,
    ])
}

mod linalg {
    pub(super) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    pub(super) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }
}

fn norm_sq(vector: [f64; 3]) -> f64 {
    vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]
}

#[cfg(test)]
mod tests {
    use super::{
        lowered_reorient_facing_from_directional_anchor, SpatialPlacementDirectionalAnchorError,
        SpatialPlacementReorientAnchorMode,
    };
    use crate::facade::{
        SpatialAnchorRef, SpatialAxis, SpatialCatalogResolvedDirectionWitness,
        SpatialCatalogResolvedPointWitness, SpatialCatalogWitnessResolutionClass, SpatialFrameRef,
        SpatialPlacementSpec, SpatialWitnessFailureClass,
    };
    use crate::spatial_intent::lowering::placement_anchor_progression::{
        lower_supported_reorient_anchor, lower_supported_reorient_anchor_with_catalog,
    };
    use crate::test_support::SpatialFixtureWitnessCatalog;

    #[test]
    fn reorient_anchor_lowering_distinguishes_point_and_directional_meaning() {
        let shape_u_axis = lower_supported_reorient_anchor(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::shape_axis(SpatialAxis::U),
        )
        .expect("shape-u-axis lowering");
        let shape_v_axis = lower_supported_reorient_anchor(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::shape_axis(SpatialAxis::V),
        )
        .expect("shape-v-axis lowering");
        let shape_w_axis = lower_supported_reorient_anchor(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::shape_axis(SpatialAxis::W),
        )
        .expect("shape-w-axis lowering");
        let frame_axis = lower_supported_reorient_anchor(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::frame_axis(SpatialFrameRef::world(), SpatialAxis::U),
        )
        .expect("frame-axis lowering");

        assert!(matches!(
            shape_u_axis.payload(),
            SpatialPlacementReorientAnchorMode::Directional(_)
        ));
        assert!(matches!(
            shape_v_axis.payload(),
            SpatialPlacementReorientAnchorMode::Directional(_)
        ));
        assert!(matches!(
            shape_w_axis.payload(),
            SpatialPlacementReorientAnchorMode::Directional(_)
        ));
        assert!(matches!(
            frame_axis.payload(),
            SpatialPlacementReorientAnchorMode::Directional(_)
        ));
    }

    #[test]
    fn reorient_anchor_lowering_preserves_feature_owned_ambiguity() {
        let catalog = SpatialFixtureWitnessCatalog::new()
            .with_feature_owned_point(
                "feature-axis",
                crate::facade::SpatialCarrierPointRole::Anchor,
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [0.0, 0.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            )
            .with_feature_owned_direction(
                "feature-axis",
                crate::facade::SpatialCarrierDirectionRole::Axis,
                Ok(SpatialCatalogResolvedDirectionWitness::new(
                    [1.0, 0.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            );
        let error = lower_supported_reorient_anchor_with_catalog(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::feature_owned("feature-axis"),
            &catalog,
        )
        .err()
        .expect("feature-owned point+direction meaning should stay ambiguous");

        assert_eq!(
            error,
            SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning
        );
    }

    #[test]
    fn directional_reorient_can_rotate_subject_facing_from_external_axis_delta() {
        let facing = lowered_reorient_facing_from_directional_anchor(
            &SpatialPlacementSpec::world(),
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        )
        .expect("derived facing");

        assert!(facing[0] < -0.99);
    }

    #[test]
    fn feature_owned_directional_failures_stay_typed_when_point_meaning_is_absent() {
        let error = lower_supported_reorient_anchor_with_catalog(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::feature_owned("feature-axis"),
            &SpatialFixtureWitnessCatalog::new().with_feature_owned_direction(
                "feature-axis",
                crate::facade::SpatialCarrierDirectionRole::Axis,
                Err(SpatialWitnessFailureClass::Exhausted),
            ),
        )
        .err()
        .expect("feature-owned directional failure should stay typed");

        assert_eq!(
            error,
            SpatialPlacementDirectionalAnchorError::AnchorWitnessFailure(
                SpatialWitnessFailureClass::Exhausted
            )
        );
    }

    #[test]
    fn feature_owned_directional_lowering_does_not_mask_ambiguous_point_meaning() {
        let error = lower_supported_reorient_anchor_with_catalog(
            &SpatialPlacementSpec::world(),
            &SpatialAnchorRef::feature_owned("feature-axis"),
            &SpatialFixtureWitnessCatalog::new()
                .with_feature_owned_point(
                    "feature-axis",
                    crate::facade::SpatialCarrierPointRole::Anchor,
                    Err(SpatialWitnessFailureClass::Ambiguous),
                )
                .with_feature_owned_direction(
                    "feature-axis",
                    crate::facade::SpatialCarrierDirectionRole::Axis,
                    Ok(SpatialCatalogResolvedDirectionWitness::new(
                        [1.0, 0.0, 0.0],
                        SpatialCatalogWitnessResolutionClass::CarrierDerived,
                    )),
                ),
        )
        .err()
        .expect("ambiguous point-side meaning should stay ambiguous");

        assert_eq!(
            error,
            SpatialPlacementDirectionalAnchorError::AmbiguousAnchorMeaning
        );
    }
}

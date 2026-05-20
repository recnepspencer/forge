use crate::spatial_intent::refs::{
    admit_spatial_frame, EmptySpatialWitnessCatalog, SpatialAxis, SpatialCarrierDirectionRole,
    SpatialDirectionWitnessRef, SpatialFrameBasis, SpatialWitnessCatalog,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialWitnessResolutionClass {
    DirectWorld,
    FrameDerived,
    CarrierDerived,
    FallbackDerived,
    Exhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialWitnessFailureClass {
    NonFinite,
    Ambiguous,
    Undefined,
    Unsupported,
    Degenerate,
    Coincident,
    Exhausted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSpatialDirectionWitness {
    requested: SpatialDirectionWitnessRef,
    resolved_world_direction: [f64; 3],
    resolution_class: SpatialWitnessResolutionClass,
}

impl ResolvedSpatialDirectionWitness {
    pub fn requested(&self) -> &SpatialDirectionWitnessRef {
        &self.requested
    }

    pub fn resolved_world_direction(&self) -> [f64; 3] {
        self.resolved_world_direction
    }

    pub fn resolution_class(&self) -> SpatialWitnessResolutionClass {
        self.resolution_class
    }
}

pub fn resolve_spatial_direction_witness(
    requested: SpatialDirectionWitnessRef,
) -> Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass> {
    resolve_spatial_direction_witness_with_catalog(requested, &EmptySpatialWitnessCatalog)
}

pub fn resolve_spatial_direction_witness_with_catalog(
    requested: SpatialDirectionWitnessRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialDirectionWitness, SpatialWitnessFailureClass> {
    let (resolved_world_direction, resolution_class) = match &requested {
        SpatialDirectionWitnessRef::WorldDirection(direction) => (
            normalize(*direction).ok_or(classify_direction(*direction))?,
            SpatialWitnessResolutionClass::DirectWorld,
        ),
        SpatialDirectionWitnessRef::FrameAxis { frame, axis } => {
            let admitted = admit_spatial_frame(frame.clone())
                .map_err(|_| SpatialWitnessFailureClass::Degenerate)?;
            let direction = axis_from_basis(admitted.basis(), *axis);
            (
                normalize(direction).ok_or(SpatialWitnessFailureClass::Degenerate)?,
                SpatialWitnessResolutionClass::FrameDerived,
            )
        }
        SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => {
            let admitted = admit_spatial_frame(frame.clone())
                .map_err(|_| SpatialWitnessFailureClass::Degenerate)?;
            let parallel = axis_from_basis(admitted.basis(), *axis);
            let perpendicular =
                fallback_perpendicular(parallel).ok_or(SpatialWitnessFailureClass::Exhausted)?;
            (
                perpendicular,
                SpatialWitnessResolutionClass::FallbackDerived,
            )
        }
        SpatialDirectionWitnessRef::CarrierDirection { .. } => {
            return Err(SpatialWitnessFailureClass::Ambiguous);
        }
        SpatialDirectionWitnessRef::ParameterSpaceDirection {
            carrier_kind,
            carrier,
            parameter,
            role,
        } => {
            if parameter.iter().any(|value| !value.is_finite()) {
                return Err(SpatialWitnessFailureClass::NonFinite);
            }
            let resolved = catalog.resolve_parameter_space_direction(
                *carrier_kind,
                carrier,
                *parameter,
                *role,
            )?;
            (
                normalize(resolved.world_direction())
                    .ok_or(classify_direction(resolved.world_direction()))?,
                resolved.resolution_class().as_witness_resolution_class(),
            )
        }
        SpatialDirectionWitnessRef::FeatureOwnedDirection { feature, role } => {
            let _role: SpatialCarrierDirectionRole = *role;
            let resolved = catalog.resolve_feature_owned_direction(feature, *role)?;
            (
                normalize(resolved.world_direction())
                    .ok_or(classify_direction(resolved.world_direction()))?,
                resolved.resolution_class().as_witness_resolution_class(),
            )
        }
    };
    Ok(ResolvedSpatialDirectionWitness {
        requested,
        resolved_world_direction,
        resolution_class,
    })
}

fn classify_direction(direction: [f64; 3]) -> SpatialWitnessFailureClass {
    if direction.iter().any(|value| !value.is_finite()) {
        SpatialWitnessFailureClass::NonFinite
    } else {
        SpatialWitnessFailureClass::Undefined
    }
}

fn axis_from_basis(basis: SpatialFrameBasis, axis: SpatialAxis) -> [f64; 3] {
    match axis {
        SpatialAxis::U => basis.u_axis(),
        SpatialAxis::V => basis.v_axis(),
        SpatialAxis::W => basis.w_axis(),
    }
}

fn fallback_perpendicular(parallel: [f64; 3]) -> Option<[f64; 3]> {
    let reference = if parallel[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    normalize([
        parallel[1] * reference[2] - parallel[2] * reference[1],
        parallel[2] * reference[0] - parallel[0] * reference[2],
        parallel[0] * reference[1] - parallel[1] * reference[0],
    ])
}

fn normalize(vector: [f64; 3]) -> Option<[f64; 3]> {
    if vector.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let magnitude_sq = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
    if magnitude_sq <= f64::MIN_POSITIVE {
        return None;
    }
    let magnitude = magnitude_sq.sqrt();
    Some([
        vector[0] / magnitude,
        vector[1] / magnitude,
        vector[2] / magnitude,
    ])
}

#[cfg(test)]
mod tests {
    use super::{
        resolve_spatial_direction_witness, resolve_spatial_direction_witness_with_catalog,
        SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
    };
    use crate::facade::{
        SpatialAxis, SpatialCarrierDirectionRole, SpatialCarrierKind,
        SpatialCatalogResolvedDirectionWitness, SpatialCatalogWitnessResolutionClass,
        SpatialDirectionWitnessRef, SpatialFixtureWitnessCatalog, SpatialFrameRef,
    };

    #[test]
    fn direction_witness_resolution_preserves_direct_frame_and_fallback_truth() {
        let world =
            resolve_spatial_direction_witness(SpatialDirectionWitnessRef::world_direction([
                0.0, 1.0, 1.0,
            ]))
            .expect("world");
        let frame = resolve_spatial_direction_witness(SpatialDirectionWitnessRef::frame_axis(
            SpatialFrameRef::workplane("wp-1", [0.0, 0.0, 5.0], [1.0, 0.0, 0.0]),
            SpatialAxis::W,
        ))
        .expect("frame");
        let perpendicular = resolve_spatial_direction_witness(
            SpatialDirectionWitnessRef::frame_perpendicular_axis(
                SpatialFrameRef::workplane("wp-2", [0.0, 0.0, 5.0], [0.0, 0.0, 1.0]),
                SpatialAxis::W,
            ),
        )
        .expect("perpendicular");

        assert_eq!(
            world.resolution_class(),
            SpatialWitnessResolutionClass::DirectWorld
        );
        assert_eq!(
            frame.resolution_class(),
            SpatialWitnessResolutionClass::FrameDerived
        );
        assert_eq!(
            perpendicular.resolution_class(),
            SpatialWitnessResolutionClass::FallbackDerived
        );
        assert_eq!(frame.resolved_world_direction(), [1.0, 0.0, 0.0]);
        assert!(perpendicular.resolved_world_direction()[2].abs() < 1.0e-12);
    }

    #[test]
    fn direction_witness_resolution_distinguishes_ambiguous_undefined_and_unsupported() {
        assert_eq!(
            resolve_spatial_direction_witness(SpatialDirectionWitnessRef::ambiguous_curve(
                "curve-1"
            ))
            .expect_err("ambiguous"),
            SpatialWitnessFailureClass::Ambiguous
        );
        assert_eq!(
            resolve_spatial_direction_witness(SpatialDirectionWitnessRef::world_direction([
                0.0, 0.0, 0.0,
            ]))
            .expect_err("undefined"),
            SpatialWitnessFailureClass::Undefined
        );
        assert_eq!(
            resolve_spatial_direction_witness(SpatialDirectionWitnessRef::surface_normal(
                "surface-1",
                0.5,
                0.5,
            ))
            .expect_err("unsupported"),
            SpatialWitnessFailureClass::Unsupported
        );
    }

    #[test]
    fn direction_witness_resolution_supports_catalog_backed_carrier_and_feature_truth() {
        let catalog = SpatialFixtureWitnessCatalog::new()
            .with_parameter_space_direction(
                SpatialCarrierKind::Curve,
                "curve-2",
                [0.25, 0.0],
                SpatialCarrierDirectionRole::Tangent,
                Ok(SpatialCatalogResolvedDirectionWitness::new(
                    [0.0, 1.0, 0.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            )
            .with_feature_owned_direction(
                "feature-1",
                SpatialCarrierDirectionRole::Axis,
                Ok(SpatialCatalogResolvedDirectionWitness::new(
                    [0.0, 0.0, 4.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            );

        let curve = resolve_spatial_direction_witness_with_catalog(
            SpatialDirectionWitnessRef::curve_tangent("curve-2", 0.25),
            &catalog,
        )
        .expect("curve tangent");
        let feature = resolve_spatial_direction_witness_with_catalog(
            SpatialDirectionWitnessRef::feature_axis("feature-1"),
            &catalog,
        )
        .expect("feature axis");

        assert_eq!(
            curve.resolution_class(),
            SpatialWitnessResolutionClass::CarrierDerived
        );
        assert_eq!(curve.resolved_world_direction(), [0.0, 1.0, 0.0]);
        assert_eq!(
            feature.resolution_class(),
            SpatialWitnessResolutionClass::FallbackDerived
        );
        assert_eq!(feature.resolved_world_direction(), [0.0, 0.0, 1.0]);
    }
}

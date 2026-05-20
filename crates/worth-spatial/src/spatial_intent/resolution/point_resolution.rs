use crate::spatial_intent::refs::{
    admit_spatial_frame, EmptySpatialWitnessCatalog, SpatialCarrierPointRole,
    SpatialPointWitnessRef, SpatialWitnessCatalog,
};
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedSpatialPointWitness {
    requested: SpatialPointWitnessRef,
    resolved_world_point: [f64; 3],
    resolution_class: SpatialWitnessResolutionClass,
}

impl ResolvedSpatialPointWitness {
    pub fn requested(&self) -> &SpatialPointWitnessRef {
        &self.requested
    }

    pub fn resolved_world_point(&self) -> [f64; 3] {
        self.resolved_world_point
    }

    pub fn resolution_class(&self) -> SpatialWitnessResolutionClass {
        self.resolution_class
    }
}

pub fn resolve_spatial_point_witness(
    requested: SpatialPointWitnessRef,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    resolve_spatial_point_witness_with_catalog(requested, &EmptySpatialWitnessCatalog)
}

pub fn resolve_spatial_point_witness_with_catalog(
    requested: SpatialPointWitnessRef,
    catalog: &impl SpatialWitnessCatalog,
) -> Result<ResolvedSpatialPointWitness, SpatialWitnessFailureClass> {
    let (resolved_world_point, resolution_class) = match &requested {
        SpatialPointWitnessRef::WorldPoint(point) => (
            finite_point(*point).ok_or(SpatialWitnessFailureClass::NonFinite)?,
            SpatialWitnessResolutionClass::DirectWorld,
        ),
        SpatialPointWitnessRef::FrameOrigin(frame) => {
            let admitted = admit_spatial_frame(frame.clone())
                .map_err(|_| SpatialWitnessFailureClass::Degenerate)?;
            (
                admitted.basis().origin(),
                SpatialWitnessResolutionClass::FrameDerived,
            )
        }
        SpatialPointWitnessRef::CarrierPoint { .. } => {
            return Err(SpatialWitnessFailureClass::Ambiguous);
        }
        SpatialPointWitnessRef::ParameterSpacePoint {
            carrier_kind,
            carrier,
            parameter,
        } => {
            if parameter.iter().any(|value| !value.is_finite()) {
                return Err(SpatialWitnessFailureClass::NonFinite);
            }
            let resolved =
                catalog.resolve_parameter_space_point(*carrier_kind, carrier, *parameter)?;
            (
                finite_point(resolved.world_point())
                    .ok_or(SpatialWitnessFailureClass::NonFinite)?,
                resolved.resolution_class().as_witness_resolution_class(),
            )
        }
        SpatialPointWitnessRef::FeatureOwnedPoint { feature, role } => {
            let _role: SpatialCarrierPointRole = *role;
            let resolved = catalog.resolve_feature_owned_point(feature, *role)?;
            (
                finite_point(resolved.world_point())
                    .ok_or(SpatialWitnessFailureClass::NonFinite)?,
                resolved.resolution_class().as_witness_resolution_class(),
            )
        }
    };
    Ok(ResolvedSpatialPointWitness {
        requested,
        resolved_world_point,
        resolution_class,
    })
}

fn finite_point(point: [f64; 3]) -> Option<[f64; 3]> {
    if point.iter().any(|value| !value.is_finite()) {
        None
    } else {
        Some(point)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        resolve_spatial_point_witness, resolve_spatial_point_witness_with_catalog,
        SpatialPointWitnessRef, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
    };
    use crate::facade::{
        SpatialCarrierKind, SpatialCarrierPointRole, SpatialCatalogResolvedPointWitness,
        SpatialCatalogWitnessResolutionClass, SpatialFixtureWitnessCatalog, SpatialFrameRef,
    };

    #[test]
    fn point_witness_resolution_preserves_direct_and_frame_truth() {
        let direct =
            resolve_spatial_point_witness(SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]))
                .expect("direct");
        let frame = resolve_spatial_point_witness(SpatialPointWitnessRef::frame_origin(
            SpatialFrameRef::workplane("wp-1", [4.0, 5.0, 6.0], [0.0, 0.0, 1.0]),
        ))
        .expect("frame");

        assert_eq!(
            direct.resolution_class(),
            SpatialWitnessResolutionClass::DirectWorld
        );
        assert_eq!(
            frame.resolution_class(),
            SpatialWitnessResolutionClass::FrameDerived
        );
        assert_eq!(direct.resolved_world_point(), [1.0, 2.0, 3.0]);
        assert_eq!(frame.resolved_world_point(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn point_witness_resolution_rejects_ambiguous_nonfinite_and_unsupported_refs() {
        assert_eq!(
            resolve_spatial_point_witness(SpatialPointWitnessRef::ambiguous_curve_point("curve-1"))
                .expect_err("ambiguous"),
            SpatialWitnessFailureClass::Ambiguous
        );
        assert_eq!(
            resolve_spatial_point_witness(SpatialPointWitnessRef::world_point([
                f64::NAN,
                0.0,
                0.0,
            ]))
            .expect_err("non-finite"),
            SpatialWitnessFailureClass::NonFinite
        );
        assert_eq!(
            resolve_spatial_point_witness(SpatialPointWitnessRef::curve_point("curve-2", 0.25,))
                .expect_err("unsupported"),
            SpatialWitnessFailureClass::Unsupported
        );
    }

    #[test]
    fn point_witness_resolution_supports_catalog_backed_carrier_and_feature_truth() {
        let catalog = SpatialFixtureWitnessCatalog::new()
            .with_parameter_space_point(
                SpatialCarrierKind::Surface,
                "surface-2",
                [0.5, 0.25],
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [8.0, 9.0, 10.0],
                    SpatialCatalogWitnessResolutionClass::CarrierDerived,
                )),
            )
            .with_feature_owned_point(
                "feature-2",
                SpatialCarrierPointRole::Origin,
                Ok(SpatialCatalogResolvedPointWitness::new(
                    [1.0, 1.5, 2.0],
                    SpatialCatalogWitnessResolutionClass::FallbackDerived,
                )),
            );

        let surface = resolve_spatial_point_witness_with_catalog(
            SpatialPointWitnessRef::surface_point("surface-2", 0.5, 0.25),
            &catalog,
        )
        .expect("surface point");
        let feature = resolve_spatial_point_witness_with_catalog(
            SpatialPointWitnessRef::feature_origin("feature-2"),
            &catalog,
        )
        .expect("feature point");

        assert_eq!(
            surface.resolution_class(),
            SpatialWitnessResolutionClass::CarrierDerived
        );
        assert_eq!(surface.resolved_world_point(), [8.0, 9.0, 10.0]);
        assert_eq!(
            feature.resolution_class(),
            SpatialWitnessResolutionClass::FallbackDerived
        );
        assert_eq!(feature.resolved_world_point(), [1.0, 1.5, 2.0]);
    }
}

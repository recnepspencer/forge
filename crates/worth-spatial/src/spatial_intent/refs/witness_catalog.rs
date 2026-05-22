use crate::spatial_intent::refs::{
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
};
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};
use worth_geom::{CanonicalParameterPoint, DomainParameterPoint, ParameterSpacePoint};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialCatalogWitnessResolutionClass {
    CarrierDerived,
    FallbackDerived,
}

impl SpatialCatalogWitnessResolutionClass {
    pub fn as_witness_resolution_class(&self) -> SpatialWitnessResolutionClass {
        match self {
            Self::CarrierDerived => SpatialWitnessResolutionClass::CarrierDerived,
            Self::FallbackDerived => SpatialWitnessResolutionClass::FallbackDerived,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialCatalogResolvedDirectionWitness {
    world_direction: [f64; 3],
    resolution_class: SpatialCatalogWitnessResolutionClass,
    parameter_admission: Option<SpatialCatalogParameterAdmission>,
}

impl SpatialCatalogResolvedDirectionWitness {
    pub fn new(
        world_direction: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
    ) -> Self {
        Self {
            world_direction,
            resolution_class,
            parameter_admission: None,
        }
    }

    pub fn with_parameter_admission(
        world_direction: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
        parameter_admission: SpatialCatalogParameterAdmission,
    ) -> Self {
        Self {
            world_direction,
            resolution_class,
            parameter_admission: Some(parameter_admission),
        }
    }

    pub fn world_direction(&self) -> [f64; 3] {
        self.world_direction
    }

    pub fn resolution_class(&self) -> SpatialCatalogWitnessResolutionClass {
        self.resolution_class
    }

    pub fn parameter_admission(&self) -> Option<&SpatialCatalogParameterAdmission> {
        self.parameter_admission.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialCatalogResolvedPointWitness {
    world_point: [f64; 3],
    resolution_class: SpatialCatalogWitnessResolutionClass,
    parameter_admission: Option<SpatialCatalogParameterAdmission>,
}

impl SpatialCatalogResolvedPointWitness {
    pub fn new(
        world_point: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
    ) -> Self {
        Self {
            world_point,
            resolution_class,
            parameter_admission: None,
        }
    }

    pub fn with_parameter_admission(
        world_point: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
        parameter_admission: SpatialCatalogParameterAdmission,
    ) -> Self {
        Self {
            world_point,
            resolution_class,
            parameter_admission: Some(parameter_admission),
        }
    }

    pub fn world_point(&self) -> [f64; 3] {
        self.world_point
    }

    pub fn resolution_class(&self) -> SpatialCatalogWitnessResolutionClass {
        self.resolution_class
    }

    pub fn parameter_admission(&self) -> Option<&SpatialCatalogParameterAdmission> {
        self.parameter_admission.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialCatalogParameterAdmission {
    requested: ParameterSpacePoint,
    domain_point: DomainParameterPoint,
    canonical_point: CanonicalParameterPoint,
    trimmed_posture: Option<SpatialCatalogTrimmedAdmissionPosture>,
}

impl SpatialCatalogParameterAdmission {
    pub fn new(
        requested: ParameterSpacePoint,
        domain_point: DomainParameterPoint,
        canonical_point: CanonicalParameterPoint,
    ) -> Self {
        Self {
            requested,
            domain_point,
            canonical_point,
            trimmed_posture: None,
        }
    }

    pub fn with_trimmed_posture(
        mut self,
        trimmed_posture: SpatialCatalogTrimmedAdmissionPosture,
    ) -> Self {
        self.trimmed_posture = Some(trimmed_posture);
        self
    }

    pub fn requested(&self) -> ParameterSpacePoint {
        self.requested
    }

    pub fn domain_point(&self) -> &DomainParameterPoint {
        &self.domain_point
    }

    pub fn canonical_point(&self) -> &CanonicalParameterPoint {
        &self.canonical_point
    }

    pub fn trimmed_posture(&self) -> Option<SpatialCatalogTrimmedAdmissionPosture> {
        self.trimmed_posture
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialCatalogTrimmedAdmissionPosture {
    PolygonalRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialCatalogResolvedGeometricTag {
    PointLike(SpatialCatalogResolvedPointWitness),
    DirectionLike(SpatialCatalogResolvedDirectionWitness),
    UnsupportedClass,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialGeometricTagFailureClass {
    Resolution(SpatialWitnessFailureClass),
    ResolvedDirectionLike,
    ResolvedUnsupportedClass,
}

pub trait SpatialWitnessCatalog {
    fn resolve_geometric_tag(
        &self,
        tag: &str,
    ) -> Result<SpatialCatalogResolvedGeometricTag, SpatialWitnessFailureClass>;

    fn resolve_geometric_tag_point(
        &self,
        tag: &str,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        match self.resolve_geometric_tag(tag)? {
            SpatialCatalogResolvedGeometricTag::PointLike(resolved) => Ok(resolved),
            SpatialCatalogResolvedGeometricTag::DirectionLike(_)
            | SpatialCatalogResolvedGeometricTag::UnsupportedClass => {
                Err(SpatialWitnessFailureClass::Unsupported)
            }
        }
    }

    fn resolve_parameter_space_direction(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: ParameterSpacePoint,
        role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>;

    fn resolve_feature_owned_direction(
        &self,
        feature: &str,
        role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>;

    fn resolve_parameter_space_point(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: ParameterSpacePoint,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>;

    fn resolve_feature_owned_point(
        &self,
        feature: &str,
        role: SpatialCarrierPointRole,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EmptySpatialWitnessCatalog;

impl SpatialWitnessCatalog for EmptySpatialWitnessCatalog {
    fn resolve_geometric_tag(
        &self,
        _tag: &str,
    ) -> Result<SpatialCatalogResolvedGeometricTag, SpatialWitnessFailureClass> {
        Err(SpatialWitnessFailureClass::Unsupported)
    }

    fn resolve_parameter_space_direction(
        &self,
        _carrier_kind: SpatialCarrierKind,
        _carrier: &str,
        _parameter: ParameterSpacePoint,
        _role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass> {
        Err(SpatialWitnessFailureClass::Unsupported)
    }

    fn resolve_feature_owned_direction(
        &self,
        _feature: &str,
        _role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass> {
        Err(SpatialWitnessFailureClass::Unsupported)
    }

    fn resolve_parameter_space_point(
        &self,
        _carrier_kind: SpatialCarrierKind,
        _carrier: &str,
        _parameter: ParameterSpacePoint,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        Err(SpatialWitnessFailureClass::Unsupported)
    }

    fn resolve_feature_owned_point(
        &self,
        _feature: &str,
        _role: SpatialCarrierPointRole,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        Err(SpatialWitnessFailureClass::Unsupported)
    }
}

#[cfg(test)]
#[path = "witness_catalog_tests.rs"]
mod witness_catalog_tests;

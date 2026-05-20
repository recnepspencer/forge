use crate::spatial_intent::refs::{
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
};
use crate::spatial_intent::resolution::{
    SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialCatalogResolvedDirectionWitness {
    world_direction: [f64; 3],
    resolution_class: SpatialCatalogWitnessResolutionClass,
}

impl SpatialCatalogResolvedDirectionWitness {
    pub fn new(
        world_direction: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
    ) -> Self {
        Self {
            world_direction,
            resolution_class,
        }
    }

    pub fn world_direction(&self) -> [f64; 3] {
        self.world_direction
    }

    pub fn resolution_class(&self) -> SpatialCatalogWitnessResolutionClass {
        self.resolution_class
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialCatalogResolvedPointWitness {
    world_point: [f64; 3],
    resolution_class: SpatialCatalogWitnessResolutionClass,
}

impl SpatialCatalogResolvedPointWitness {
    pub fn new(
        world_point: [f64; 3],
        resolution_class: SpatialCatalogWitnessResolutionClass,
    ) -> Self {
        Self {
            world_point,
            resolution_class,
        }
    }

    pub fn world_point(&self) -> [f64; 3] {
        self.world_point
    }

    pub fn resolution_class(&self) -> SpatialCatalogWitnessResolutionClass {
        self.resolution_class
    }
}

pub trait SpatialWitnessCatalog {
    fn resolve_parameter_space_direction(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: [f64; 2],
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
        parameter: [f64; 2],
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
    fn resolve_parameter_space_direction(
        &self,
        _carrier_kind: SpatialCarrierKind,
        _carrier: &str,
        _parameter: [f64; 2],
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
        _parameter: [f64; 2],
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

#[derive(Clone, Debug, Default)]
pub struct SpatialFixtureWitnessCatalog {
    direction_parameter_entries: Vec<DirectionParameterEntry>,
    direction_feature_entries: Vec<DirectionFeatureEntry>,
    point_parameter_entries: Vec<PointParameterEntry>,
    point_feature_entries: Vec<PointFeatureEntry>,
}

impl SpatialFixtureWitnessCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parameter_space_direction(
        mut self,
        carrier_kind: SpatialCarrierKind,
        carrier: impl Into<String>,
        parameter: [f64; 2],
        role: SpatialCarrierDirectionRole,
        outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.direction_parameter_entries
            .push(DirectionParameterEntry {
                carrier_kind,
                carrier: carrier.into(),
                parameter,
                role,
                outcome,
            });
        self
    }

    pub fn with_feature_owned_direction(
        mut self,
        feature: impl Into<String>,
        role: SpatialCarrierDirectionRole,
        outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.direction_feature_entries.push(DirectionFeatureEntry {
            feature: feature.into(),
            role,
            outcome,
        });
        self
    }

    pub fn with_parameter_space_point(
        mut self,
        carrier_kind: SpatialCarrierKind,
        carrier: impl Into<String>,
        parameter: [f64; 2],
        outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.point_parameter_entries.push(PointParameterEntry {
            carrier_kind,
            carrier: carrier.into(),
            parameter,
            outcome,
        });
        self
    }

    pub fn with_feature_owned_point(
        mut self,
        feature: impl Into<String>,
        role: SpatialCarrierPointRole,
        outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.point_feature_entries.push(PointFeatureEntry {
            feature: feature.into(),
            role,
            outcome,
        });
        self
    }
}

impl SpatialWitnessCatalog for SpatialFixtureWitnessCatalog {
    fn resolve_parameter_space_direction(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: [f64; 2],
        role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass> {
        self.direction_parameter_entries
            .iter()
            .find(|entry| {
                entry.carrier_kind == carrier_kind
                    && entry.carrier == carrier
                    && entry.parameter == parameter
                    && entry.role == role
            })
            .map(|entry| entry.outcome)
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }

    fn resolve_feature_owned_direction(
        &self,
        feature: &str,
        role: SpatialCarrierDirectionRole,
    ) -> Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass> {
        self.direction_feature_entries
            .iter()
            .find(|entry| entry.feature == feature && entry.role == role)
            .map(|entry| entry.outcome)
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }

    fn resolve_parameter_space_point(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: [f64; 2],
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        self.point_parameter_entries
            .iter()
            .find(|entry| {
                entry.carrier_kind == carrier_kind
                    && entry.carrier == carrier
                    && entry.parameter == parameter
            })
            .map(|entry| entry.outcome)
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }

    fn resolve_feature_owned_point(
        &self,
        feature: &str,
        role: SpatialCarrierPointRole,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        self.point_feature_entries
            .iter()
            .find(|entry| entry.feature == feature && entry.role == role)
            .map(|entry| entry.outcome)
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }
}

#[derive(Clone, Debug)]
struct DirectionParameterEntry {
    carrier_kind: SpatialCarrierKind,
    carrier: String,
    parameter: [f64; 2],
    role: SpatialCarrierDirectionRole,
    outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
}

#[derive(Clone, Debug)]
struct DirectionFeatureEntry {
    feature: String,
    role: SpatialCarrierDirectionRole,
    outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
}

#[derive(Clone, Debug)]
struct PointParameterEntry {
    carrier_kind: SpatialCarrierKind,
    carrier: String,
    parameter: [f64; 2],
    outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
}

#[derive(Clone, Debug)]
struct PointFeatureEntry {
    feature: String,
    role: SpatialCarrierPointRole,
    outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
}

#[cfg(test)]
#[path = "witness_catalog_tests.rs"]
mod witness_catalog_tests;

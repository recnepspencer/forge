use crate::facade::{
    anchor_selection::SpatialWitnessFailureClass,
    refs::{SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole},
    refs::{
        SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedGeometricTag,
        SpatialCatalogResolvedPointWitness, SpatialWitnessCatalog,
    },
};
use worth_geom::ParameterSpacePoint;

#[derive(Clone, Debug, Default)]
pub struct SpatialFixtureWitnessCatalog {
    tag_entries: Vec<GeometricTagEntry>,
    direction_parameter_entries: Vec<DirectionParameterEntry>,
    direction_feature_entries: Vec<DirectionFeatureEntry>,
    point_parameter_entries: Vec<PointParameterEntry>,
    point_feature_entries: Vec<PointFeatureEntry>,
}

impl SpatialFixtureWitnessCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_geometric_tag_point(
        mut self,
        tag: impl Into<String>,
        outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.tag_entries.push(GeometricTagEntry {
            tag: tag.into(),
            outcome: outcome.map(SpatialCatalogResolvedGeometricTag::PointLike),
        });
        self
    }

    pub fn with_geometric_tag_direction(
        mut self,
        tag: impl Into<String>,
        outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
    ) -> Self {
        self.tag_entries.push(GeometricTagEntry {
            tag: tag.into(),
            outcome: outcome.map(SpatialCatalogResolvedGeometricTag::DirectionLike),
        });
        self
    }

    pub fn with_geometric_tag_unsupported_class(mut self, tag: impl Into<String>) -> Self {
        self.tag_entries.push(GeometricTagEntry {
            tag: tag.into(),
            outcome: Ok(SpatialCatalogResolvedGeometricTag::UnsupportedClass),
        });
        self
    }

    pub fn with_parameter_space_direction(
        mut self,
        carrier_kind: SpatialCarrierKind,
        carrier: impl Into<String>,
        parameter: ParameterSpacePoint,
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
        parameter: ParameterSpacePoint,
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
    fn resolve_geometric_tag(
        &self,
        tag: &str,
    ) -> Result<SpatialCatalogResolvedGeometricTag, SpatialWitnessFailureClass> {
        self.tag_entries
            .iter()
            .find(|entry| entry.tag == tag)
            .map(|entry| entry.outcome.clone())
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }

    fn resolve_parameter_space_direction(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: ParameterSpacePoint,
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
            .map(|entry| entry.outcome.clone())
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
            .map(|entry| entry.outcome.clone())
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }

    fn resolve_parameter_space_point(
        &self,
        carrier_kind: SpatialCarrierKind,
        carrier: &str,
        parameter: ParameterSpacePoint,
    ) -> Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass> {
        self.point_parameter_entries
            .iter()
            .find(|entry| {
                entry.carrier_kind == carrier_kind
                    && entry.carrier == carrier
                    && entry.parameter == parameter
            })
            .map(|entry| entry.outcome.clone())
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
            .map(|entry| entry.outcome.clone())
            .unwrap_or(Err(SpatialWitnessFailureClass::Unsupported))
    }
}

#[derive(Clone, Debug)]
struct DirectionParameterEntry {
    carrier_kind: SpatialCarrierKind,
    carrier: String,
    parameter: ParameterSpacePoint,
    role: SpatialCarrierDirectionRole,
    outcome: Result<SpatialCatalogResolvedDirectionWitness, SpatialWitnessFailureClass>,
}

#[derive(Clone, Debug)]
struct GeometricTagEntry {
    tag: String,
    outcome: Result<SpatialCatalogResolvedGeometricTag, SpatialWitnessFailureClass>,
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
    parameter: ParameterSpacePoint,
    outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
}

#[derive(Clone, Debug)]
struct PointFeatureEntry {
    feature: String,
    role: SpatialCarrierPointRole,
    outcome: Result<SpatialCatalogResolvedPointWitness, SpatialWitnessFailureClass>,
}

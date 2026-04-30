use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::data::aspects::{
    WorthAspect, WorthDiagnosticsAspect, WorthGeometryAspect, WorthLineageAspect,
    WorthNamingAspect, WorthTopologyAspect,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthQueryCollection {
    TopologyEntity,
    TopologyRelation,
    PersistentName,
    TopologyDiagnostic,
    MaterializedTopology,
    InterpretedTopology,
    TopologyValidation,
    DerivedReadDiagnostics,
    TopologyEquivalenceContract,
}

impl WorthQueryCollection {
    pub const ALL: [Self; 9] = [
        Self::TopologyEntity,
        Self::TopologyRelation,
        Self::PersistentName,
        Self::TopologyDiagnostic,
        Self::MaterializedTopology,
        Self::InterpretedTopology,
        Self::TopologyValidation,
        Self::DerivedReadDiagnostics,
        Self::TopologyEquivalenceContract,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyEntity => "WorthTopologyEntity",
            Self::TopologyRelation => "WorthTopologyRelation",
            Self::PersistentName => "WorthPersistentName",
            Self::TopologyDiagnostic => "WorthTopologyDiagnostic",
            Self::MaterializedTopology => "WorthMaterializedTopology",
            Self::InterpretedTopology => "WorthInterpretedTopology",
            Self::TopologyValidation => "WorthTopologyValidation",
            Self::DerivedReadDiagnostics => "WorthDerivedReadDiagnostics",
            Self::TopologyEquivalenceContract => "WorthTopologyEquivalenceContract",
        }
    }
}

impl From<WorthQueryCollection> for String {
    fn from(value: WorthQueryCollection) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthQuerySchemaBasis {
    AuthoritativeTopologyTruth,
    TopologyEntityLiveView,
    TopologyRelationLiveView,
    PersistentNameLiveView,
    TopologyDiagnosticLiveView,
    MaterializedTopologyComputed,
    InterpretedTopologyComputed,
    TopologyValidationComputed,
    DerivedReadDiagnosticsComputed,
    TopologyEquivalenceContractComputed,
}

impl WorthQuerySchemaBasis {
    pub const ALL: [Self; 10] = [
        Self::AuthoritativeTopologyTruth,
        Self::TopologyEntityLiveView,
        Self::TopologyRelationLiveView,
        Self::PersistentNameLiveView,
        Self::TopologyDiagnosticLiveView,
        Self::MaterializedTopologyComputed,
        Self::InterpretedTopologyComputed,
        Self::TopologyValidationComputed,
        Self::DerivedReadDiagnosticsComputed,
        Self::TopologyEquivalenceContractComputed,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeTopologyTruth => "worth.schema.authoritative_topology_truth",
            Self::TopologyEntityLiveView => "worth.schema.live.topology_entity",
            Self::TopologyRelationLiveView => "worth.schema.live.topology_relation",
            Self::PersistentNameLiveView => "worth.schema.live.persistent_name",
            Self::TopologyDiagnosticLiveView => "worth.schema.live.topology_diagnostic",
            Self::MaterializedTopologyComputed => "worth.schema.computed.materialized_topology",
            Self::InterpretedTopologyComputed => "worth.schema.computed.interpreted_topology",
            Self::TopologyValidationComputed => "worth.schema.computed.topology_validation",
            Self::DerivedReadDiagnosticsComputed => {
                "worth.schema.computed.derived_read_diagnostics"
            }
            Self::TopologyEquivalenceContractComputed => {
                "worth.schema.computed.topology_equivalence_contract"
            }
        }
    }
}

impl From<WorthQuerySchemaBasis> for String {
    fn from(value: WorthQuerySchemaBasis) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthQueryAspectFamily {
    Topology,
    Geometry,
    Lineage,
    Naming,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorthQueryAspectPath {
    family: WorthQueryAspectFamily,
    path: &'static str,
}

impl WorthQueryAspectPath {
    pub const TOPOLOGY_STRUCTURE: Self =
        Self::new(WorthQueryAspectFamily::Topology, "topology.structure");
    pub const TOPOLOGY_OWNERSHIP: Self =
        Self::new(WorthQueryAspectFamily::Topology, "topology.ownership");
    pub const TOPOLOGY_BOUNDARY: Self =
        Self::new(WorthQueryAspectFamily::Topology, "topology.boundary");
    pub const TOPOLOGY_RADIAL: Self =
        Self::new(WorthQueryAspectFamily::Topology, "topology.radial");

    pub const GEOMETRY_BINDING: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.binding");
    pub const GEOMETRY_EMBEDDING: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.embedding");
    pub const GEOMETRY_PROVENANCE: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.provenance");
    pub const GEOMETRY_APPROXIMATION: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.approximation");
    pub const GEOMETRY_UV_ANCHORING: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.uv_anchoring");
    pub const GEOMETRY_CARRIER: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.carrier");
    pub const GEOMETRY_PRECISION: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.precision");
    pub const GEOMETRY_FALLBACK: Self =
        Self::new(WorthQueryAspectFamily::Geometry, "geometry.fallback");

    pub const LINEAGE_PROVENANCE: Self =
        Self::new(WorthQueryAspectFamily::Lineage, "lineage.provenance");
    pub const NAMING_PERSISTENT_NAME: Self =
        Self::new(WorthQueryAspectFamily::Naming, "naming.persistent_name");
    pub const DIAGNOSTICS_DECISIONS: Self =
        Self::new(WorthQueryAspectFamily::Diagnostics, "diagnostics.decisions");
    pub const DIAGNOSTICS_INTERPRETATIONS: Self = Self::new(
        WorthQueryAspectFamily::Diagnostics,
        "diagnostics.interpretations",
    );

    pub const ALL: [Self; 16] = [
        Self::TOPOLOGY_STRUCTURE,
        Self::TOPOLOGY_OWNERSHIP,
        Self::TOPOLOGY_BOUNDARY,
        Self::TOPOLOGY_RADIAL,
        Self::GEOMETRY_BINDING,
        Self::GEOMETRY_EMBEDDING,
        Self::GEOMETRY_PROVENANCE,
        Self::GEOMETRY_APPROXIMATION,
        Self::GEOMETRY_UV_ANCHORING,
        Self::GEOMETRY_CARRIER,
        Self::GEOMETRY_PRECISION,
        Self::GEOMETRY_FALLBACK,
        Self::LINEAGE_PROVENANCE,
        Self::NAMING_PERSISTENT_NAME,
        Self::DIAGNOSTICS_DECISIONS,
        Self::DIAGNOSTICS_INTERPRETATIONS,
    ];

    const fn new(family: WorthQueryAspectFamily, path: &'static str) -> Self {
        Self { family, path }
    }

    pub const fn family(self) -> WorthQueryAspectFamily {
        self.family
    }

    pub const fn as_str(self) -> &'static str {
        self.path
    }

    pub fn section(self) -> &'static str {
        self.path
            .split_once('.')
            .map(|(section, _)| section)
            .expect("worth query aspect paths are static aspect.field values")
    }

    pub fn field(self) -> &'static str {
        self.path
            .split_once('.')
            .map(|(_, field)| field)
            .expect("worth query aspect paths are static aspect.field values")
    }

    pub const fn from_worth_aspect(aspect: WorthAspect) -> Self {
        match aspect {
            WorthAspect::Topology(WorthTopologyAspect::Structure) => Self::TOPOLOGY_STRUCTURE,
            WorthAspect::Topology(WorthTopologyAspect::Ownership) => Self::TOPOLOGY_OWNERSHIP,
            WorthAspect::Topology(WorthTopologyAspect::Boundary) => Self::TOPOLOGY_BOUNDARY,
            WorthAspect::Topology(WorthTopologyAspect::Radial) => Self::TOPOLOGY_RADIAL,
            WorthAspect::Geometry(WorthGeometryAspect::Binding) => Self::GEOMETRY_BINDING,
            WorthAspect::Geometry(WorthGeometryAspect::Embedding) => Self::GEOMETRY_EMBEDDING,
            WorthAspect::Geometry(WorthGeometryAspect::Provenance) => Self::GEOMETRY_PROVENANCE,
            WorthAspect::Geometry(WorthGeometryAspect::Approximation) => {
                Self::GEOMETRY_APPROXIMATION
            }
            WorthAspect::Geometry(WorthGeometryAspect::UvAnchoring) => Self::GEOMETRY_UV_ANCHORING,
            WorthAspect::Geometry(WorthGeometryAspect::Carrier) => Self::GEOMETRY_CARRIER,
            WorthAspect::Geometry(WorthGeometryAspect::Precision) => Self::GEOMETRY_PRECISION,
            WorthAspect::Geometry(WorthGeometryAspect::Fallback) => Self::GEOMETRY_FALLBACK,
            WorthAspect::Lineage(WorthLineageAspect::Provenance) => Self::LINEAGE_PROVENANCE,
            WorthAspect::Naming(WorthNamingAspect::PersistentName) => Self::NAMING_PERSISTENT_NAME,
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions) => {
                Self::DIAGNOSTICS_DECISIONS
            }
            WorthAspect::Diagnostics(WorthDiagnosticsAspect::Interpretations) => {
                Self::DIAGNOSTICS_INTERPRETATIONS
            }
        }
    }
}

impl From<WorthQueryAspectPath> for String {
    fn from(value: WorthQueryAspectPath) -> Self {
        value.as_str().to_string()
    }
}

pub fn worth_query_aspect_paths(
    aspects: impl IntoIterator<Item = WorthAspect>,
) -> Vec<WorthQueryAspectPath> {
    aspects
        .into_iter()
        .map(WorthQueryAspectPath::from_worth_aspect)
        .collect()
}

pub fn worth_query_aspect_path_strings(
    aspects: impl IntoIterator<Item = WorthAspect>,
) -> Vec<String> {
    worth_query_aspect_paths(aspects)
        .into_iter()
        .map(String::from)
        .collect()
}

pub fn worth_query_aspect_paths_from_set(
    aspects: &BTreeSet<WorthAspect>,
) -> Vec<WorthQueryAspectPath> {
    worth_query_aspect_paths(aspects.iter().copied())
}

#[cfg(test)]
mod tests;

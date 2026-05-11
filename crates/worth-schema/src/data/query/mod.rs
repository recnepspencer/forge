use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};

mod declarations;
mod mutation_admission;

pub use declarations::{
    QueryComputedDeclarationBuilder, QueryDeclarationError, QueryLiveDeclarationBuilder,
};
pub use mutation_admission::{
    admit_query_mutation_batch, query_mutation_support_contract, QueryMutationAdmission,
    QueryMutationAdmissionBlocker, QueryMutationAdmissionReport, QueryMutationSupportContract,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QueryLiveField {
    Aspect(QueryAspectPath),
    IdentityId,
    LineageProvenance,
    TopologyKind,
    TopologySourceIdentity,
    TopologyTargetIdentity,
    NamingTargetIdentity,
}

impl QueryLiveField {
    pub fn aspect(self) -> &'static str {
        match self {
            Self::Aspect(path) => path.section(),
            Self::IdentityId => "identity",
            Self::LineageProvenance => "lineage",
            Self::TopologyKind | Self::TopologySourceIdentity | Self::TopologyTargetIdentity => {
                "topology"
            }
            Self::NamingTargetIdentity => "naming",
        }
    }

    pub fn field(self) -> &'static str {
        match self {
            Self::Aspect(path) => path.field(),
            Self::IdentityId => "id",
            Self::LineageProvenance => "provenance",
            Self::TopologyKind => "kind",
            Self::TopologySourceIdentity => "source_identity",
            Self::TopologyTargetIdentity => "target_identity",
            Self::NamingTargetIdentity => "target_identity",
        }
    }

    pub fn delivered_name(self) -> &'static str {
        match self {
            Self::Aspect(path) => path.as_str(),
            Self::IdentityId => "identity.id",
            Self::LineageProvenance => "lineage.provenance",
            Self::TopologyKind => "topology.kind",
            Self::TopologySourceIdentity => "topology.source_identity",
            Self::TopologyTargetIdentity => "topology.target_identity",
            Self::NamingTargetIdentity => "naming.target_identity",
        }
    }
}

impl From<QueryAspectPath> for QueryLiveField {
    fn from(value: QueryAspectPath) -> Self {
        Self::Aspect(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum QueryCollection {
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

impl QueryCollection {
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
            Self::TopologyEntity => "TopologyEntity",
            Self::TopologyRelation => "TopologyRelation",
            Self::PersistentName => "PersistentName",
            Self::TopologyDiagnostic => "TopologyDiagnostic",
            Self::MaterializedTopology => "MaterializedTopology",
            Self::InterpretedTopology => "InterpretedTopology",
            Self::TopologyValidation => "TopologyValidation",
            Self::DerivedReadDiagnostics => "DerivedReadDiagnostics",
            Self::TopologyEquivalenceContract => "TopologyEquivalenceContract",
        }
    }
}

impl From<QueryCollection> for String {
    fn from(value: QueryCollection) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum QuerySchemaBasis {
    AuthoritativeTopologyTruth,
    TopologyEntityLiveView,
    TopologyDomainQuery,
    TopologyRelationLiveView,
    PersistentNameLiveView,
    TopologyDiagnosticLiveView,
    MaterializedTopologyComputed,
    InterpretedTopologyComputed,
    TopologyValidationComputed,
    DerivedReadDiagnosticsComputed,
    TopologyEquivalenceContractComputed,
}

impl QuerySchemaBasis {
    pub const ALL: [Self; 11] = [
        Self::AuthoritativeTopologyTruth,
        Self::TopologyEntityLiveView,
        Self::TopologyDomainQuery,
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
            Self::AuthoritativeTopologyTruth => ".schema.authoritative_topology_truth",
            Self::TopologyEntityLiveView => ".schema.live.topology_entity",
            Self::TopologyDomainQuery => ".schema.domain.topology_query",
            Self::TopologyRelationLiveView => ".schema.live.topology_relation",
            Self::PersistentNameLiveView => ".schema.live.persistent_name",
            Self::TopologyDiagnosticLiveView => ".schema.live.topology_diagnostic",
            Self::MaterializedTopologyComputed => ".schema.computed.materialized_topology",
            Self::InterpretedTopologyComputed => ".schema.computed.interpreted_topology",
            Self::TopologyValidationComputed => ".schema.computed.topology_validation",
            Self::DerivedReadDiagnosticsComputed => ".schema.computed.derived_read_diagnostics",
            Self::TopologyEquivalenceContractComputed => {
                ".schema.computed.topology_equivalence_contract"
            }
        }
    }
}

impl From<QuerySchemaBasis> for String {
    fn from(value: QuerySchemaBasis) -> Self {
        value.as_str().to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum QueryAspectFamily {
    Topology,
    Geometry,
    Lineage,
    Naming,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct QueryAspectPath {
    family: QueryAspectFamily,
    path: &'static str,
}

impl QueryAspectPath {
    pub const TOPOLOGY_STRUCTURE: Self =
        Self::new(QueryAspectFamily::Topology, "topology.structure");
    pub const TOPOLOGY_OWNERSHIP: Self =
        Self::new(QueryAspectFamily::Topology, "topology.ownership");
    pub const TOPOLOGY_BOUNDARY: Self = Self::new(QueryAspectFamily::Topology, "topology.boundary");
    pub const TOPOLOGY_RADIAL: Self = Self::new(QueryAspectFamily::Topology, "topology.radial");

    pub const GEOMETRY_BINDING: Self = Self::new(QueryAspectFamily::Geometry, "geometry.binding");
    pub const GEOMETRY_EMBEDDING: Self =
        Self::new(QueryAspectFamily::Geometry, "geometry.embedding");
    pub const GEOMETRY_PROVENANCE: Self =
        Self::new(QueryAspectFamily::Geometry, "geometry.provenance");
    pub const GEOMETRY_APPROXIMATION: Self =
        Self::new(QueryAspectFamily::Geometry, "geometry.approximation");
    pub const GEOMETRY_UV_ANCHORING: Self =
        Self::new(QueryAspectFamily::Geometry, "geometry.uv_anchoring");
    pub const GEOMETRY_CARRIER: Self = Self::new(QueryAspectFamily::Geometry, "geometry.carrier");
    pub const GEOMETRY_PRECISION: Self =
        Self::new(QueryAspectFamily::Geometry, "geometry.precision");
    pub const GEOMETRY_FALLBACK: Self = Self::new(QueryAspectFamily::Geometry, "geometry.fallback");

    pub const LINEAGE_PROVENANCE: Self =
        Self::new(QueryAspectFamily::Lineage, "lineage.provenance");
    pub const NAMING_PERSISTENT_NAME: Self =
        Self::new(QueryAspectFamily::Naming, "naming.persistent_name");
    pub const DIAGNOSTICS_DECISIONS: Self =
        Self::new(QueryAspectFamily::Diagnostics, "diagnostics.decisions");
    pub const DIAGNOSTICS_INTERPRETATIONS: Self = Self::new(
        QueryAspectFamily::Diagnostics,
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

    const fn new(family: QueryAspectFamily, path: &'static str) -> Self {
        Self { family, path }
    }

    pub const fn family(self) -> QueryAspectFamily {
        self.family
    }

    pub const fn as_str(self) -> &'static str {
        self.path
    }

    pub fn from_str(path: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.as_str() == path)
    }

    pub fn section(self) -> &'static str {
        self.path
            .split_once('.')
            .map(|(section, _)| section)
            .expect(" query aspect paths are static aspect.field values")
    }

    pub fn field(self) -> &'static str {
        self.path
            .split_once('.')
            .map(|(_, field)| field)
            .expect(" query aspect paths are static aspect.field values")
    }

    pub const fn from_aspect(aspect: Aspect) -> Self {
        match aspect {
            Aspect::Topology(TopologyAspect::Structure) => Self::TOPOLOGY_STRUCTURE,
            Aspect::Topology(TopologyAspect::Ownership) => Self::TOPOLOGY_OWNERSHIP,
            Aspect::Topology(TopologyAspect::Boundary) => Self::TOPOLOGY_BOUNDARY,
            Aspect::Topology(TopologyAspect::Radial) => Self::TOPOLOGY_RADIAL,
            Aspect::Geometry(GeometryAspect::Binding) => Self::GEOMETRY_BINDING,
            Aspect::Geometry(GeometryAspect::Embedding) => Self::GEOMETRY_EMBEDDING,
            Aspect::Geometry(GeometryAspect::Provenance) => Self::GEOMETRY_PROVENANCE,
            Aspect::Geometry(GeometryAspect::Approximation) => Self::GEOMETRY_APPROXIMATION,
            Aspect::Geometry(GeometryAspect::UvAnchoring) => Self::GEOMETRY_UV_ANCHORING,
            Aspect::Geometry(GeometryAspect::Carrier) => Self::GEOMETRY_CARRIER,
            Aspect::Geometry(GeometryAspect::Precision) => Self::GEOMETRY_PRECISION,
            Aspect::Geometry(GeometryAspect::Fallback) => Self::GEOMETRY_FALLBACK,
            Aspect::Lineage(LineageAspect::Provenance) => Self::LINEAGE_PROVENANCE,
            Aspect::Naming(NamingAspect::PersistentName) => Self::NAMING_PERSISTENT_NAME,
            Aspect::Diagnostics(DiagnosticsAspect::Decisions) => Self::DIAGNOSTICS_DECISIONS,
            Aspect::Diagnostics(DiagnosticsAspect::Interpretations) => {
                Self::DIAGNOSTICS_INTERPRETATIONS
            }
        }
    }

    pub fn into_aspect(self) -> Aspect {
        match self {
            Self::TOPOLOGY_STRUCTURE => Aspect::Topology(TopologyAspect::Structure),
            Self::TOPOLOGY_OWNERSHIP => Aspect::Topology(TopologyAspect::Ownership),
            Self::TOPOLOGY_BOUNDARY => Aspect::Topology(TopologyAspect::Boundary),
            Self::TOPOLOGY_RADIAL => Aspect::Topology(TopologyAspect::Radial),
            Self::GEOMETRY_BINDING => Aspect::Geometry(GeometryAspect::Binding),
            Self::GEOMETRY_EMBEDDING => Aspect::Geometry(GeometryAspect::Embedding),
            Self::GEOMETRY_PROVENANCE => Aspect::Geometry(GeometryAspect::Provenance),
            Self::GEOMETRY_APPROXIMATION => Aspect::Geometry(GeometryAspect::Approximation),
            Self::GEOMETRY_UV_ANCHORING => Aspect::Geometry(GeometryAspect::UvAnchoring),
            Self::GEOMETRY_CARRIER => Aspect::Geometry(GeometryAspect::Carrier),
            Self::GEOMETRY_PRECISION => Aspect::Geometry(GeometryAspect::Precision),
            Self::GEOMETRY_FALLBACK => Aspect::Geometry(GeometryAspect::Fallback),
            Self::LINEAGE_PROVENANCE => Aspect::Lineage(LineageAspect::Provenance),
            Self::NAMING_PERSISTENT_NAME => Aspect::Naming(NamingAspect::PersistentName),
            Self::DIAGNOSTICS_DECISIONS => Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            Self::DIAGNOSTICS_INTERPRETATIONS => {
                Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
            }
            _ => unreachable!(" query aspect paths must be one of the declared constants"),
        }
    }
}

impl From<QueryAspectPath> for String {
    fn from(value: QueryAspectPath) -> Self {
        value.as_str().to_string()
    }
}

pub fn query_aspect_paths(aspects: impl IntoIterator<Item = Aspect>) -> Vec<QueryAspectPath> {
    aspects
        .into_iter()
        .map(QueryAspectPath::from_aspect)
        .collect()
}

pub fn query_aspect_path_strings(aspects: impl IntoIterator<Item = Aspect>) -> Vec<String> {
    query_aspect_paths(aspects)
        .into_iter()
        .map(String::from)
        .collect()
}

pub fn query_aspect_paths_from_set(aspects: &BTreeSet<Aspect>) -> Vec<QueryAspectPath> {
    query_aspect_paths(aspects.iter().copied())
}

#[cfg(test)]
mod mutation_admission_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod traversal_tests;

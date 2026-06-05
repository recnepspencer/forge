use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

use crate::bindings::identity::{vertex_geometry_basis, SpatialBindingIdentity};

use super::{
    evaluate_vertex_geometry_completeness, SpatialBindingAuthorityError,
    SpatialBindingCompleteness, SpatialBindingIllegalityReason, SpatialBindingKind,
    SpatialBindingUnsupportedReason,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VertexGeometryProvenanceKind {
    CanonicalWitness,
    RealizedVertex,
}

impl VertexGeometryProvenanceKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CanonicalWitness => "canonical_witness",
            Self::RealizedVertex => "realized_vertex",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VertexToleranceRegime {
    ExactBits,
    AdmittedTolerance,
}

impl VertexToleranceRegime {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ExactBits => "exact_bits",
            Self::AdmittedTolerance => "admitted_tolerance",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexBindingSite {
    topology_vertex_identity: String,
    persistent_name: Option<String>,
}

impl VertexBindingSite {
    pub fn new(topology_vertex_identity: impl Into<String>) -> Self {
        Self {
            topology_vertex_identity: topology_vertex_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_vertex_identity(&self) -> &str {
        &self.topology_vertex_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VertexGeometryBindingSpec {
    site: VertexBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
    provenance: VertexGeometryProvenanceKind,
    tolerance_regime: VertexToleranceRegime,
}

impl VertexGeometryBindingSpec {
    pub fn new(
        site: VertexBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
        provenance: VertexGeometryProvenanceKind,
        tolerance_regime: VertexToleranceRegime,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
            provenance,
            tolerance_regime,
        }
    }

    pub fn site(&self) -> &VertexBindingSite {
        &self.site
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        &self.geometry_identity
    }

    pub fn provenance(&self) -> VertexGeometryProvenanceKind {
        self.provenance
    }

    pub fn tolerance_regime(&self) -> VertexToleranceRegime {
        self.tolerance_regime
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedVertexGeometryBinding {
    spec: VertexGeometryBindingSpec,
    identity: SpatialBindingIdentity,
    completeness: SpatialBindingCompleteness,
}

impl AdmittedVertexGeometryBinding {
    pub(crate) fn admit(
        spec: VertexGeometryBindingSpec,
    ) -> Result<Self, SpatialBindingAuthorityError> {
        if spec.site().topology_vertex_identity().is_empty() {
            return Err(SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::VertexGeometry,
                ),
            ));
        }
        if spec.birth_contract().topology_contract().vertex_count() == 0 {
            return Err(SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::VertexGeometry,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ));
        }

        let completeness = evaluate_vertex_geometry_completeness(spec.geometry_identity());
        let identity = SpatialBindingIdentity::from_basis(vertex_geometry_basis(
            spec.site().topology_vertex_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
            spec.provenance().as_str(),
            spec.tolerance_regime().as_str(),
        ));

        Ok(Self {
            spec,
            identity,
            completeness,
        })
    }

    pub fn kind(&self) -> SpatialBindingKind {
        SpatialBindingKind::VertexGeometry
    }

    pub fn site(&self) -> &VertexBindingSite {
        self.spec.site()
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.spec.birth_contract()
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        self.spec.geometry_identity()
    }

    pub fn provenance(&self) -> VertexGeometryProvenanceKind {
        self.spec.provenance()
    }

    pub fn tolerance_regime(&self) -> VertexToleranceRegime {
        self.spec.tolerance_regime()
    }

    pub fn identity(&self) -> &SpatialBindingIdentity {
        &self.identity
    }

    pub fn completeness(&self) -> &SpatialBindingCompleteness {
        &self.completeness
    }
}

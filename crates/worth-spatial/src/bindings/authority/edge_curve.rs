use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

use crate::bindings::identity::{edge_curve_basis, SpatialBindingIdentity};

use super::{
    SpatialBindingAuthorityError, SpatialBindingCompleteness, SpatialBindingIncompleteness,
    SpatialBindingKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeBindingSite {
    topology_edge_identity: String,
    persistent_name: Option<String>,
}

impl EdgeBindingSite {
    pub fn new(topology_edge_identity: impl Into<String>) -> Self {
        Self {
            topology_edge_identity: topology_edge_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_edge_identity(&self) -> &str {
        &self.topology_edge_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeCurveBindingSpec {
    site: EdgeBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
}

impl EdgeCurveBindingSpec {
    pub fn new(
        site: EdgeBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
        }
    }

    pub fn site(&self) -> &EdgeBindingSite {
        &self.site
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        &self.geometry_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedEdgeCurveBinding {
    spec: EdgeCurveBindingSpec,
    identity: SpatialBindingIdentity,
    completeness: SpatialBindingCompleteness,
}

impl AdmittedEdgeCurveBinding {
    pub(crate) fn admit(spec: EdgeCurveBindingSpec) -> Result<Self, SpatialBindingAuthorityError> {
        if spec.site().topology_edge_identity().is_empty() {
            return Err(SpatialBindingAuthorityError::MissingTopologyIdentity(
                SpatialBindingKind::EdgeCurve,
            ));
        }
        if spec.birth_contract().topology_contract().edge_count() == 0 {
            return Err(
                SpatialBindingAuthorityError::UnsupportedTopologyBirthClass {
                    binding_kind: SpatialBindingKind::EdgeCurve,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            );
        }

        let completeness = if spec.geometry_identity().vertices().len() < 2 {
            SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::CurveWitnessRequiresAtLeastTwoVertices,
            )
        } else {
            SpatialBindingCompleteness::Complete
        };
        let identity = SpatialBindingIdentity::from_basis(edge_curve_basis(
            spec.site().topology_edge_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
        ));

        Ok(Self {
            spec,
            identity,
            completeness,
        })
    }

    pub fn kind(&self) -> SpatialBindingKind {
        SpatialBindingKind::EdgeCurve
    }

    pub fn site(&self) -> &EdgeBindingSite {
        self.spec.site()
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.spec.birth_contract()
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        self.spec.geometry_identity()
    }

    pub fn identity(&self) -> &SpatialBindingIdentity {
        &self.identity
    }

    pub fn completeness(&self) -> &SpatialBindingCompleteness {
        &self.completeness
    }
}

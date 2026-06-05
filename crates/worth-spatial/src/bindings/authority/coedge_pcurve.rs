use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

use crate::bindings::identity::{coedge_pcurve_basis, SpatialBindingIdentity};

use super::{
    SpatialBindingAuthorityError, SpatialBindingCompleteness, SpatialBindingIncompleteness,
    SpatialBindingKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoedgeBindingSite {
    topology_coedge_identity: String,
    persistent_name: Option<String>,
}

impl CoedgeBindingSite {
    pub fn new(topology_coedge_identity: impl Into<String>) -> Self {
        Self {
            topology_coedge_identity: topology_coedge_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_coedge_identity(&self) -> &str {
        &self.topology_coedge_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoedgePCurveBindingSpec {
    site: CoedgeBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
}

impl CoedgePCurveBindingSpec {
    pub fn new(
        site: CoedgeBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
        }
    }

    pub fn site(&self) -> &CoedgeBindingSite {
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
pub struct AdmittedCoedgePCurveBinding {
    spec: CoedgePCurveBindingSpec,
    identity: SpatialBindingIdentity,
    completeness: SpatialBindingCompleteness,
}

impl AdmittedCoedgePCurveBinding {
    pub(crate) fn admit(
        spec: CoedgePCurveBindingSpec,
    ) -> Result<Self, SpatialBindingAuthorityError> {
        if spec.site().topology_coedge_identity().is_empty() {
            return Err(SpatialBindingAuthorityError::MissingTopologyIdentity(
                SpatialBindingKind::CoedgePCurve,
            ));
        }
        if spec.birth_contract().topology_contract().loop_count() == 0 {
            return Err(
                SpatialBindingAuthorityError::UnsupportedTopologyBirthClass {
                    binding_kind: SpatialBindingKind::CoedgePCurve,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            );
        }

        let completeness = if spec.geometry_identity().support_planes().is_empty() {
            SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::PCurveWitnessRequiresPlanarSupport,
            )
        } else if spec.geometry_identity().vertices().len() < 2 {
            SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::CurveWitnessRequiresAtLeastTwoVertices,
            )
        } else {
            SpatialBindingCompleteness::Complete
        };
        let identity = SpatialBindingIdentity::from_basis(coedge_pcurve_basis(
            spec.site().topology_coedge_identity(),
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
        SpatialBindingKind::CoedgePCurve
    }

    pub fn site(&self) -> &CoedgeBindingSite {
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

use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

use crate::bindings::identity::{face_surface_basis, SpatialBindingIdentity};

use super::{
    evaluate_face_surface_completeness, SpatialBindingAuthorityError, SpatialBindingCompleteness,
    SpatialBindingIllegalityReason, SpatialBindingKind, SpatialBindingUnsupportedReason,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FaceBindingSite {
    topology_face_identity: String,
    persistent_name: Option<String>,
}

impl FaceBindingSite {
    pub fn new(topology_face_identity: impl Into<String>) -> Self {
        Self {
            topology_face_identity: topology_face_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_face_identity(&self) -> &str {
        &self.topology_face_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FaceSurfaceBindingSpec {
    site: FaceBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
}

impl FaceSurfaceBindingSpec {
    pub fn new(
        site: FaceBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
        }
    }

    pub fn site(&self) -> &FaceBindingSite {
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
pub struct AdmittedFaceSurfaceBinding {
    spec: FaceSurfaceBindingSpec,
    identity: SpatialBindingIdentity,
    completeness: SpatialBindingCompleteness,
}

impl AdmittedFaceSurfaceBinding {
    pub(crate) fn admit(
        spec: FaceSurfaceBindingSpec,
    ) -> Result<Self, SpatialBindingAuthorityError> {
        if spec.site().topology_face_identity().is_empty() {
            return Err(SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ));
        }
        if spec.birth_contract().topology_contract().face_count() == 0 {
            return Err(SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::FaceSurface,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ));
        }

        let completeness = evaluate_face_surface_completeness(spec.geometry_identity());
        let identity = SpatialBindingIdentity::from_basis(face_surface_basis(
            spec.site().topology_face_identity(),
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
        SpatialBindingKind::FaceSurface
    }

    pub fn site(&self) -> &FaceBindingSite {
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

use crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding;

use super::SpatialRebindingAuthorityError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum NeighborhoodBindingFamily {
    FaceSurface,
    FaceSurfacePointAnchor,
    FaceSurfaceDirectionAnchor,
    EdgeCurve,
    EdgeCurvePointAnchor,
    EdgeCurveDirectionAnchor,
    CoedgePCurve,
    CoedgePCurvePointAnchor,
    CoedgePCurveDirectionAnchor,
    VertexGeometry,
}

impl NeighborhoodBindingFamily {
    pub fn from_binding(
        binding: &SpatialAdmittedPrimitiveBinding,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        match binding {
            SpatialAdmittedPrimitiveBinding::FaceSurface(_) => Ok(Self::FaceSurface),
            SpatialAdmittedPrimitiveBinding::EdgeCurve(_) => Ok(Self::EdgeCurve),
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(_) => Ok(Self::CoedgePCurve),
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(_) => {
                Ok(Self::FaceSurfacePointAnchor)
            }
            SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor(_) => {
                Ok(Self::EdgeCurvePointAnchor)
            }
            SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor(_) => {
                Ok(Self::CoedgePCurvePointAnchor)
            }
            SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor(_) => {
                Ok(Self::FaceSurfaceDirectionAnchor)
            }
            SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor(_) => {
                Ok(Self::EdgeCurveDirectionAnchor)
            }
            SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor(_) => {
                Ok(Self::CoedgePCurveDirectionAnchor)
            }
            SpatialAdmittedPrimitiveBinding::VertexGeometry(_) => Ok(Self::VertexGeometry),
        }
    }

    pub fn rebinding_kind_label(self) -> &'static str {
        match self {
            Self::FaceSurface => "face_surface",
            Self::FaceSurfacePointAnchor => "face_surface_point_anchor",
            Self::FaceSurfaceDirectionAnchor => "face_surface_direction_anchor",
            Self::EdgeCurve => "edge_curve",
            Self::EdgeCurvePointAnchor => "edge_curve_point_anchor",
            Self::EdgeCurveDirectionAnchor => "edge_curve_direction_anchor",
            Self::CoedgePCurve => "coedge_pcurve",
            Self::CoedgePCurvePointAnchor => "coedge_pcurve_point_anchor",
            Self::CoedgePCurveDirectionAnchor => "coedge_pcurve_direction_anchor",
            Self::VertexGeometry => "vertex_geometry",
        }
    }

    pub fn supports_face_surface_rebinding(self) -> bool {
        matches!(
            self,
            Self::FaceSurface | Self::FaceSurfacePointAnchor | Self::FaceSurfaceDirectionAnchor
        )
    }

    pub fn supports_edge_curve_rebinding(self) -> bool {
        matches!(
            self,
            Self::EdgeCurve | Self::EdgeCurvePointAnchor | Self::EdgeCurveDirectionAnchor
        )
    }

    pub fn supports_coedge_pcurve_rebinding(self) -> bool {
        matches!(
            self,
            Self::CoedgePCurve | Self::CoedgePCurvePointAnchor | Self::CoedgePCurveDirectionAnchor
        )
    }

    pub fn supports_vertex_geometry_rebinding(self) -> bool {
        matches!(self, Self::VertexGeometry)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplacementCandidate {
    label: String,
    family: NeighborhoodBindingFamily,
    site_identity: String,
    binding: SpatialAdmittedPrimitiveBinding,
}

impl ReplacementCandidate {
    pub fn new(
        label: impl Into<String>,
        binding: SpatialAdmittedPrimitiveBinding,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        let label = label.into();
        if label.is_empty() {
            return Err(SpatialRebindingAuthorityError::MissingReplacementLabel);
        }
        let family = NeighborhoodBindingFamily::from_binding(&binding)?;
        let site_identity = binding_site_identity(&binding).to_string();
        Ok(Self {
            label,
            family,
            site_identity,
            binding,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn binding(&self) -> &SpatialAdmittedPrimitiveBinding {
        &self.binding
    }

    pub fn family(&self) -> NeighborhoodBindingFamily {
        self.family
    }

    pub fn site_identity(&self) -> &str {
        &self.site_identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplacementCandidateSet {
    candidates: Vec<ReplacementCandidate>,
}

impl ReplacementCandidateSet {
    pub fn new(
        mut candidates: Vec<ReplacementCandidate>,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        if candidates.is_empty() {
            return Err(SpatialRebindingAuthorityError::CandidateSetEmpty);
        }
        candidates.sort_by(|left, right| {
            let left_key = (
                left.binding().identity(),
                left.label(),
                left.site_identity(),
            );
            let right_key = (
                right.binding().identity(),
                right.label(),
                right.site_identity(),
            );
            left_key.cmp(&right_key)
        });
        Ok(Self { candidates })
    }

    pub fn candidates(&self) -> &[ReplacementCandidate] {
        &self.candidates
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalTopologyReplacementNeighborhood {
    family: NeighborhoodBindingFamily,
    prior_site_identity: String,
    candidates: ReplacementCandidateSet,
}

impl LocalTopologyReplacementNeighborhood {
    pub fn new(
        family: NeighborhoodBindingFamily,
        prior_site_identity: impl Into<String>,
        candidates: ReplacementCandidateSet,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        let prior_site_identity = prior_site_identity.into();
        if prior_site_identity.is_empty() {
            return Err(SpatialRebindingAuthorityError::MissingPriorSiteIdentity);
        }
        for candidate in candidates.candidates() {
            if candidate.family() != family {
                return Err(SpatialRebindingAuthorityError::CandidateFamilyMismatch {
                    expected: family,
                    actual: candidate.family(),
                });
            }
        }
        Ok(Self {
            family,
            prior_site_identity,
            candidates,
        })
    }

    pub fn family(&self) -> NeighborhoodBindingFamily {
        self.family
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn candidates(&self) -> &[ReplacementCandidate] {
        self.candidates.candidates()
    }
}

pub(crate) fn binding_site_identity(binding: &SpatialAdmittedPrimitiveBinding) -> &str {
    binding.topology_site_identity()
}

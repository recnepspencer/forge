use worth_geom::facade::ParameterSpacePoint;

use crate::bindings::authority::SpatialAdmittedPrimitiveBinding;

use super::{
    neighborhood::{
        LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidate,
    },
    SpatialRebindingAuthorityError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingContinuityClass {
    Exact,
    AuthoritativeSuccessor,
    CorrespondenceOnly,
    InsufficientEvidence,
    Ambiguous,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingContinuityAssessment {
    continuity_class: BindingContinuityClass,
    candidate_label: Option<String>,
    candidate_identity: Option<String>,
}

impl BindingContinuityAssessment {
    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn candidate_label(&self) -> Option<&str> {
        self.candidate_label.as_deref()
    }

    pub fn candidate_identity(&self) -> Option<&str> {
        self.candidate_identity.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BindingSnapshot {
    family: NeighborhoodBindingFamily,
    site_identity: String,
    birth_class: String,
    geometry_digest: String,
    completeness_complete: bool,
    anchor: Option<AnchorSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AnchorSnapshot {
    carrier_kind: String,
    carrier_identity: String,
    parameter_bits: [u64; 2],
    direction_role: Option<String>,
}

impl BindingSnapshot {
    fn from_binding(
        binding: &SpatialAdmittedPrimitiveBinding,
    ) -> Result<Self, SpatialRebindingAuthorityError> {
        let family = NeighborhoodBindingFamily::from_binding(binding)?;
        Ok(match binding {
            SpatialAdmittedPrimitiveBinding::FaceSurface(binding) => Self {
                family,
                site_identity: binding.site().topology_face_identity().to_string(),
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurve(binding) => Self {
                family,
                site_identity: binding.site().topology_edge_identity().to_string(),
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurve(binding) => Self {
                family,
                site_identity: binding.site().topology_coedge_identity().to_string(),
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: None,
            },
            SpatialAdmittedPrimitiveBinding::FaceSurfacePointAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_face_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurvePointAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_edge_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurvePointAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_coedge_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::point(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::FaceSurfaceDirectionAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_face_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::EdgeCurveDirectionAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_edge_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::CoedgePCurveDirectionAnchor(binding) => Self {
                family,
                site_identity: binding
                    .binding()
                    .site()
                    .topology_coedge_identity()
                    .to_string(),
                birth_class: binding
                    .binding()
                    .birth_contract()
                    .topology_birth_class()
                    .to_string(),
                geometry_digest: binding
                    .binding()
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness_complete: binding.completeness().is_complete(),
                anchor: Some(AnchorSnapshot::direction(
                    binding.anchor().ownership().carrier_kind().as_str(),
                    binding.anchor().ownership().carrier_identity(),
                    binding.anchor().canonical_parameter().point(),
                    binding.anchor().role_as_str(),
                )),
            },
            SpatialAdmittedPrimitiveBinding::VertexGeometry(_) => {
                return Err(SpatialRebindingAuthorityError::UnsupportedBindingKind(
                    crate::bindings::authority::SpatialBindingKind::VertexGeometry,
                ))
            }
        })
    }
}

impl AnchorSnapshot {
    fn point(carrier_kind: &str, carrier_identity: &str, parameter: ParameterSpacePoint) -> Self {
        Self {
            carrier_kind: carrier_kind.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter_bits: [parameter.u().to_bits(), parameter.v().to_bits()],
            direction_role: None,
        }
    }

    fn direction(
        carrier_kind: &str,
        carrier_identity: &str,
        parameter: ParameterSpacePoint,
        role: &'static str,
    ) -> Self {
        Self {
            carrier_kind: carrier_kind.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter_bits: [parameter.u().to_bits(), parameter.v().to_bits()],
            direction_role: Some(role.to_string()),
        }
    }
}

pub fn evaluate_continuity(
    prior_binding: &SpatialAdmittedPrimitiveBinding,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) -> Result<BindingContinuityAssessment, SpatialRebindingAuthorityError> {
    let prior = BindingSnapshot::from_binding(prior_binding)?;
    let mut best_rank = 0usize;
    let mut best_candidates: Vec<&ReplacementCandidate> = Vec::new();
    for candidate in neighborhood.candidates() {
        let rank = continuity_rank(&prior, &BindingSnapshot::from_binding(candidate.binding())?);
        if rank > best_rank {
            best_rank = rank;
            best_candidates.clear();
            best_candidates.push(candidate);
        } else if rank == best_rank {
            best_candidates.push(candidate);
        }
    }
    let continuity_class = if best_rank == 0 {
        BindingContinuityClass::None
    } else if best_candidates.len() > 1 {
        BindingContinuityClass::Ambiguous
    } else {
        rank_to_class(best_rank)
    };
    let selected = best_candidates.first().copied();
    Ok(BindingContinuityAssessment {
        continuity_class,
        candidate_label: selected.map(|candidate| candidate.label().to_string()),
        candidate_identity: selected
            .map(|candidate| candidate.binding().identity().as_str().to_string()),
    })
}

fn continuity_rank(prior: &BindingSnapshot, candidate: &BindingSnapshot) -> usize {
    if prior.family != candidate.family {
        return 0;
    }
    if !candidate.completeness_complete {
        return 1;
    }
    if prior.geometry_digest == candidate.geometry_digest
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return 5;
    }
    if prior.birth_class == candidate.birth_class
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return 4;
    }
    if prior.birth_class == candidate.birth_class {
        return 3;
    }
    if prior.family == candidate.family {
        return 2;
    }
    0
}

fn same_anchor_semantics(
    prior: Option<&AnchorSnapshot>,
    candidate: Option<&AnchorSnapshot>,
) -> bool {
    match (prior, candidate) {
        (None, None) => true,
        (Some(prior), Some(candidate)) => {
            prior.carrier_kind == candidate.carrier_kind
                && prior.parameter_bits == candidate.parameter_bits
                && prior.direction_role == candidate.direction_role
        }
        _ => false,
    }
}

fn rank_to_class(rank: usize) -> BindingContinuityClass {
    match rank {
        5 => BindingContinuityClass::Exact,
        4 => BindingContinuityClass::AuthoritativeSuccessor,
        3 => BindingContinuityClass::CorrespondenceOnly,
        2 | 1 => BindingContinuityClass::InsufficientEvidence,
        _ => BindingContinuityClass::None,
    }
}

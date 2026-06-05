mod carrier_ownership;
mod parameter_space_direction;
mod parameter_space_point;
mod resolution;

pub use carrier_ownership::{AnchorCarrierKind, AnchorCarrierOwnership};
pub use parameter_space_direction::{
    AdmittedCarrierOwnedDirectionAnchor, AdmittedCoedgePCurveDirectionAnchorBinding,
    AdmittedEdgeCurveDirectionAnchorBinding, AdmittedFaceSurfaceDirectionAnchorBinding,
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
};
pub use parameter_space_point::{
    AdmittedCarrierOwnedPointAnchor, AdmittedCoedgePCurvePointAnchorBinding,
    AdmittedEdgeCurvePointAnchorBinding, AdmittedFaceSurfacePointAnchorBinding,
    CarrierOwnedParameterPointAnchorSpec,
};
pub use resolution::SpatialAnchorAuthorityError;

use crate::bindings::authority::{
    attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face,
    AdmittedCoedgePCurveBinding, AdmittedEdgeCurveBinding, AdmittedFaceSurfaceBinding,
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
};

use self::{
    parameter_space_direction::attach_direction_anchor, parameter_space_point::attach_point_anchor,
};

pub(crate) fn attach_parameter_space_point_to_face_internal(
    binding: AdmittedFaceSurfaceBinding,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedFaceSurfacePointAnchorBinding, SpatialAnchorAuthorityError> {
    attach_point_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::FaceSurface,
        binding.site().topology_face_identity(),
    )
}

pub(crate) fn attach_parameter_space_point_to_edge_internal(
    binding: AdmittedEdgeCurveBinding,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedEdgeCurvePointAnchorBinding, SpatialAnchorAuthorityError> {
    attach_point_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::EdgeCurve,
        binding.site().topology_edge_identity(),
    )
}

pub(crate) fn attach_parameter_space_point_to_coedge_internal(
    binding: AdmittedCoedgePCurveBinding,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedCoedgePCurvePointAnchorBinding, SpatialAnchorAuthorityError> {
    attach_point_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::CoedgePCurve,
        binding.site().topology_coedge_identity(),
    )
}

pub(crate) fn attach_parameter_space_direction_to_face_internal(
    binding: AdmittedFaceSurfaceBinding,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedFaceSurfaceDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    attach_direction_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::FaceSurface,
        binding.site().topology_face_identity(),
    )
}

pub(crate) fn attach_parameter_space_direction_to_edge_internal(
    binding: AdmittedEdgeCurveBinding,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedEdgeCurveDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    attach_direction_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::EdgeCurve,
        binding.site().topology_edge_identity(),
    )
}

pub(crate) fn attach_parameter_space_direction_to_coedge_internal(
    binding: AdmittedCoedgePCurveBinding,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedCoedgePCurveDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    attach_direction_anchor(
        binding.clone(),
        anchor_spec,
        AnchorCarrierKind::CoedgePCurve,
        binding.site().topology_coedge_identity(),
    )
}

pub fn attach_parameter_space_point_to_face(
    binding_spec: FaceSurfaceBindingSpec,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedFaceSurfacePointAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_surface_to_face(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_point_to_face_internal(binding, anchor_spec)
}

pub fn attach_parameter_space_point_to_edge(
    binding_spec: EdgeCurveBindingSpec,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedEdgeCurvePointAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_curve_to_edge(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_point_to_edge_internal(binding, anchor_spec)
}

pub fn attach_parameter_space_point_to_coedge(
    binding_spec: CoedgePCurveBindingSpec,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
) -> Result<AdmittedCoedgePCurvePointAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_pcurve_to_coedge(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_point_to_coedge_internal(binding, anchor_spec)
}

pub fn attach_parameter_space_direction_to_face(
    binding_spec: FaceSurfaceBindingSpec,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedFaceSurfaceDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_surface_to_face(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_direction_to_face_internal(binding, anchor_spec)
}

pub fn attach_parameter_space_direction_to_edge(
    binding_spec: EdgeCurveBindingSpec,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedEdgeCurveDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_curve_to_edge(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_direction_to_edge_internal(binding, anchor_spec)
}

pub fn attach_parameter_space_direction_to_coedge(
    binding_spec: CoedgePCurveBindingSpec,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AdmittedCoedgePCurveDirectionAnchorBinding, SpatialAnchorAuthorityError> {
    let binding = attach_pcurve_to_coedge(binding_spec)
        .map_err(SpatialAnchorAuthorityError::BindingAuthority)?;
    attach_parameter_space_direction_to_coedge_internal(binding, anchor_spec)
}

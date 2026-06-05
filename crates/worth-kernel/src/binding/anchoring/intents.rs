use worth_spatial::facade::bindings::{
    CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveAnchorBindingIntent {
    AttachParameterSpacePointToFace(FaceSurfaceBindingSpec, CarrierOwnedParameterPointAnchorSpec),
    AttachParameterSpacePointToEdge(EdgeCurveBindingSpec, CarrierOwnedParameterPointAnchorSpec),
    AttachParameterSpacePointToCoedge(
        CoedgePCurveBindingSpec,
        CarrierOwnedParameterPointAnchorSpec,
    ),
    AttachParameterSpaceDirectionToFace(
        FaceSurfaceBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
    AttachParameterSpaceDirectionToEdge(
        EdgeCurveBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
    AttachParameterSpaceDirectionToCoedge(
        CoedgePCurveBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
}

impl AuthorPrimitiveAnchorBindingIntent {
    pub fn attach_parameter_space_point_to_face(
        binding_spec: FaceSurfaceBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToFace(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_point_to_edge(
        binding_spec: EdgeCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToEdge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_point_to_coedge(
        binding_spec: CoedgePCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToCoedge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_face(
        binding_spec: FaceSurfaceBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToFace(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_edge(
        binding_spec: EdgeCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToEdge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_coedge(
        binding_spec: CoedgePCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToCoedge(binding_spec, anchor_spec)
    }
}

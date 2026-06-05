use worth_spatial::facade::bindings::{
    CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    VertexGeometryBindingSpec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveBindingIntent {
    AttachSurfaceToFace(FaceSurfaceBindingSpec),
    AttachCurveToEdge(EdgeCurveBindingSpec),
    AttachPCurveToCoedge(CoedgePCurveBindingSpec),
    AttachVertexGeometry(VertexGeometryBindingSpec),
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

impl AuthorPrimitiveBindingIntent {
    pub fn attach_surface_to_face(spec: FaceSurfaceBindingSpec) -> Self {
        Self::AttachSurfaceToFace(spec)
    }

    pub fn attach_curve_to_edge(spec: EdgeCurveBindingSpec) -> Self {
        Self::AttachCurveToEdge(spec)
    }

    pub fn attach_pcurve_to_coedge(spec: CoedgePCurveBindingSpec) -> Self {
        Self::AttachPCurveToCoedge(spec)
    }

    pub fn attach_vertex_geometry(spec: VertexGeometryBindingSpec) -> Self {
        Self::AttachVertexGeometry(spec)
    }

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

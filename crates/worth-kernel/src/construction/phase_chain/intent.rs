use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionRequest};
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
use worth_spatial::facade::SpatialPlacementSpec;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntent {
    request: PrimitiveConstructionRequest,
}

impl PrimitiveConstructionIntent {
    pub fn simplex_solid(spec: SimplexSolidSpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::simplex_solid_spec(spec))
    }

    pub fn orthotope(spec: OrthotopeSpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::orthotope_spec(spec))
    }

    pub fn regular_prism(spec: RegularPrismSpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::regular_prism_spec(spec))
    }

    pub fn regular_pyramid(spec: RegularPyramidSpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::regular_pyramid_spec(spec))
    }

    pub fn wire_body(spec: WireBodySpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::wire_body_spec(spec))
    }

    pub fn shell_with_hole(spec: ShellWithHoleSpec) -> Self {
        Self::from_request(PrimitiveConstructionRequest::shell_with_hole_spec(spec))
    }

    pub fn from_request(request: PrimitiveConstructionRequest) -> Self {
        Self { request }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.request.family()
    }

    pub fn request_digest(&self) -> &str {
        self.request.request_digest()
    }

    pub fn placement_spec(&self) -> SpatialPlacementSpec {
        self.request.placement_spec()
    }

    pub fn at(self, origin: [f64; 3]) -> Self {
        Self::from_request(self.request.with_origin(origin))
    }

    pub fn facing(self, facing: [f64; 3]) -> Self {
        Self::from_request(self.request.with_facing(facing))
    }

    pub(crate) fn with_placement_spec(self, placement: SpatialPlacementSpec) -> Self {
        Self::from_request(self.request.with_placement_spec(placement))
    }

    pub fn request(&self) -> &PrimitiveConstructionRequest {
        &self.request
    }

    pub fn into_request(self) -> PrimitiveConstructionRequest {
        self.request
    }
}

impl From<PrimitiveConstructionRequest> for PrimitiveConstructionIntent {
    fn from(request: PrimitiveConstructionRequest) -> Self {
        Self::from_request(request)
    }
}

impl From<SimplexSolidSpec> for PrimitiveConstructionIntent {
    fn from(spec: SimplexSolidSpec) -> Self {
        Self::simplex_solid(spec)
    }
}

impl From<OrthotopeSpec> for PrimitiveConstructionIntent {
    fn from(spec: OrthotopeSpec) -> Self {
        Self::orthotope(spec)
    }
}

impl From<RegularPrismSpec> for PrimitiveConstructionIntent {
    fn from(spec: RegularPrismSpec) -> Self {
        Self::regular_prism(spec)
    }
}

impl From<RegularPyramidSpec> for PrimitiveConstructionIntent {
    fn from(spec: RegularPyramidSpec) -> Self {
        Self::regular_pyramid(spec)
    }
}

impl From<WireBodySpec> for PrimitiveConstructionIntent {
    fn from(spec: WireBodySpec) -> Self {
        Self::wire_body(spec)
    }
}

impl From<ShellWithHoleSpec> for PrimitiveConstructionIntent {
    fn from(spec: ShellWithHoleSpec) -> Self {
        Self::shell_with_hole(spec)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitScopeClass {
    PlanarBRepLineSegmentEdgeSurgery,
}

impl PlanarBooleanEdgeSplitScopeClass {
    pub fn stable_name(self) -> &'static str {
        match self {
            Self::PlanarBRepLineSegmentEdgeSurgery => "planar-brep-line-segment-edge-surgery",
        }
    }
}

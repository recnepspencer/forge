use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimplexSolidSpec {
    pub scale: f64,
}

impl SimplexSolidSpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::SimplexSolid
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrthotopeSpec {
    pub half_extents: [f64; 3],
}

impl OrthotopeSpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::Orthotope
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularPrismSpec {
    pub sides: u32,
    pub radius: f64,
    pub height: f64,
}

impl RegularPrismSpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::RegularPrism
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularPyramidSpec {
    pub sides: u32,
    pub radius: f64,
    pub height: f64,
}

impl RegularPyramidSpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::RegularPyramid
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireBodySpec {
    pub edge_count: u32,
}

impl WireBodySpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::WireBody
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellWithHoleSpec {
    pub outer_loop_edge_count: u32,
    pub hole_loop_edge_counts: Vec<u32>,
}

impl ShellWithHoleSpec {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        PrimitiveConstructionFamily::ShellWithHole
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimplexSolidSpec {
    pub scale: f64,
    pub auxiliary_altitude_component: f64,
}

impl SimplexSolidSpec {
    pub fn new(scale: f64) -> Self {
        Self {
            scale,
            auxiliary_altitude_component: 0.0,
        }
    }

    pub fn with_auxiliary_altitude_component(mut self, auxiliary_altitude_component: f64) -> Self {
        self.auxiliary_altitude_component = auxiliary_altitude_component;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrthotopeSpec {
    pub half_extents: [f64; 3],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularPrismSpec {
    pub sides: u32,
    pub radius: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegularPyramidSpec {
    pub sides: u32,
    pub radius: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireBodySpec {
    pub edge_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShellWithHoleSpec {
    pub outer_loop_edge_count: u32,
    pub hole_loop_edge_counts: Vec<u32>,
}

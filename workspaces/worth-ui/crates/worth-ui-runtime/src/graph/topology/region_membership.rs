#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphRegionMembership {
    region_name: Box<str>,
}

impl UiGraphRegionMembership {
    pub(in crate::graph::topology) fn new(region_name: Box<str>) -> Self {
        Self { region_name }
    }

    pub fn region_name(&self) -> &str {
        &self.region_name
    }
}

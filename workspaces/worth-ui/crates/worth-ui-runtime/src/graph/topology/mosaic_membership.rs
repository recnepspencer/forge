#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMosaicMembership {
    mosaic_name: Box<str>,
}

impl UiGraphMosaicMembership {
    pub(in crate::graph::topology) fn new(mosaic_name: Box<str>) -> Self {
        Self { mosaic_name }
    }

    pub fn mosaic_name(&self) -> &str {
        &self.mosaic_name
    }
}

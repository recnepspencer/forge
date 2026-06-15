#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutAxis {
    Row,
    Column,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutDimension {
    Width,
    Height,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutSizingValue {
    NamedToken(String),
    Number(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutSizingSpec {
    Fit,
    Fill,
    Fixed(WorthUiLayoutSizingValue),
    Share(u32),
    Ratio {
        numerator: u32,
        denominator: u32,
    },
    Clamp {
        min: WorthUiLayoutSizingValue,
        preferred: Box<WorthUiLayoutSizingSpec>,
        max: WorthUiLayoutSizingValue,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutTopologyNode {
    axis: WorthUiLayoutAxis,
    dimension: Option<WorthUiLayoutDimension>,
    sizing: Option<WorthUiLayoutSizingSpec>,
    scroll_owner: bool,
    resizable: bool,
    restorable: bool,
    children: Vec<WorthUiLayoutTopologyChild>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutTopologyChild {
    Region(WorthUiLayoutTopologyNode),
    Slot(WorthUiLayoutSlotNode),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutSlotNode {
    slot_name: String,
}

impl WorthUiLayoutTopologyNode {
    pub fn new(
        axis: WorthUiLayoutAxis,
        dimension: Option<WorthUiLayoutDimension>,
        sizing: Option<WorthUiLayoutSizingSpec>,
        scroll_owner: bool,
        resizable: bool,
        restorable: bool,
        children: Vec<WorthUiLayoutTopologyChild>,
    ) -> Self {
        Self {
            axis,
            dimension,
            sizing,
            scroll_owner,
            resizable,
            restorable,
            children,
        }
    }

    pub fn axis(&self) -> &WorthUiLayoutAxis {
        &self.axis
    }

    pub fn dimension(&self) -> Option<&WorthUiLayoutDimension> {
        self.dimension.as_ref()
    }

    pub fn sizing(&self) -> Option<&WorthUiLayoutSizingSpec> {
        self.sizing.as_ref()
    }

    pub fn scroll_owner(&self) -> bool {
        self.scroll_owner
    }

    pub fn resizable(&self) -> bool {
        self.resizable
    }

    pub fn restorable(&self) -> bool {
        self.restorable
    }

    pub fn children(&self) -> &[WorthUiLayoutTopologyChild] {
        &self.children
    }
}

impl WorthUiLayoutSlotNode {
    pub fn new(slot_name: impl Into<String>) -> Self {
        Self {
            slot_name: slot_name.into(),
        }
    }

    pub fn slot_name(&self) -> &str {
        &self.slot_name
    }
}

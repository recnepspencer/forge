macro_rules! reference {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name(u16);
        impl $name {
            pub fn new(index: u16) -> Self {
                Self(index)
            }
            pub fn index(self) -> u16 {
                self.0
            }
        }
    };
}

reference!(UiMountedClipReference);
reference!(UiMountedLayerReference);
reference!(UiMountedPaintBatchReference);
reference!(UiMountedSpatialBatchReference);
reference!(UiMountedRealtimeBatchReference);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedTableProjectionStatus {
    Produced,
    Omitted(super::UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedClipProjection {
    Clip(UiMountedClipReference),
    Unclipped,
    Omitted(super::UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedLayerProjection {
    Layer(UiMountedLayerReference),
    Omitted(super::UiMountedOmissionReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPaintPrimitiveKind {
    FilledRect,
    CanvasSpatialBatch,
    RealtimeBatch,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiMountedClipRow {
    bounds: super::UiMountedCanonicalBox,
    parent: Option<UiMountedClipReference>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedLayerRow {
    semantic_order: u32,
    clip: UiMountedClipProjection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPaintBatchRow {
    primitive_count: u32,
    layer: UiMountedLayerProjection,
    resource: Option<super::UiMountedResourceReference>,
    primitive_kind: UiMountedPaintPrimitiveKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedSpatialBatchRow {
    primitive_count: u32,
    hit_region_count: u32,
    overlay_row_count: u16,
    tool_state_row_count: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedRealtimeBatchRow {
    overlay_row_count: u16,
}

macro_rules! table {
    ($name:ident, $row:ty) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            rows: Box<[$row]>,
        }
        impl $name {
            pub fn new(rows: Vec<$row>) -> Self {
                Self {
                    rows: rows.into_boxed_slice(),
                }
            }
            pub fn rows(&self) -> &[$row] {
                &self.rows
            }
        }
    };
}

table!(UiMountedPaintBatchTable, UiMountedPaintBatchRow);
table!(UiMountedSpatialBatchTable, UiMountedSpatialBatchRow);
table!(UiMountedRealtimeBatchTable, UiMountedRealtimeBatchRow);

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedClipTable {
    status: UiMountedTableProjectionStatus,
    rows: Box<[UiMountedClipRow]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedLayerTable {
    status: UiMountedTableProjectionStatus,
    rows: Box<[UiMountedLayerRow]>,
}

macro_rules! optional_table {
    ($name:ident, $row:ty) => {
        impl $name {
            pub fn produced(rows: Vec<$row>) -> Self {
                Self {
                    status: UiMountedTableProjectionStatus::Produced,
                    rows: rows.into_boxed_slice(),
                }
            }
            pub fn omitted(reason: super::UiMountedOmissionReason) -> Self {
                Self {
                    status: UiMountedTableProjectionStatus::Omitted(reason),
                    rows: Box::new([]),
                }
            }
            pub fn status(&self) -> UiMountedTableProjectionStatus {
                self.status
            }
            pub fn rows(&self) -> &[$row] {
                &self.rows
            }
        }
    };
}

optional_table!(UiMountedClipTable, UiMountedClipRow);
optional_table!(UiMountedLayerTable, UiMountedLayerRow);

impl UiMountedClipRow {
    pub fn new(
        bounds: super::UiMountedCanonicalBox,
        parent: Option<UiMountedClipReference>,
    ) -> Self {
        Self { bounds, parent }
    }
    pub fn bounds(self) -> super::UiMountedCanonicalBox {
        self.bounds
    }
    pub fn parent(self) -> Option<UiMountedClipReference> {
        self.parent
    }
}

impl UiMountedLayerRow {
    pub fn new(semantic_order: u32, clip: UiMountedClipProjection) -> Self {
        Self {
            semantic_order,
            clip,
        }
    }
    pub fn semantic_order(self) -> u32 {
        self.semantic_order
    }
    pub fn clip(self) -> UiMountedClipProjection {
        self.clip
    }
}

impl UiMountedPaintBatchRow {
    pub fn new(
        primitive_count: u32,
        layer: UiMountedLayerProjection,
        resource: Option<super::UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Self {
        Self {
            primitive_count,
            layer,
            resource,
            primitive_kind,
        }
    }
    pub fn primitive_count(self) -> u32 {
        self.primitive_count
    }
    pub fn layer(self) -> UiMountedLayerProjection {
        self.layer
    }
    pub fn resource(self) -> Option<super::UiMountedResourceReference> {
        self.resource
    }
    pub fn primitive_kind(self) -> UiMountedPaintPrimitiveKind {
        self.primitive_kind
    }
}

impl UiMountedSpatialBatchRow {
    pub fn new(
        primitive_count: u32,
        hit_region_count: u32,
        overlay_row_count: u16,
        tool_state_row_count: u16,
    ) -> Self {
        Self {
            primitive_count,
            hit_region_count,
            overlay_row_count,
            tool_state_row_count,
        }
    }
    pub fn primitive_count(self) -> u32 {
        self.primitive_count
    }
    pub fn hit_region_count(self) -> u32 {
        self.hit_region_count
    }
    pub fn overlay_row_count(self) -> u16 {
        self.overlay_row_count
    }
    pub fn tool_state_row_count(self) -> u16 {
        self.tool_state_row_count
    }
}

impl UiMountedRealtimeBatchRow {
    pub fn new(overlay_row_count: u16) -> Self {
        Self { overlay_row_count }
    }
    pub fn overlay_row_count(self) -> u16 {
        self.overlay_row_count
    }
}

use super::super::WorthUiBoxEdges;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutKind {
    Row,
    Column,
    Inline,
    Stack,
    Grid,
    Spacer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutCrossAlign {
    Start,
    Center,
    End,
    Baseline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutFit {
    Hug,
    Fill,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFlowLayoutFill {
    None,
    Width,
    Height,
    Both,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiFlowLayoutReceipt {
    kind: WorthUiFlowLayoutKind,
    gap_token: String,
    gap_points: f32,
    padding_token: String,
    padding_edges: WorthUiBoxEdges,
    align: WorthUiFlowLayoutAlign,
    cross_align: WorthUiFlowLayoutCrossAlign,
    fit: WorthUiFlowLayoutFit,
    fill: WorthUiFlowLayoutFill,
    receipt_digest: u64,
}

impl WorthUiFlowLayoutReceipt {
    pub(crate) fn new(
        kind: WorthUiFlowLayoutKind,
        gap_token: impl Into<String>,
        gap_points: f32,
        padding_token: impl Into<String>,
        padding_edges: WorthUiBoxEdges,
        align: WorthUiFlowLayoutAlign,
        cross_align: WorthUiFlowLayoutCrossAlign,
        fit: WorthUiFlowLayoutFit,
        fill: WorthUiFlowLayoutFill,
        receipt_digest: u64,
    ) -> Self {
        Self {
            kind,
            gap_token: gap_token.into(),
            gap_points,
            padding_token: padding_token.into(),
            padding_edges,
            align,
            cross_align,
            fit,
            fill,
            receipt_digest,
        }
    }

    pub fn kind(&self) -> WorthUiFlowLayoutKind {
        self.kind
    }

    pub fn gap_points(&self) -> f32 {
        self.gap_points
    }

    pub fn gap_token(&self) -> &str {
        &self.gap_token
    }

    pub fn padding_points(&self) -> f32 {
        self.padding_edges.max_axis_point()
    }

    pub fn padding_edges(&self) -> WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn padding_token(&self) -> &str {
        &self.padding_token
    }

    pub fn align(&self) -> WorthUiFlowLayoutAlign {
        self.align
    }

    pub fn cross_align(&self) -> WorthUiFlowLayoutCrossAlign {
        self.cross_align
    }

    pub fn fit(&self) -> WorthUiFlowLayoutFit {
        self.fit
    }

    pub fn fill(&self) -> WorthUiFlowLayoutFill {
        self.fill
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

use crate::runtime::core::WebSignalKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementDeclarationOrigin {
    ExprSpec,
    CallbackSignalTracked,
    CallbackConstantizedNoSignalReads,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementDeclarationCandidate {
    pub id: String,
    pub signal_kind: Option<WebSignalKind>,
    pub origin: PlacementDeclarationOrigin,
    pub has_live_callback: bool,
    pub callback_runtime_read_count: usize,
    pub host_capability_read_count: usize,
    pub is_unavailable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NmtTopologyPattern {
    OpenWireChain(OpenWireChainSpec),
    OpenSheetPatch(OpenSheetPatchSpec),
    OpenRadialFan(OpenRadialFanSpec),
    OpenLayerStack(OpenLayerStackSpec),
}

impl NmtTopologyPattern {
    pub(crate) fn query_key(&self) -> &'static str {
        match self {
            Self::OpenWireChain(_) => "open-wire-chain",
            Self::OpenSheetPatch(_) => "open-sheet-patch",
            Self::OpenRadialFan(_) => "open-radial-fan",
            Self::OpenLayerStack(_) => "open-layer-stack",
        }
    }

    pub(crate) fn human_name(&self) -> &'static str {
        match self {
            Self::OpenWireChain(_) => "open wire chain topology",
            Self::OpenSheetPatch(_) => "open sheet patch topology",
            Self::OpenRadialFan(_) => "open radial fan topology",
            Self::OpenLayerStack(_) => "open layer stack topology",
        }
    }

    pub(crate) fn layer_count(&self) -> usize {
        match self {
            Self::OpenLayerStack(spec) => spec.layers,
            _ => 1,
        }
    }
}

impl From<OpenWireChainSpec> for NmtTopologyPattern {
    fn from(spec: OpenWireChainSpec) -> Self {
        Self::OpenWireChain(spec)
    }
}

impl From<OpenSheetPatchSpec> for NmtTopologyPattern {
    fn from(spec: OpenSheetPatchSpec) -> Self {
        Self::OpenSheetPatch(spec)
    }
}

impl From<OpenRadialFanSpec> for NmtTopologyPattern {
    fn from(spec: OpenRadialFanSpec) -> Self {
        Self::OpenRadialFan(spec)
    }
}

impl From<OpenLayerStackSpec> for NmtTopologyPattern {
    fn from(spec: OpenLayerStackSpec) -> Self {
        Self::OpenLayerStack(spec)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenWireChainSpec {
    edge_count: usize,
}

impl OpenWireChainSpec {
    pub fn new() -> Self {
        Self { edge_count: 4 }
    }

    pub fn edges(mut self, edge_count: usize) -> Self {
        self.edge_count = edge_count;
        self
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}

impl Default for OpenWireChainSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSheetPatchSpec {
    strips: usize,
}

impl OpenSheetPatchSpec {
    pub fn new() -> Self {
        Self { strips: 4 }
    }

    pub fn strips(mut self, strips: usize) -> Self {
        self.strips = strips;
        self
    }

    pub fn strip_count(&self) -> usize {
        self.strips
    }
}

impl Default for OpenSheetPatchSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenRadialFanSpec {
    incident_faces: usize,
}

impl OpenRadialFanSpec {
    pub fn new() -> Self {
        Self { incident_faces: 3 }
    }

    pub fn incident_faces(mut self, incident_faces: usize) -> Self {
        self.incident_faces = incident_faces;
        self
    }

    pub fn incident_face_count(&self) -> usize {
        self.incident_faces
    }
}

impl Default for OpenRadialFanSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenLayerPattern {
    WireChain(OpenWireChainSpec),
    SheetPatch(OpenSheetPatchSpec),
    RadialFan(OpenRadialFanSpec),
}

impl OpenLayerPattern {
    pub fn wire_chain(spec: OpenWireChainSpec) -> Self {
        Self::WireChain(spec)
    }

    pub fn sheet_patch(spec: OpenSheetPatchSpec) -> Self {
        Self::SheetPatch(spec)
    }

    pub fn radial_fan(spec: OpenRadialFanSpec) -> Self {
        Self::RadialFan(spec)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenLayerStackSpec {
    layers: usize,
    layer_pattern: OpenLayerPattern,
    layer_identity: bool,
    open_boundary_receipts: bool,
    radial_adjacency_receipts: bool,
}

impl OpenLayerStackSpec {
    pub fn new() -> Self {
        Self {
            layers: 2,
            layer_pattern: OpenLayerPattern::SheetPatch(OpenSheetPatchSpec::new()),
            layer_identity: false,
            open_boundary_receipts: false,
            radial_adjacency_receipts: false,
        }
    }

    pub fn layers(mut self, layers: usize) -> Self {
        self.layers = layers;
        self
    }

    pub fn layer_pattern(mut self, layer_pattern: OpenLayerPattern) -> Self {
        self.layer_pattern = layer_pattern;
        self
    }

    pub fn with_layer_identity(mut self) -> Self {
        self.layer_identity = true;
        self
    }

    pub fn with_open_boundary_receipts(mut self) -> Self {
        self.open_boundary_receipts = true;
        self
    }

    pub fn with_radial_adjacency_receipts(mut self) -> Self {
        self.radial_adjacency_receipts = true;
        self
    }

    pub fn layer_count(&self) -> usize {
        self.layers
    }

    pub fn requests_layer_identity_receipts(&self) -> bool {
        self.layer_identity
    }

    pub fn requests_open_boundary_receipts(&self) -> bool {
        self.open_boundary_receipts
    }

    pub fn requests_radial_adjacency_receipts(&self) -> bool {
        self.radial_adjacency_receipts
    }

    pub(crate) fn pattern(&self) -> &OpenLayerPattern {
        &self.layer_pattern
    }

    pub(crate) fn requires_layer_identity(&self) -> bool {
        self.requests_layer_identity_receipts()
    }

    pub(crate) fn requires_boundary_receipts(&self) -> bool {
        self.requests_open_boundary_receipts()
    }

    pub(crate) fn requires_radial_receipts(&self) -> bool {
        self.requests_radial_adjacency_receipts()
    }
}

impl Default for OpenLayerStackSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanSplitEdgeFragmentEndpointKind {
    OriginalSourceStart,
    SplitVertex,
    OriginalSourceEnd,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PlanarBooleanSplitEdgeFragmentEndpointRef {
    endpoint_kind: PlanarBooleanSplitEdgeFragmentEndpointKind,
    endpoint_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    parameter_bits: u64,
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl PlanarBooleanSplitEdgeFragmentEndpointRef {
    pub(crate) fn original_source_start(
        source_edge_identity: &str,
        carrier_identity: &str,
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        Self::new(
            PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart,
            format!("original-source-start:{source_edge_identity}:{carrier_identity}"),
            source_edge_identity,
            carrier_identity,
            0.0f64.to_bits(),
            local_frame_identity,
            precision_basis_identity,
        )
    }

    pub(crate) fn original_source_end(
        source_edge_identity: &str,
        carrier_identity: &str,
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        Self::new(
            PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd,
            format!("original-source-end:{source_edge_identity}:{carrier_identity}"),
            source_edge_identity,
            carrier_identity,
            1.0f64.to_bits(),
            local_frame_identity,
            precision_basis_identity,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn split_vertex(
        split_vertex_identity: &str,
        source_edge_identity: &str,
        carrier_identity: &str,
        parameter_bits: u64,
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        Self::new(
            PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex,
            split_vertex_identity.to_string(),
            source_edge_identity,
            carrier_identity,
            parameter_bits,
            local_frame_identity,
            precision_basis_identity,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        endpoint_kind: PlanarBooleanSplitEdgeFragmentEndpointKind,
        endpoint_identity: String,
        source_edge_identity: &str,
        carrier_identity: &str,
        parameter_bits: u64,
        local_frame_identity: &str,
        precision_basis_identity: &str,
    ) -> Self {
        Self {
            endpoint_kind,
            endpoint_identity,
            source_edge_identity: source_edge_identity.to_string(),
            carrier_identity: carrier_identity.to_string(),
            parameter_bits,
            local_frame_identity: local_frame_identity.to_string(),
            precision_basis_identity: precision_basis_identity.to_string(),
        }
    }

    pub fn endpoint_kind(&self) -> PlanarBooleanSplitEdgeFragmentEndpointKind {
        self.endpoint_kind
    }
    pub fn endpoint_identity(&self) -> &str {
        &self.endpoint_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn parameter_bits(&self) -> u64 {
        self.parameter_bits
    }
    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
}

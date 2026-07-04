#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyConsumerResidueOwner {
    WorthTopo,
    ForgeQuery,
}

impl TopologyConsumerResidueOwner {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorthTopo => "worth-topo",
            Self::ForgeQuery => "forge-query",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyConsumerResidueDisposition {
    ExplicitResidue,
    QueryGap,
    AuthoritativeOrdinaryConsumer,
}

impl TopologyConsumerResidueDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitResidue => "explicit-residue",
            Self::QueryGap => "query-gap",
            Self::AuthoritativeOrdinaryConsumer => "authoritative-ordinary-consumer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologyConsumerResidueRow {
    source_path: &'static str,
    current_surface: &'static str,
    owner: TopologyConsumerResidueOwner,
    disposition: TopologyConsumerResidueDisposition,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl TopologyConsumerResidueRow {
    pub const fn new(
        source_path: &'static str,
        current_surface: &'static str,
        owner: TopologyConsumerResidueOwner,
        disposition: TopologyConsumerResidueDisposition,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            source_path,
            current_surface,
            owner,
            disposition,
            blocker,
            removal_trigger,
        }
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn current_surface(&self) -> &'static str {
        self.current_surface
    }

    pub const fn owner(&self) -> TopologyConsumerResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> TopologyConsumerResidueDisposition {
        self.disposition
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

const TOPOLOGY_RESIDUE: [TopologyConsumerResidueRow; 2] = [
    TopologyConsumerResidueRow::new(
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs",
        "HistoricalEvaluationRequest::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())",
        TopologyConsumerResidueOwner::ForgeQuery,
        TopologyConsumerResidueDisposition::ExplicitResidue,
        "query-backed historical read-model path still declares retained reuse before phase 13 boundary cutover",
        "replace once Query-backed public/read-model consumers lower typed retained reuse products",
    ),
    TopologyConsumerResidueRow::new(
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs",
        "HistoricalCapabilityDescriptor::retained_snapshot(... HistoricalPathReuseDescriptor::retained_reuse())",
        TopologyConsumerResidueOwner::ForgeQuery,
        TopologyConsumerResidueDisposition::QueryGap,
        "historical capability lane remains blocked on Forge Query compiled-product-aware retained capability support",
        "remove once Forge Query exposes a compiled-product-aware historical retained capability boundary",
    ),
];

pub fn current_topology_consumer_residue_manifest() -> &'static [TopologyConsumerResidueRow] {
    &TOPOLOGY_RESIDUE
}

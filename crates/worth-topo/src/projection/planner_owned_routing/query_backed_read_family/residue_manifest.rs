#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryBackedConsumerResidueOwner {
    WorthTopo,
    ForgeQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryBackedConsumerResidueDisposition {
    ExplicitResidue,
    QueryGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueryBackedConsumerResidueRow {
    source_path: &'static str,
    current_surface: &'static str,
    owner: QueryBackedConsumerResidueOwner,
    disposition: QueryBackedConsumerResidueDisposition,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl QueryBackedConsumerResidueRow {
    pub const fn new(
        source_path: &'static str,
        current_surface: &'static str,
        owner: QueryBackedConsumerResidueOwner,
        disposition: QueryBackedConsumerResidueDisposition,
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

    pub const fn owner(&self) -> QueryBackedConsumerResidueOwner {
        self.owner
    }

    pub const fn disposition(&self) -> QueryBackedConsumerResidueDisposition {
        self.disposition
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }
}

const QUERY_BACKED_CONSUMER_RESIDUE: [QueryBackedConsumerResidueRow; 2] = [
    QueryBackedConsumerResidueRow::new(
        "crates/worth-topo/src/projection/read_views/domain/read_proof/parity.rs",
        "TopologyReadViewParityArtifact::view_digest_hex",
        QueryBackedConsumerResidueOwner::WorthTopo,
        QueryBackedConsumerResidueDisposition::ExplicitResidue,
        "determinism proof still records rendered-view parity alongside typed query/runtime proof for read-model certification",
        "remove once every public read-model consumer lowers only compiled-product and query-boundary identities without view-parity accompaniment",
    ),
    QueryBackedConsumerResidueRow::new(
        "crates/worth-topo/src/projection/runtime_boundary/read_execution/basis_context.rs",
        "HistoricalPathReuseDescriptor::retained_reuse()",
        QueryBackedConsumerResidueOwner::ForgeQuery,
        QueryBackedConsumerResidueDisposition::QueryGap,
        "historical retained-read capability still lives on a Forge Query retained-reuse descriptor that is not yet compiled-product-aware",
        "remove once Forge Query exposes a compiled-product-aware retained historical read boundary for topology consumers",
    ),
];

pub fn current_query_backed_consumer_residue_manifest() -> &'static [QueryBackedConsumerResidueRow]
{
    &QUERY_BACKED_CONSUMER_RESIDUE
}

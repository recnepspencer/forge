#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSourceLoopSplitAttributionKind {
    Preserved,
    SplitIntoMultipleIslands,
    ContributedToBornLoop,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSourceLoopSplitAttributionRow {
    attribution_identity: String,
    source_loop_identity: String,
    island_identities: Vec<String>,
    kind: PlanarBooleanSourceLoopSplitAttributionKind,
}

impl PlanarBooleanSourceLoopSplitAttributionRow {
    pub(crate) fn new(
        attribution_identity: String,
        source_loop_identity: String,
        island_identities: Vec<String>,
        kind: PlanarBooleanSourceLoopSplitAttributionKind,
    ) -> Self {
        Self {
            attribution_identity,
            source_loop_identity,
            island_identities,
            kind,
        }
    }

    pub fn attribution_identity(&self) -> &str {
        &self.attribution_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn island_identities(&self) -> &[String] {
        &self.island_identities
    }

    pub fn kind(&self) -> PlanarBooleanSourceLoopSplitAttributionKind {
        self.kind
    }
}

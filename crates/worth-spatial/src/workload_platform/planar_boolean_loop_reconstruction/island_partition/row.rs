#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopIslandKind {
    PreservedSourceLoop,
    BornFromOverlapNeighborhood,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIslandPartitionRow {
    island_identity: String,
    source_loop_identity: String,
    member_loop_identities: Vec<String>,
    kind: PlanarBooleanLoopIslandKind,
}

impl PlanarBooleanLoopIslandPartitionRow {
    pub(crate) fn new(
        island_identity: String,
        source_loop_identity: String,
        member_loop_identities: Vec<String>,
        kind: PlanarBooleanLoopIslandKind,
    ) -> Self {
        Self {
            island_identity,
            source_loop_identity,
            member_loop_identities,
            kind,
        }
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn member_loop_identities(&self) -> &[String] {
        &self.member_loop_identities
    }

    pub fn kind(&self) -> PlanarBooleanLoopIslandKind {
        self.kind
    }
}

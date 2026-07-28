#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualSnapshotRelation {
    Current,
    RetainedPredecessor,
    Historical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualSnapshotAffinity {
    snapshot: u64,
    presentation_attempt: u64,
    frame: u64,
    semantic_surface: u64,
    host_surface: u64,
    binding_generation: u64,
    presentation_epoch: u64,
    relation: UiVisualSnapshotRelation,
}

impl UiVisualSnapshotAffinity {
    #[doc(hidden)]
    pub const fn from_runtime_projection(
        values: [u64; 7],
        relation: UiVisualSnapshotRelation,
    ) -> Self {
        Self {
            snapshot: values[0],
            presentation_attempt: values[1],
            frame: values[2],
            semantic_surface: values[3],
            host_surface: values[4],
            binding_generation: values[5],
            presentation_epoch: values[6],
            relation,
        }
    }

    pub const fn snapshot(self) -> u64 {
        self.snapshot
    }

    pub const fn presentation_attempt(self) -> u64 {
        self.presentation_attempt
    }

    pub const fn frame(self) -> u64 {
        self.frame
    }

    pub const fn semantic_surface(self) -> u64 {
        self.semantic_surface
    }

    pub const fn host_surface(self) -> u64 {
        self.host_surface
    }

    pub const fn binding_generation(self) -> u64 {
        self.binding_generation
    }

    pub const fn presentation_epoch(self) -> u64 {
        self.presentation_epoch
    }

    pub const fn relation(self) -> UiVisualSnapshotRelation {
        self.relation
    }
}

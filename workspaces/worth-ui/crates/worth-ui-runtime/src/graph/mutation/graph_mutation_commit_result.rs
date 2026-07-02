use crate::graph::{
    UiGraphAuthority, UiGraphGeneration, UiGraphInstantiationLocalDenial, UiGraphSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMutationCommitResult {
    committed_snapshot: UiGraphSnapshot,
}

impl UiGraphMutationCommitResult {
    pub(crate) fn new(committed_snapshot: UiGraphSnapshot) -> Self {
        Self { committed_snapshot }
    }

    pub fn committed_generation(&self) -> UiGraphGeneration {
        self.committed_snapshot.generation()
    }

    pub fn graph(&self) -> UiGraphAuthority<'_> {
        UiGraphAuthority::new(&self.committed_snapshot)
    }

    pub(crate) fn into_committed_snapshot(self) -> UiGraphSnapshot {
        self.committed_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphMutationCommitDenial {
    local_denials: Vec<UiGraphInstantiationLocalDenial>,
}

impl UiGraphMutationCommitDenial {
    pub(crate) fn from_local_denials(local_denials: Vec<UiGraphInstantiationLocalDenial>) -> Self {
        Self { local_denials }
    }

    pub fn local_denials(&self) -> &[UiGraphInstantiationLocalDenial] {
        &self.local_denials
    }
}

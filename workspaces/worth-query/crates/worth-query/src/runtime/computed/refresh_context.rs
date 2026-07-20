use crate::memory_workspace::{WorthQueryCommitIdentity, WorthQuerySnapshotIdentity};
use crate::runtime::mutation::WorthQueryMutationMetadata;
use crate::runtime::WorthQueryAspectTouch;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRetainedRefreshOrigin {
    Mutation,
    DeclarationInitialization,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthQueryRetainedRefreshContext {
    origin: WorthQueryRetainedRefreshOrigin,
    refresh_identity: WorthQueryCommitIdentity,
    snapshot_identity: WorthQuerySnapshotIdentity,
    touched_aspects: Vec<WorthQueryAspectTouch>,
    refresh_metadata: WorthQueryMutationMetadata,
}

impl WorthQueryRetainedRefreshContext {
    pub(in crate::runtime) fn from_mutation(
        refresh_identity: WorthQueryCommitIdentity,
        snapshot_identity: WorthQuerySnapshotIdentity,
        touched_aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
        refresh_metadata: WorthQueryMutationMetadata,
    ) -> Self {
        let touched_aspects = touched_aspects
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        Self {
            origin: WorthQueryRetainedRefreshOrigin::Mutation,
            refresh_identity,
            snapshot_identity,
            touched_aspects,
            refresh_metadata,
        }
    }

    pub(in crate::runtime) fn from_declaration_initialization(
        refresh_identity: WorthQueryCommitIdentity,
        snapshot_identity: WorthQuerySnapshotIdentity,
        refresh_metadata: WorthQueryMutationMetadata,
    ) -> Self {
        Self {
            origin: WorthQueryRetainedRefreshOrigin::DeclarationInitialization,
            refresh_identity,
            snapshot_identity,
            touched_aspects: Vec::new(),
            refresh_metadata,
        }
    }

    pub fn origin(&self) -> WorthQueryRetainedRefreshOrigin {
        self.origin
    }

    pub fn refresh_identity(&self) -> &WorthQueryCommitIdentity {
        &self.refresh_identity
    }

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn admitted_touched_aspects(&self) -> &[WorthQueryAspectTouch] {
        &self.touched_aspects
    }

    pub fn refresh_metadata(&self) -> &WorthQueryMutationMetadata {
        &self.refresh_metadata
    }

    pub fn mutation_commit_identity(&self) -> Option<&WorthQueryCommitIdentity> {
        (self.origin == WorthQueryRetainedRefreshOrigin::Mutation).then_some(&self.refresh_identity)
    }
}

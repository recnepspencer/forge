use crate::memory_workspace::{ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity};
use crate::runtime::mutation::ForgeQueryMutationMetadata;
use crate::runtime::ForgeQueryAspectTouch;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRetainedRefreshOrigin {
    Mutation,
    DeclarationInitialization,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeQueryRetainedRefreshContext {
    origin: ForgeQueryRetainedRefreshOrigin,
    refresh_identity: ForgeQueryCommitIdentity,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    touched_aspects: Vec<ForgeQueryAspectTouch>,
    refresh_metadata: ForgeQueryMutationMetadata,
}

impl ForgeQueryRetainedRefreshContext {
    pub(in crate::runtime) fn from_mutation(
        refresh_identity: ForgeQueryCommitIdentity,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        touched_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        refresh_metadata: ForgeQueryMutationMetadata,
    ) -> Self {
        let touched_aspects = touched_aspects
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        Self {
            origin: ForgeQueryRetainedRefreshOrigin::Mutation,
            refresh_identity,
            snapshot_identity,
            touched_aspects,
            refresh_metadata,
        }
    }

    pub(in crate::runtime) fn from_declaration_initialization(
        refresh_identity: ForgeQueryCommitIdentity,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        refresh_metadata: ForgeQueryMutationMetadata,
    ) -> Self {
        Self {
            origin: ForgeQueryRetainedRefreshOrigin::DeclarationInitialization,
            refresh_identity,
            snapshot_identity,
            touched_aspects: Vec::new(),
            refresh_metadata,
        }
    }

    pub fn origin(&self) -> ForgeQueryRetainedRefreshOrigin {
        self.origin
    }

    pub fn refresh_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.refresh_identity
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn admitted_touched_aspects(&self) -> &[ForgeQueryAspectTouch] {
        &self.touched_aspects
    }

    pub fn refresh_metadata(&self) -> &ForgeQueryMutationMetadata {
        &self.refresh_metadata
    }

    pub fn mutation_commit_identity(&self) -> Option<&ForgeQueryCommitIdentity> {
        (self.origin == ForgeQueryRetainedRefreshOrigin::Mutation).then_some(&self.refresh_identity)
    }
}

use std::sync::Arc;

use crate::history::data::{CanonicalCommitEnvelope, CommitId};
use crate::identity::data::VersionId;

use super::{RelationalCommitIdentity, RelationalCommitParentage};

/// Descriptive roots carried by a commit identity before the Phase-5 branch
/// root becomes the read/currentness authority. The branch target owns the
/// concrete vocabulary so commit and reference observations cannot drift.
pub type RelationalCommitRootDescriptor = crate::branch::RelationalBranchRootDescriptor;

/// Sealed immutable catalog artifact. The envelope is held behind an `Arc` and
/// is never exposed mutably after catalog insertion.
#[derive(Debug)]
pub(crate) struct RelationalCommitArtifact {
    identity: RelationalCommitIdentity,
    parentage: RelationalCommitParentage,
    roots: RelationalCommitRootDescriptor,
    envelope: Arc<CanonicalCommitEnvelope>,
}

// The Phase-4 fork path may share an immutable catalog handle, but it may
// never deep-clone the sealed artifact.  This deliberately overlapping
// implementation is a compile-time tripwire: adding `Clone` to the artifact
// makes the two implementations conflict and fails the crate before a cost
// regression can hide behind a zero counter.
#[allow(dead_code)]
trait RelationalCommitArtifactCloneTripwire {}

#[allow(dead_code)]
impl RelationalCommitArtifactCloneTripwire for RelationalCommitArtifact {}

#[allow(dead_code)]
impl<T: Clone> RelationalCommitArtifactCloneTripwire for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalCommitArtifactDenial {
    Parentage,
}

impl RelationalCommitArtifact {
    pub(super) fn validate_envelope(
        envelope: &CanonicalCommitEnvelope,
    ) -> Result<(), RelationalCommitArtifactDenial> {
        RelationalCommitParentage::from_ordered(envelope.commit.parents.clone())
            .map(|_| ())
            .map_err(|_| RelationalCommitArtifactDenial::Parentage)
    }

    pub(super) fn from_envelope(
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<Self, RelationalCommitArtifactDenial> {
        let identity = RelationalCommitIdentity::new(
            envelope.commit.commit_id,
            envelope.commit.version_id,
            envelope.branch_context.clone(),
        );
        let parentage = RelationalCommitParentage::from_ordered(envelope.commit.parents.clone())
            .map_err(|_| RelationalCommitArtifactDenial::Parentage)?;
        Ok(Self {
            identity,
            parentage,
            roots: crate::branch::RelationalBranchTarget::roots_for_commit(&envelope.commit),
            envelope,
        })
    }

    pub fn identity(&self) -> &RelationalCommitIdentity {
        &self.identity
    }

    pub fn parentage(&self) -> &RelationalCommitParentage {
        &self.parentage
    }

    pub fn roots(&self) -> &RelationalCommitRootDescriptor {
        &self.roots
    }

    pub(crate) fn envelope(&self) -> &Arc<CanonicalCommitEnvelope> {
        &self.envelope
    }

    pub(crate) fn commit_id(&self) -> CommitId {
        self.identity.commit_id()
    }

    pub(crate) fn version_id(&self) -> VersionId {
        self.identity.version_id()
    }
}

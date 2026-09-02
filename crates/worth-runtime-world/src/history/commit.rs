use worth_relational::facade::history::RelationalCommitIdentity;
use worth_runtime_bridge::facade::AdmittedRuntimeWorldCorrespondenceBasis;
use worth_signal::facade::history::RuntimeBranch;

use crate::basis::AdmittedCompositeRuntimeWorldBasis;
use crate::identity::{
    CompositeCommitIdentity, CompositePublicationAttemptIdentity,
    RuntimeWorldBootstrapAttemptIdentity,
};

use super::OrdinaryParent;

/// Explicit change posture for one component in a composite commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositeComponentChangePosture {
    RetainExact,
    Published,
}

/// Root bootstrap or one ordinary publication attempt is the complete
/// occurrence provenance of a composite commit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositeCommitParent {
    Root,
    Ordinary(OrdinaryParent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositeCommitProvenance {
    Bootstrap(RuntimeWorldBootstrapAttemptIdentity),
    Publication(CompositePublicationAttemptIdentity),
}

/// Descriptive correlation supplied by a caller. It cannot authorize, select,
/// or identify a commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeCallerCorrelation(u128);

impl CompositeCallerCorrelation {
    pub const fn new(value: u128) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub(crate) struct CompositeComponentPublicationIdentities {
    pub(crate) relational: Option<RelationalCommitIdentity>,
    pub(crate) signal: Option<RuntimeBranch>,
}

impl CompositeComponentPublicationIdentities {
    pub(crate) const fn empty() -> Self {
        Self {
            relational: None,
            signal: None,
        }
    }
}

/// One immutable single-parent composite commit. The commit is materialized
/// before the product-reference compare-and-publish and contains no reference
/// movement itself.
#[derive(Debug)]
pub struct CompositeRuntimeWorldCommit {
    identity: CompositeCommitIdentity,
    parent: CompositeCommitParent,
    basis: AdmittedCompositeRuntimeWorldBasis,
    relational_change: CompositeComponentChangePosture,
    signal_change: CompositeComponentChangePosture,
    component_publications: CompositeComponentPublicationIdentities,
    correspondence_basis: AdmittedRuntimeWorldCorrespondenceBasis,
    provenance: CompositeCommitProvenance,
    caller_correlation: Option<CompositeCallerCorrelation>,
}

impl CompositeRuntimeWorldCommit {
    pub(crate) fn new(
        identity: CompositeCommitIdentity,
        parent: CompositeCommitParent,
        basis: AdmittedCompositeRuntimeWorldBasis,
        relational_change: CompositeComponentChangePosture,
        signal_change: CompositeComponentChangePosture,
        component_publications: CompositeComponentPublicationIdentities,
        correspondence_basis: AdmittedRuntimeWorldCorrespondenceBasis,
        provenance: CompositeCommitProvenance,
        caller_correlation: Option<CompositeCallerCorrelation>,
    ) -> Self {
        Self {
            identity,
            parent,
            basis,
            relational_change,
            signal_change,
            component_publications,
            correspondence_basis,
            provenance,
            caller_correlation,
        }
    }

    pub fn identity(&self) -> &CompositeCommitIdentity {
        &self.identity
    }

    pub fn parent(&self) -> &CompositeCommitParent {
        &self.parent
    }

    pub fn basis(&self) -> &AdmittedCompositeRuntimeWorldBasis {
        &self.basis
    }

    pub fn relational_change(&self) -> CompositeComponentChangePosture {
        self.relational_change
    }

    pub fn signal_change(&self) -> CompositeComponentChangePosture {
        self.signal_change
    }

    pub fn correspondence_basis(&self) -> &AdmittedRuntimeWorldCorrespondenceBasis {
        &self.correspondence_basis
    }

    pub fn provenance(&self) -> &CompositeCommitProvenance {
        &self.provenance
    }

    pub fn caller_correlation(&self) -> Option<CompositeCallerCorrelation> {
        self.caller_correlation
    }

    pub fn relational_publication_identity(&self) -> Option<&RelationalCommitIdentity> {
        self.component_publications.relational.as_ref()
    }

    pub fn signal_publication_identity(&self) -> Option<&RuntimeBranch> {
        self.component_publications.signal.as_ref()
    }

    pub(crate) fn component_publications(&self) -> &CompositeComponentPublicationIdentities {
        &self.component_publications
    }
}

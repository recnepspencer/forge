use std::sync::Arc;

use crate::history::CompositeRuntimeWorldCommit;
use crate::identity::{
    ProductBranchIdentity, ProductBranchLifecycleIncarnation, ProductBranchReferenceGeneration,
    RuntimeWorldOwnerIdentity,
};

/// One owner-issued immutable snapshot of the product reference cell. The
/// commit is the source of both the selected commit identity and composite
/// basis, so callers cannot mix those axes while constructing an observation.
#[derive(Debug, Clone)]
pub(crate) struct ProductBranchReferenceSnapshot {
    owner: RuntimeWorldOwnerIdentity,
    branch: ProductBranchIdentity,
    lifecycle: ProductBranchLifecycleIncarnation,
    generation: ProductBranchReferenceGeneration,
    commit: Arc<CompositeRuntimeWorldCommit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProductBranchReferenceSnapshotDenial {
    OwnerMismatch,
    BranchOwnerMismatch,
    LifecycleOwnerMismatch,
    CommitOwnerMismatch,
    BasisOwnerMismatch,
}

impl ProductBranchReferenceSnapshot {
    pub(crate) fn owner_issued(
        owner: RuntimeWorldOwnerIdentity,
        branch: ProductBranchIdentity,
        lifecycle: ProductBranchLifecycleIncarnation,
        generation: ProductBranchReferenceGeneration,
        commit: Arc<CompositeRuntimeWorldCommit>,
    ) -> Result<Self, ProductBranchReferenceSnapshotDenial> {
        if branch.owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::BranchOwnerMismatch);
        }
        if lifecycle.owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::LifecycleOwnerMismatch);
        }
        if commit.identity().owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::CommitOwnerMismatch);
        }
        if commit.basis().owner_identity() != owner {
            return Err(ProductBranchReferenceSnapshotDenial::BasisOwnerMismatch);
        }
        Ok(Self {
            owner,
            branch,
            lifecycle,
            generation,
            commit,
        })
    }

    pub(crate) const fn owner(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) fn branch(&self) -> &ProductBranchIdentity {
        &self.branch
    }

    pub(crate) const fn lifecycle(&self) -> ProductBranchLifecycleIncarnation {
        self.lifecycle
    }

    pub(crate) const fn generation(&self) -> ProductBranchReferenceGeneration {
        self.generation
    }

    pub(crate) fn commit(&self) -> &CompositeRuntimeWorldCommit {
        &self.commit
    }
}

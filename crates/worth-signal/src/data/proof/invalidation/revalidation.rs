use crate::data::aspect::{Aspect, AspectMask};
use crate::data::output::PartitionSubscription;

use super::binding::{DependencyRevision, PendingDependencyRevalidation, ResolvedDependencyCause};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CanonicalDependencyCauseSet {
    basis: CanonicalInvalidationBasis,
    dirty_aspects: AspectMask,
    dirty_scoped_aspects: Vec<(Aspect, PartitionSubscription)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CanonicalInvalidationBasis {
    DependencyCauses(Vec<ResolvedDependencyCause>),
    SourceRecompute(ResolvedDependencyBasis),
    StructuralRecompute(ResolvedDependencyBasis),
}

impl CanonicalDependencyCauseSet {
    pub(crate) fn from_dependency_causes(causes: Vec<ResolvedDependencyCause>) -> Self {
        let mut dirty_aspects = AspectMask::EMPTY;
        let mut dirty_scoped_aspects = Vec::new();
        for cause in &causes {
            dirty_aspects.insert(cause.key.aspect);
            dirty_scoped_aspects.extend(
                cause
                    .changed_scopes
                    .as_slice()
                    .iter()
                    .cloned()
                    .map(|scope| (cause.key.aspect, scope)),
            );
        }
        dirty_scoped_aspects.sort_unstable();
        dirty_scoped_aspects.dedup();
        Self {
            basis: CanonicalInvalidationBasis::DependencyCauses(causes),
            dirty_aspects,
            dirty_scoped_aspects,
        }
    }

    pub(crate) fn from_source_recompute(
        revision: DependencyRevision,
        dirty_aspects: AspectMask,
        mut dirty_scoped_aspects: Vec<(Aspect, PartitionSubscription)>,
    ) -> Self {
        dirty_scoped_aspects.sort_unstable();
        dirty_scoped_aspects.dedup();
        Self {
            basis: CanonicalInvalidationBasis::SourceRecompute(ResolvedDependencyBasis::new(
                revision,
            )),
            dirty_aspects,
            dirty_scoped_aspects,
        }
    }

    pub(crate) fn structural(revision: DependencyRevision) -> Self {
        Self {
            basis: CanonicalInvalidationBasis::StructuralRecompute(ResolvedDependencyBasis::new(
                revision,
            )),
            dirty_aspects: AspectMask::EMPTY,
            dirty_scoped_aspects: Vec::new(),
        }
    }

    pub(crate) const fn dirty_aspects(&self) -> AspectMask {
        self.dirty_aspects
    }

    pub(crate) fn dirty_scoped_aspects(&self) -> &[(Aspect, PartitionSubscription)] {
        &self.dirty_scoped_aspects
    }

    pub(crate) fn is_bound_to_revision(&self, revision: DependencyRevision) -> bool {
        match &self.basis {
            CanonicalInvalidationBasis::DependencyCauses(causes) => causes
                .iter()
                .all(|cause| cause.key.dependency_revision == revision),
            CanonicalInvalidationBasis::SourceRecompute(basis)
            | CanonicalInvalidationBasis::StructuralRecompute(basis) => {
                basis.is_bound_to_revision(revision)
            }
        }
    }

    pub(crate) const fn is_source_recompute(&self) -> bool {
        matches!(self.basis, CanonicalInvalidationBasis::SourceRecompute(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedDependencyBasis {
    dependency_revision: DependencyRevision,
}

impl ResolvedDependencyBasis {
    pub(crate) const fn new(dependency_revision: DependencyRevision) -> Self {
        Self {
            dependency_revision,
        }
    }

    pub(crate) const fn is_bound_to_revision(self, revision: DependencyRevision) -> bool {
        self.dependency_revision.0 == revision.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeInvalidationInput {
    Pending(PendingDependencyRevalidation),
    Resolved(CanonicalDependencyCauseSet),
    ResolvedNoChange(ResolvedDependencyBasis),
}

impl NodeInvalidationInput {
    pub(crate) fn resolved_dirty_aspects(&self) -> Option<AspectMask> {
        match self {
            Self::Pending(_) => None,
            Self::Resolved(causes) => Some(causes.dirty_aspects()),
            Self::ResolvedNoChange(_) => Some(AspectMask::EMPTY),
        }
    }
}

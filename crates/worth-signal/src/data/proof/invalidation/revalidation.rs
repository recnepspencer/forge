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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CanonicalInvalidationOrigin {
    DependencyCommit,
    SourceRecompute,
    StructuralRecompute,
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
        origin_generation: u64,
        dirty_aspects: AspectMask,
        mut dirty_scoped_aspects: Vec<(Aspect, PartitionSubscription)>,
    ) -> Self {
        dirty_scoped_aspects.sort_unstable();
        dirty_scoped_aspects.dedup();
        Self {
            basis: CanonicalInvalidationBasis::SourceRecompute(ResolvedDependencyBasis::new(
                revision,
                origin_generation,
            )),
            dirty_aspects,
            dirty_scoped_aspects,
        }
    }

    pub(crate) fn structural(revision: DependencyRevision) -> Self {
        Self {
            basis: CanonicalInvalidationBasis::StructuralRecompute(ResolvedDependencyBasis::new(
                revision, revision.0,
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

    pub(crate) const fn origin(&self) -> CanonicalInvalidationOrigin {
        match self.basis {
            CanonicalInvalidationBasis::DependencyCauses(_) => {
                CanonicalInvalidationOrigin::DependencyCommit
            }
            CanonicalInvalidationBasis::SourceRecompute(_) => {
                CanonicalInvalidationOrigin::SourceRecompute
            }
            CanonicalInvalidationBasis::StructuralRecompute(_) => {
                CanonicalInvalidationOrigin::StructuralRecompute
            }
        }
    }

    pub(crate) fn dependency_causes(&self) -> Option<&[ResolvedDependencyCause]> {
        match &self.basis {
            CanonicalInvalidationBasis::DependencyCauses(causes) => Some(causes),
            CanonicalInvalidationBasis::SourceRecompute(_)
            | CanonicalInvalidationBasis::StructuralRecompute(_) => None,
        }
    }

    pub(crate) const fn origin_generation(&self) -> Option<u64> {
        match &self.basis {
            CanonicalInvalidationBasis::SourceRecompute(basis)
            | CanonicalInvalidationBasis::StructuralRecompute(basis) => {
                Some(basis.origin_generation())
            }
            CanonicalInvalidationBasis::DependencyCauses(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedDependencyBasis {
    dependency_revision: DependencyRevision,
    origin_generation: u64,
}

impl ResolvedDependencyBasis {
    pub(crate) const fn new(
        dependency_revision: DependencyRevision,
        origin_generation: u64,
    ) -> Self {
        Self {
            dependency_revision,
            origin_generation,
        }
    }

    pub(crate) const fn is_bound_to_revision(self, revision: DependencyRevision) -> bool {
        self.dependency_revision.0 == revision.0
    }

    pub(crate) const fn origin_generation(self) -> u64 {
        self.origin_generation
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

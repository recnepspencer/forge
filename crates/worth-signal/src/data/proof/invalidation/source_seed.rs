use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::proof::PartitionScopeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceRecomputeSeed {
    source: NodeId,
    aspect: Aspect,
    changed_scopes: PartitionScopeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum DirectInvalidationBasis {
    InitialCompute,
    SourceRecompute {
        dirty_aspects: crate::data::aspect::AspectMask,
        scoped_aspects: Vec<(Aspect, crate::data::output::PartitionSubscription)>,
    },
}

impl DirectInvalidationBasis {
    pub(crate) const fn initial_compute() -> Self {
        Self::InitialCompute
    }

    pub(crate) fn from_seed(
        aspect: Aspect,
        scopes: impl IntoIterator<Item = crate::data::output::PartitionSubscription>,
    ) -> Self {
        let mut basis = Self::SourceRecompute {
            dirty_aspects: crate::data::aspect::AspectMask::EMPTY,
            scoped_aspects: Vec::new(),
        };
        basis.merge_seed(aspect, scopes);
        basis
    }

    pub(crate) fn merge_seed(
        &mut self,
        aspect: Aspect,
        scopes: impl IntoIterator<Item = crate::data::output::PartitionSubscription>,
    ) {
        if matches!(self, Self::InitialCompute) {
            return;
        }
        let Self::SourceRecompute {
            dirty_aspects,
            scoped_aspects,
        } = self
        else {
            unreachable!("direct invalidation basis has only two variants")
        };
        let was_already_whole = dirty_aspects
            .contains(crate::data::aspect::AspectMask::from_aspect(aspect))
            && !scoped_aspects
                .iter()
                .any(|(candidate, _)| *candidate == aspect);
        dirty_aspects.insert(aspect);
        let scopes = scopes.into_iter().collect::<Vec<_>>();
        if scopes.is_empty() {
            scoped_aspects.retain(|(candidate, _)| *candidate != aspect);
            return;
        }
        if was_already_whole {
            return;
        }
        for scope in scopes {
            if !scoped_aspects
                .iter()
                .any(|candidate| candidate == &(aspect, scope.clone()))
            {
                scoped_aspects.push((aspect, scope));
            }
        }
        scoped_aspects.sort_unstable();
    }

    pub(crate) const fn dirty_aspects(&self) -> crate::data::aspect::AspectMask {
        match self {
            Self::InitialCompute => crate::data::aspect::AspectMask::ALL,
            Self::SourceRecompute { dirty_aspects, .. } => *dirty_aspects,
        }
    }

    pub(crate) fn scoped_aspects(&self) -> &[(Aspect, crate::data::output::PartitionSubscription)] {
        match self {
            Self::InitialCompute => &[],
            Self::SourceRecompute { scoped_aspects, .. } => scoped_aspects,
        }
    }
}

impl SourceRecomputeSeed {
    pub(crate) fn new(source: NodeId, aspect: Aspect, changed_scopes: PartitionScopeSet) -> Self {
        Self {
            source,
            aspect,
            changed_scopes,
        }
    }

    pub(crate) const fn source(&self) -> NodeId {
        self.source
    }

    pub(crate) const fn aspect(&self) -> Aspect {
        self.aspect
    }

    pub(crate) fn changed_scopes(&self) -> &PartitionScopeSet {
        &self.changed_scopes
    }
}

#[cfg(test)]
mod tests {
    use super::DirectInvalidationBasis;
    use crate::data::aspect::Aspect;
    use crate::data::output::PartitionSubscription;

    #[test]
    fn whole_aspect_direct_basis_remains_stronger_than_scoped_follow_up() {
        let aspect = Aspect::new(2);
        let mut basis = DirectInvalidationBasis::from_seed(aspect, []);

        basis.merge_seed(aspect, [PartitionSubscription::whole_partition("curve")]);

        assert!(basis.scoped_aspects().is_empty());
    }

    #[test]
    fn whole_aspect_follow_up_supersedes_existing_direct_scopes() {
        let aspect = Aspect::new(2);
        let mut basis = DirectInvalidationBasis::from_seed(
            aspect,
            [PartitionSubscription::whole_partition("curve")],
        );

        basis.merge_seed(aspect, []);

        assert!(basis.scoped_aspects().is_empty());
    }
}

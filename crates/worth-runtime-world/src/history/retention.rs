use crate::identity::RuntimeWorldOwnerIdentity;
use crate::retention::ComponentBasisDependencyClass;

/// History's future retention lane consumes an exact component dependency
/// class; it never becomes a second owner-lease authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HistoryRetentionContract {
    owner: RuntimeWorldOwnerIdentity,
    dependency: ComponentBasisDependencyClass,
}

impl HistoryRetentionContract {
    pub(crate) const fn new(
        owner: RuntimeWorldOwnerIdentity,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self { owner, dependency }
    }

    pub(crate) const fn owner(self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) const fn dependency(self) -> ComponentBasisDependencyClass {
        self.dependency
    }
}

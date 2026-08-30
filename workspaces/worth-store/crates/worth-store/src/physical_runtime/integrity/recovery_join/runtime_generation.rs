use crate::physical_runtime::LifecycleGeneration;

/// Store-owned lifecycle generation to which recovery integrity facts are bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct RecoveryIntegrityRuntimeGeneration(LifecycleGeneration);

impl RecoveryIntegrityRuntimeGeneration {
    pub(in crate::physical_runtime) const fn bind(generation: LifecycleGeneration) -> Self {
        Self(generation)
    }

    pub(in crate::physical_runtime) const fn generation(self) -> LifecycleGeneration {
        self.0
    }

    pub(in crate::physical_runtime) const fn matches(self, current: LifecycleGeneration) -> bool {
        self.0.get() == current.get()
    }
}

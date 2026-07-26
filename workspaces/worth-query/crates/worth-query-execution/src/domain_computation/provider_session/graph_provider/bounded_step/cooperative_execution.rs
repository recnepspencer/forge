/// Move-only proof that a native provider execution was admitted through the
/// exact runtime-owned start or restore port.
///
/// The wrapper's fields and constructor are private. Returning a raw execution
/// from provider authoring code therefore cannot satisfy the provider contract.
#[must_use = "cooperative execution admission must be returned to the runtime"]
pub struct WorthQueryCooperativeGraphProviderExecution<E> {
    arena_identity: u64,
    execution: E,
}

impl<E> WorthQueryCooperativeGraphProviderExecution<E> {
    pub(super) const fn new(arena_identity: u64, execution: E) -> Self {
        Self {
            arena_identity,
            execution,
        }
    }

    pub(super) const fn arena_identity(&self) -> u64 {
        self.arena_identity
    }

    pub(super) fn into_execution(self) -> E {
        self.execution
    }
}

impl<E> std::fmt::Debug for WorthQueryCooperativeGraphProviderExecution<E> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryCooperativeGraphProviderExecution")
            .field("arena_identity", &self.arena_identity)
            .finish_non_exhaustive()
    }
}

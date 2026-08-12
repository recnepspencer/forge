use std::sync::{Arc, Mutex};

use crate::logic::runtime::RelationalRuntime;

#[derive(Clone)]
pub(crate) enum RelationalVisibilityRuntimeAuthority {
    Immutable(Arc<RelationalRuntime>),
    Shared(Arc<Mutex<RelationalRuntime>>),
}

impl RelationalVisibilityRuntimeAuthority {
    pub(crate) fn immutable(runtime: Arc<RelationalRuntime>) -> Self {
        Self::Immutable(runtime)
    }

    pub(crate) fn shared(runtime: Arc<Mutex<RelationalRuntime>>) -> Self {
        Self::Shared(runtime)
    }

    pub(crate) fn with_runtime<T>(&self, read: impl FnOnce(&RelationalRuntime) -> T) -> T {
        match self {
            Self::Immutable(runtime) => read(runtime),
            Self::Shared(runtime) => {
                let runtime = runtime
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                read(&runtime)
            }
        }
    }

    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.with_runtime(RelationalRuntime::runtime_instance_id)
    }
}

impl std::fmt::Debug for RelationalVisibilityRuntimeAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RelationalVisibilityRuntimeAuthority")
            .field("runtime_instance_id", &self.runtime_instance_id())
            .field(
                "ownership",
                &match self {
                    Self::Immutable(_) => "immutable",
                    Self::Shared(_) => "shared",
                },
            )
            .finish()
    }
}

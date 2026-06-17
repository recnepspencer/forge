use std::sync::Arc;

use arc_swap::ArcSwapOption;

use super::ForgeQuerySharedReadGenerationEntry;

#[derive(Debug, Default)]
pub(in crate::runtime) struct ForgeQuerySharedReadCurrentGeneration {
    entry: ArcSwapOption<ForgeQuerySharedReadGenerationEntry>,
}

impl ForgeQuerySharedReadCurrentGeneration {
    pub(in crate::runtime) fn load(&self) -> Option<Arc<ForgeQuerySharedReadGenerationEntry>> {
        self.entry.load_full()
    }

    pub(in crate::runtime) fn publish(&self, entry: Arc<ForgeQuerySharedReadGenerationEntry>) {
        self.entry.store(Some(entry));
    }

    pub(in crate::runtime) fn clear_if_generation(
        &self,
        entry: &Arc<ForgeQuerySharedReadGenerationEntry>,
    ) {
        let Some(current) = self.entry.load_full() else {
            return;
        };
        if Arc::ptr_eq(&current, entry) {
            self.entry.store(None);
        }
    }
}

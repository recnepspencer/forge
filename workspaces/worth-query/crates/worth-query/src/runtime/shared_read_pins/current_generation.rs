use std::sync::Arc;

use arc_swap::ArcSwapOption;

use super::WorthQuerySharedReadGenerationEntry;

#[derive(Debug, Default)]
pub(in crate::runtime) struct WorthQuerySharedReadCurrentGeneration {
    entry: ArcSwapOption<WorthQuerySharedReadGenerationEntry>,
}

impl WorthQuerySharedReadCurrentGeneration {
    pub(in crate::runtime) fn load(&self) -> Option<Arc<WorthQuerySharedReadGenerationEntry>> {
        self.entry.load_full()
    }

    pub(in crate::runtime) fn publish(&self, entry: Arc<WorthQuerySharedReadGenerationEntry>) {
        self.entry.store(Some(entry));
    }

    pub(in crate::runtime) fn clear_if_generation(
        &self,
        entry: &Arc<WorthQuerySharedReadGenerationEntry>,
    ) {
        let Some(current) = self.entry.load_full() else {
            return;
        };
        if Arc::ptr_eq(&current, entry) {
            self.entry.store(None);
        }
    }
}

use std::sync::{Arc, Weak};

use super::PhysicalSubmissionState;

pub(in crate::physical_runtime) struct PhysicalEffectActivity {
    state: Weak<PhysicalSubmissionState>,
}

impl PhysicalEffectActivity {
    pub(super) fn new(state: &Arc<PhysicalSubmissionState>) -> Self {
        Self {
            state: Arc::downgrade(state),
        }
    }
}

impl Drop for PhysicalEffectActivity {
    fn drop(&mut self) {
        if let Some(state) = self.state.upgrade() {
            state.finish_effect();
        }
    }
}

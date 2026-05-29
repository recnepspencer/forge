use std::sync::Mutex;

use crate::logic::runtime::RuntimeComplexityCounters;

#[derive(Debug, Default)]
pub(crate) struct RuntimeInstrumentation {
    pub(crate) complexity_counters: Mutex<RuntimeComplexityCounters>,
}

impl RuntimeInstrumentation {
    pub(crate) fn new() -> Self {
        Self {
            complexity_counters: Mutex::new(RuntimeComplexityCounters::default()),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            complexity_counters: Mutex::new(
                self.complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .clone(),
            ),
        }
    }

    pub(crate) fn count(&self, update: impl FnOnce(&mut RuntimeComplexityCounters)) {
        update(
            &mut self
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned"),
        );
    }
}

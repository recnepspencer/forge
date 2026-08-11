//! Process-local owner of the fault posture selected through test control.

use std::sync::Mutex;

use super::FaultScript;

pub(crate) struct FaultSelection {
    selected: Mutex<FaultScript>,
}

impl FaultSelection {
    pub(crate) fn new() -> Self {
        Self {
            selected: Mutex::new(FaultScript::Succeed),
        }
    }

    pub(crate) fn select(&self, script: FaultScript) {
        *self.selected.lock().expect("fault selection lock poisoned") = script;
    }

    pub(crate) fn current(&self) -> FaultScript {
        *self.selected.lock().expect("fault selection lock poisoned")
    }
}

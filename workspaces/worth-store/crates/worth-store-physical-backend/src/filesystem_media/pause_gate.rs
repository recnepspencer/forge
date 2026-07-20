use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug, Clone)]
pub struct MediaPauseGate {
    state: Arc<(Mutex<PauseState>, Condvar)>,
}

#[derive(Debug, Default)]
struct PauseState {
    reached: bool,
    released: bool,
    context: Option<super::MediaOperationContext>,
}

impl MediaPauseGate {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(crate) fn for_certification() -> Self {
        Self {
            state: Arc::new((Mutex::new(PauseState::default()), Condvar::new())),
        }
    }

    pub fn wait_until_reached(&self) {
        let (lock, condition) = &*self.state;
        let mut state = lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        while !state.reached {
            state = condition
                .wait(state)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    pub fn release(&self) {
        let (lock, condition) = &*self.state;
        let mut state = lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.released = true;
        condition.notify_all();
    }

    pub fn reached_context(&self) -> Option<super::MediaOperationContext> {
        let (lock, _) = &*self.state;
        lock.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .context
    }

    pub(super) fn pause(&self, context: Option<super::MediaOperationContext>) {
        let (lock, condition) = &*self.state;
        let mut state = lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.reached = true;
        state.context = context;
        condition.notify_all();
        while !state.released {
            state = condition
                .wait(state)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }
}

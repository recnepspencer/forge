pub(super) struct ManualClock {
    now: std::sync::Mutex<std::time::SystemTime>,
}

impl ManualClock {
    pub(super) fn new(now: std::time::SystemTime) -> Self {
        Self {
            now: std::sync::Mutex::new(now),
        }
    }

    pub(super) fn set(&self, now: std::time::SystemTime) {
        *self.now.lock().unwrap() = now;
    }
}

impl crate::OfflineInspectionClock for ManualClock {
    fn now(&self) -> std::time::SystemTime {
        *self.now.lock().unwrap()
    }
}

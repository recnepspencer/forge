#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentAttemptLineage(u64);

pub(crate) struct UiIntentAttemptLineageState {
    next: u64,
}

impl UiIntentAttemptLineageState {
    pub(crate) const fn new() -> Self {
        Self { next: 1 }
    }

    pub(crate) fn issue(&mut self) -> Option<UiIntentAttemptLineage> {
        let lineage = self.preview()?;
        self.next += 1;
        Some(lineage)
    }

    pub(crate) fn preview(&self) -> Option<UiIntentAttemptLineage> {
        self.next.checked_add(1)?;
        Some(UiIntentAttemptLineage(self.next))
    }
}

impl UiIntentAttemptLineage {
    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}

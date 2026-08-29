#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiPortalShutdownReport {
    closed_records: usize,
    abandoned_indeterminate_records: usize,
    final_active_records: usize,
}

impl super::UiPortalRuntimeState {
    pub(crate) fn shutdown(&mut self) -> UiPortalShutdownReport {
        let mut closed_records = 0usize;
        for record in self.records.values_mut() {
            if record.posture == crate::runtime::portal::UiPortalLifecyclePosture::Closed {
                continue;
            }
            closed_records = closed_records.saturating_add(1);
            terminalize(record);
        }
        self.last_closed = None;
        if closed_records > 0 {
            self.revision = self.revision.saturating_add(1);
        }
        let report = UiPortalShutdownReport {
            closed_records,
            abandoned_indeterminate_records: 0,
            final_active_records: self.active_count(),
        };
        self.records.clear();
        report
    }
}

fn terminalize(record: &mut super::UiPortalRecord) {
    record.posture = crate::runtime::portal::UiPortalLifecyclePosture::Closed;
    record.dismissal = Some(crate::runtime::portal::UiPortalDismissalCause::ApplicationShutdown);
    record.placement = None;
}

impl UiPortalShutdownReport {
    pub(crate) const fn closed_records(self) -> usize {
        self.closed_records
    }

    pub(crate) const fn abandoned_indeterminate_records(self) -> usize {
        self.abandoned_indeterminate_records
    }

    pub(crate) const fn final_active_records(self) -> usize {
        self.final_active_records
    }
}

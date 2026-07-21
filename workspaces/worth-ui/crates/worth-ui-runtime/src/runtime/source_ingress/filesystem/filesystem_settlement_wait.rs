use std::time::Duration;

#[derive(Clone, Copy)]
pub(super) struct WorthUiFilesystemSettlementWait {
    pub(super) duration: Duration,
    pub(super) permits_snapshot_freeze: bool,
}

pub(super) fn settlement_wait(
    has_pending_events: bool,
    quiet_window: Duration,
    remaining: Duration,
) -> WorthUiFilesystemSettlementWait {
    let required_settlement = quiet_window.saturating_add(quiet_window);
    if !has_pending_events || remaining <= required_settlement {
        WorthUiFilesystemSettlementWait {
            duration: remaining,
            permits_snapshot_freeze: false,
        }
    } else {
        WorthUiFilesystemSettlementWait {
            duration: quiet_window,
            permits_snapshot_freeze: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::settlement_wait;

    #[test]
    fn deadline_limited_event_wait_cannot_start_another_quiet_window() {
        let wait = settlement_wait(true, Duration::from_secs(5), Duration::from_millis(25));

        assert_eq!(wait.duration, Duration::from_millis(25));
        assert!(!wait.permits_snapshot_freeze);
    }
}

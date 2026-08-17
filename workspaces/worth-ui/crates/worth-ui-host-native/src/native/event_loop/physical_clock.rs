use std::time::{Duration, Instant};

use winit::event_loop::ControlFlow;

pub(super) struct UiNativePhysicalEventClock {
    epoch: Instant,
}

impl UiNativePhysicalEventClock {
    pub(super) fn new() -> Self {
        Self {
            epoch: Instant::now(),
        }
    }

    pub(super) fn current_tick(&self) -> u64 {
        elapsed_millis(self.epoch.elapsed())
    }

    pub(super) fn deadline(&self, tick: u64) -> Option<Instant> {
        self.epoch.checked_add(Duration::from_millis(tick))
    }
}

pub(super) fn tighten_deadline(current: ControlFlow, deadline: Instant) -> ControlFlow {
    match current {
        ControlFlow::Poll => ControlFlow::Poll,
        ControlFlow::Wait => ControlFlow::WaitUntil(deadline),
        ControlFlow::WaitUntil(current) => ControlFlow::WaitUntil(current.min(deadline)),
    }
}

fn elapsed_millis(elapsed: Duration) -> u64 {
    elapsed.as_millis().min(u128::from(u64::MAX)) as u64
}

#[cfg(test)]
mod tests {
    use super::{elapsed_millis, tighten_deadline};
    use std::time::{Duration, Instant};
    use winit::event_loop::ControlFlow;

    #[test]
    fn physical_signal_deadline_only_tightens_event_loop_waiting() {
        let now = Instant::now();
        let early = now + Duration::from_millis(8);
        let late = now + Duration::from_millis(13);
        assert_eq!(
            tighten_deadline(ControlFlow::Poll, early),
            ControlFlow::Poll
        );
        assert_eq!(
            tighten_deadline(ControlFlow::Wait, early),
            ControlFlow::WaitUntil(early)
        );
        assert_eq!(
            tighten_deadline(ControlFlow::WaitUntil(late), early),
            ControlFlow::WaitUntil(early)
        );
        assert_eq!(
            tighten_deadline(ControlFlow::WaitUntil(early), late),
            ControlFlow::WaitUntil(early)
        );
        assert_eq!(elapsed_millis(Duration::from_micros(7_999)), 7);
        assert_eq!(elapsed_millis(Duration::from_millis(8)), 8);
    }
}

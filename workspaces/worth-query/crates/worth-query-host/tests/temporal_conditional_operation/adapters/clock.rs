use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use worth_query_host::facade::domain;

pub struct CourtroomClock;

impl domain::WorthQueryNamedClock for CourtroomClock {
    const PORTABLE_IDENTITY: &'static str = "worth.query.host.courtroom.clock";
}

#[derive(Clone)]
pub struct ClockController {
    scripted: Arc<Mutex<VecDeque<(u64, u64)>>>,
}

impl ClockController {
    pub fn push(&self, sequence: u64, now: u64) {
        self.scripted.lock().unwrap().push_back((sequence, now));
    }
}

pub struct ClockSource {
    sequence: u64,
    now: u64,
    controller: ClockController,
}

impl ClockSource {
    pub fn due() -> (Self, ClockController) {
        let controller = ClockController {
            scripted: Arc::new(Mutex::new(VecDeque::new())),
        };
        (
            Self {
                sequence: 0,
                now: 10,
                controller: controller.clone(),
            },
            controller,
        )
    }
}

impl domain::WorthQueryNamedClockSource<CourtroomClock> for ClockSource {
    const SEMANTIC_IDENTITY: &'static str = "worth.query.host.courtroom.clock-source";

    fn source_identity(&self) -> domain::WorthQueryClockSourceIdentity {
        domain::WorthQueryClockSourceIdentity::declare("courtroom-source").unwrap()
    }

    fn timeline_identity(&self) -> domain::WorthQueryClockTimelineIdentity {
        domain::WorthQueryClockTimelineIdentity::declare("courtroom-timeline").unwrap()
    }

    fn observe(
        &mut self,
    ) -> Result<
        domain::WorthQueryNamedClockReading<CourtroomClock>,
        domain::WorthQueryNamedClockFailure,
    > {
        if let Some((sequence, now)) = self.controller.scripted.lock().unwrap().pop_front() {
            self.sequence = sequence;
            self.now = now;
        } else {
            self.sequence += 1;
        }
        Ok(domain::WorthQueryNamedClockReading::new(
            self.sequence,
            domain::WorthQueryClockCoordinate::from_nanoseconds(self.now),
        ))
    }
}

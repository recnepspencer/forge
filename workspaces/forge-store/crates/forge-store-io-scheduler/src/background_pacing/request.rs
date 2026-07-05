use super::{BackgroundCapacityAdmission, BackgroundPacingDenial};

#[derive(Debug, Eq, PartialEq)]
pub struct BackgroundIdleCapacityLeaseRequest {
    capacity: BackgroundCapacityAdmission,
    foreground_pressure_events: u64,
    late_yield: bool,
}

impl BackgroundIdleCapacityLeaseRequest {
    pub const fn new(capacity: BackgroundCapacityAdmission) -> Self {
        Self {
            foreground_pressure_events: 0,
            late_yield: false,
            capacity,
        }
    }

    pub const fn with_foreground_pressure_events(
        mut self,
        foreground_pressure_events: u64,
    ) -> Self {
        self.foreground_pressure_events = foreground_pressure_events;
        self
    }

    pub const fn with_late_yield(mut self) -> Self {
        self.late_yield = true;
        self
    }

    pub const fn capacity(&self) -> &BackgroundCapacityAdmission {
        &self.capacity
    }
    pub const fn foreground_pressure_events(&self) -> u64 {
        self.foreground_pressure_events
    }
    pub const fn late_yield(&self) -> bool {
        self.late_yield
    }
}

pub const fn reject_raw_background_label_as_background_pacing_authority(
) -> Result<(), BackgroundPacingDenial> {
    Err(BackgroundPacingDenial::RawBackgroundLabelCannotPace)
}

pub const fn reject_semantic_lifecycle_receipt_as_background_pacing_authority(
) -> Result<(), BackgroundPacingDenial> {
    Err(BackgroundPacingDenial::SemanticLifecycleReceiptCannotPace)
}

pub const fn reject_log_line_as_background_pacing_authority() -> Result<(), BackgroundPacingDenial>
{
    Err(BackgroundPacingDenial::LogLineCannotPace)
}

pub const fn reject_elapsed_time_as_background_pacing_authority(
) -> Result<(), BackgroundPacingDenial> {
    Err(BackgroundPacingDenial::ElapsedTimeCannotPace)
}

pub const fn reject_worker_local_queue_as_background_pacing_authority(
) -> Result<(), BackgroundPacingDenial> {
    Err(BackgroundPacingDenial::WorkerLocalQueueCannotPace)
}

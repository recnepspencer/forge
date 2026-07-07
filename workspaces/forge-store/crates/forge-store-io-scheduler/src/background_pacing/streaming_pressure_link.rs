//! Maps streaming pressure classes to blob-chunks ingest/read admission law.
use super::class::BackgroundIoPressureClass;
use crate::foreground_reservation::ForegroundIoLaneKind;

pub const fn admits_blob_ingest_pressure(class: BackgroundIoPressureClass) -> bool {
    matches!(class, BackgroundIoPressureClass::IngestPressure)
}

pub const fn admits_verification_pressure(class: BackgroundIoPressureClass) -> bool {
    matches!(class, BackgroundIoPressureClass::VerificationPressure)
}

pub const fn ingest_pressure_foreground_lane_admits(lane: ForegroundIoLaneKind) -> bool {
    matches!(
        lane,
        ForegroundIoLaneKind::CommitCriticalWalWrite | ForegroundIoLaneKind::OrdinaryPageWrite
    )
}

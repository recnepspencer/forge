//! Maps foreground read lanes to blob streaming read admission law.
use super::lane::ForegroundIoLaneKind;

pub const fn admits_streaming_read_lane(lane: ForegroundIoLaneKind) -> bool {
    matches!(
        lane,
        ForegroundIoLaneKind::PointRead
            | ForegroundIoLaneKind::RangeRead
            | ForegroundIoLaneKind::InternalForegroundRead
            | ForegroundIoLaneKind::CommitCriticalWalWrite
            | ForegroundIoLaneKind::OrdinaryPageWrite
    )
}

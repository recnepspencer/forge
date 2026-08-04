//! Maps foreground read lanes to blob streaming read admission law.
use super::lane::ForegroundIoLaneKind;

pub const fn admits_streaming_read_lane(lane: ForegroundIoLaneKind) -> bool {
    matches!(
        lane,
        ForegroundIoLaneKind::PointRead
            | ForegroundIoLaneKind::RangeRead
            | ForegroundIoLaneKind::InternalForegroundRead
            | ForegroundIoLaneKind::CommitCriticalWalAppend
            | ForegroundIoLaneKind::CommitCriticalWalWrite
            | ForegroundIoLaneKind::RootPublication
            | ForegroundIoLaneKind::OrdinaryPageWrite
    )
}

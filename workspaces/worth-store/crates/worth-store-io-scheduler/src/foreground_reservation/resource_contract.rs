use super::{
    ForegroundIoLaneKind, ForegroundReservationAdmissionDenial, ForegroundResourceBudget,
    ForegroundResourceUnitKind,
};

pub(super) fn require_declared_resource_budget(
    requested: ForegroundResourceBudget,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    if requested.is_empty() {
        Err(ForegroundReservationAdmissionDenial::MissingDeclaredResourceBudget)
    } else {
        Ok(())
    }
}

pub(super) fn require_lane_resource_contract(
    lane: ForegroundIoLaneKind,
    requested: ForegroundResourceBudget,
) -> Result<(), ForegroundReservationAdmissionDenial> {
    for unit in required_units_for_lane(lane) {
        if requested.amount_for(*unit) == 0 {
            return Err(
                ForegroundReservationAdmissionDenial::MissingRequiredResourceUnit {
                    lane,
                    unit: *unit,
                },
            );
        }
    }
    Ok(())
}

const fn required_units_for_lane(
    lane: ForegroundIoLaneKind,
) -> &'static [ForegroundResourceUnitKind] {
    match lane {
        ForegroundIoLaneKind::PointRead
        | ForegroundIoLaneKind::RangeRead
        | ForegroundIoLaneKind::InteractiveRead
        | ForegroundIoLaneKind::InternalForegroundRead => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundResourceUnitKind::ReadAheadWindow,
            ForegroundResourceUnitKind::WorkerPermit,
            ForegroundResourceUnitKind::CacheResidencyHint,
        ],
        ForegroundIoLaneKind::ArtifactMetadataRead => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::WorkerPermit,
        ],
        ForegroundIoLaneKind::CommitCriticalWalAppend => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundResourceUnitKind::WorkerPermit,
        ],
        ForegroundIoLaneKind::CommitCriticalWalWrite => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundResourceUnitKind::FlushPermit,
            ForegroundResourceUnitKind::SyncDebt,
            ForegroundResourceUnitKind::WorkerPermit,
        ],
        ForegroundIoLaneKind::RootPublication => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundResourceUnitKind::FlushPermit,
            ForegroundResourceUnitKind::SyncDebt,
            ForegroundResourceUnitKind::WorkerPermit,
        ],
        ForegroundIoLaneKind::OrdinaryPageWrite => &[
            ForegroundResourceUnitKind::QueueSlot,
            ForegroundResourceUnitKind::BandwidthToken,
            ForegroundResourceUnitKind::WriteBackWindow,
            ForegroundResourceUnitKind::DirtyPageBudget,
            ForegroundResourceUnitKind::WorkerPermit,
        ],
    }
}

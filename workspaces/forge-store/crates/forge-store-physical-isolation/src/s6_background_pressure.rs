use forge_store_contracts::S6BackgroundPressureDeclaration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhysicalIsolationBackgroundPressureKind {
    CompactionRewrite,
    CheckpointFlush,
    ScrubScan,
}

pub const fn physical_isolation_compaction_background_pressure(
) -> PhysicalIsolationBackgroundPressureKind {
    PhysicalIsolationBackgroundPressureKind::CompactionRewrite
}

pub const fn physical_isolation_checkpoint_background_pressure(
) -> PhysicalIsolationBackgroundPressureKind {
    PhysicalIsolationBackgroundPressureKind::CheckpointFlush
}

pub const fn physical_isolation_scrub_background_pressure(
) -> PhysicalIsolationBackgroundPressureKind {
    PhysicalIsolationBackgroundPressureKind::ScrubScan
}

pub const fn physical_isolation_s6_background_pressure_declaration(
    pressure: PhysicalIsolationBackgroundPressureKind,
) -> S6BackgroundPressureDeclaration {
    match pressure {
        PhysicalIsolationBackgroundPressureKind::CompactionRewrite => {
            S6BackgroundPressureDeclaration::compaction_rewrite()
        }
        PhysicalIsolationBackgroundPressureKind::CheckpointFlush => {
            S6BackgroundPressureDeclaration::checkpoint_flush()
        }
        PhysicalIsolationBackgroundPressureKind::ScrubScan => {
            S6BackgroundPressureDeclaration::scrub_scan()
        }
    }
}

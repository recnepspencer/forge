use crate::{CurrentPhysicalRoot, ManifestEpoch, RootEpoch};
use worth_store_recovery_physics::{CheckpointCoveredLsnRange, PageLsn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointReadInterlockDenial {
    CheckpointPublicationRootNotReadmitted {
        checkpoint_root: RootEpoch,
        admitted_root: RootEpoch,
    },
    CheckpointPublicationRootCheckpointMismatch,
    StaleCheckpointRootEpoch {
        old_root: RootEpoch,
        published_root: RootEpoch,
    },
    StaleCheckpointManifestEpoch {
        old_manifest: ManifestEpoch,
        published_manifest: ManifestEpoch,
    },
    CheckpointCutoverReceiptMismatch,
    CheckpointCutoverRangeMismatch {
        validation_range: CheckpointCoveredLsnRange,
        receipt_range: CheckpointCoveredLsnRange,
    },
    PageLsnFrontierOutsideCutoverRange {
        page_lsn: PageLsn,
    },
    PrePublicationReadReceiptMismatch {
        expected: CurrentPhysicalRoot,
        observed: CurrentPhysicalRoot,
    },
    PostPublicationReadReceiptMismatch {
        expected: CurrentPhysicalRoot,
        observed: CurrentPhysicalRoot,
    },
    MixedRootDuringCheckpointPublication,
    CopiedCheckpointReportCannotAdmitReadInterlock,
    SameRunSelfComparisonCannotAdmitReadInterlock,
}

pub const fn reject_copied_checkpoint_report_as_checkpoint_interlock(
) -> CheckpointReadInterlockDenial {
    CheckpointReadInterlockDenial::CopiedCheckpointReportCannotAdmitReadInterlock
}

pub const fn reject_same_run_self_comparison_as_checkpoint_interlock(
) -> CheckpointReadInterlockDenial {
    CheckpointReadInterlockDenial::SameRunSelfComparisonCannotAdmitReadInterlock
}

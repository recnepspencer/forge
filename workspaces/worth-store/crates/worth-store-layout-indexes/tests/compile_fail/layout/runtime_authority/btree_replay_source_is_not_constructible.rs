use worth_store_layout_indexes::BaselineBTreeReplayAdmission;
use worth_store_recovery_physics::{AdmittedBTreeReplayPhysicalSource, AdmittedBTreeReplaySource};

fn worth(
    intent: BaselineBTreeReplayAdmission,
    physical: AdmittedBTreeReplayPhysicalSource,
) -> AdmittedBTreeReplaySource<BaselineBTreeReplayAdmission> {
    AdmittedBTreeReplaySource { intent, physical }
}

fn main() {}

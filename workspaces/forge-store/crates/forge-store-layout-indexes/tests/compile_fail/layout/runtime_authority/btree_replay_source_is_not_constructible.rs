use forge_store_layout_indexes::BaselineBTreeReplayAdmission;
use forge_store_recovery_physics::{AdmittedBTreeReplayPhysicalSource, AdmittedBTreeReplaySource};

fn forge(
    intent: BaselineBTreeReplayAdmission,
    physical: AdmittedBTreeReplayPhysicalSource,
) -> AdmittedBTreeReplaySource<BaselineBTreeReplayAdmission> {
    AdmittedBTreeReplaySource { intent, physical }
}

fn main() {}

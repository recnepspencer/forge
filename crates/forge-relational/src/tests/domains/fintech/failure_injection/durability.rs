use std::path::PathBuf;

use crate::facade::durability::RecoveryPlan;

pub(crate) fn corrupt_latest_checkpoint_file(plan: &RecoveryPlan) -> Option<PathBuf> {
    let store = plan.store.as_ref()?;
    let latest = store.checkpoints.last()?;
    std::fs::write(&latest.path, b"{not-json").ok()?;
    Some(latest.path.clone())
}

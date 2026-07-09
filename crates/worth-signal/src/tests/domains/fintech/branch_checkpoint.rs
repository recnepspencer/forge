use crate::facade::*;

use super::audit_surface::PrimaryAuditSurface;

#[derive(Debug, Clone)]
pub(super) struct BranchCheckpoint {
    pub branch: SignalBranchHandle,
    pub snapshot: SignalSnapshotV1,
    pub audit: PrimaryAuditSurface,
}

impl BranchCheckpoint {
    pub(super) fn new(
        branch: SignalBranchHandle,
        snapshot: SignalSnapshotV1,
        audit: PrimaryAuditSurface,
    ) -> Self {
        Self {
            branch,
            snapshot,
            audit,
        }
    }
}

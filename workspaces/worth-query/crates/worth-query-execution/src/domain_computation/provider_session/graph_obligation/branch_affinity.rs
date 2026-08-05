use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalExecutionBasisIdentity;
use worth_relational::facade::snapshots::SnapshotHandle;
use worth_runtime_bridge::facade::TruthBranchIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryGraphWorkBranchAffinity {
    relational: BranchId,
    truth: TruthBranchIdentity,
}

impl WorthQueryGraphWorkBranchAffinity {
    pub(super) fn from_query_basis(basis: &RelationalExecutionBasisIdentity) -> Self {
        Self::from_relational_branch(basis.branch_id().clone())
    }

    pub(super) fn from_snapshot(snapshot: &SnapshotHandle) -> Self {
        Self::from_relational_branch(snapshot.branch_id.clone())
    }

    fn from_relational_branch(relational: BranchId) -> Self {
        let truth = TruthBranchIdentity::from_relational_branch_id(relational.0.clone());
        Self { relational, truth }
    }

    pub(in crate::domain_computation) const fn relational(&self) -> &BranchId {
        &self.relational
    }

    pub(in crate::domain_computation) const fn truth(&self) -> &TruthBranchIdentity {
        &self.truth
    }

    pub(super) fn admits_query_basis(&self, basis: &RelationalExecutionBasisIdentity) -> bool {
        basis.branch_id() == &self.relational
    }

    pub(super) fn admits_snapshot(&self, snapshot: &SnapshotHandle) -> bool {
        snapshot.branch_id == self.relational
    }
}

#[cfg(test)]
mod tests {
    use worth_relational::facade::{history::BranchId, snapshots::SnapshotHandle};
    use worth_runtime_bridge::facade::TruthBranchIdentity;

    use super::WorthQueryGraphWorkBranchAffinity;

    #[test]
    fn equal_version_snapshot_from_another_branch_cannot_satisfy_affinity() {
        let admitted = SnapshotHandle::new(7, 19, BranchId("ordinary".to_owned()));
        let substitute = SnapshotHandle::new(7, 19, BranchId("hostile".to_owned()));
        let affinity = WorthQueryGraphWorkBranchAffinity::from_snapshot(&admitted);

        assert!(affinity.admits_snapshot(&admitted));
        assert!(!affinity.admits_snapshot(&substitute));
        assert_eq!(affinity.relational(), &admitted.branch_id);
        assert_eq!(
            affinity.truth(),
            &TruthBranchIdentity::from_relational_branch_id(admitted.branch_id.0)
        );
    }
}

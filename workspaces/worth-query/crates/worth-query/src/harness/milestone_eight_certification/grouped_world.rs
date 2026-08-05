use super::*;

#[derive(Clone)]
pub(super) struct GroupedRowFixture {
    member_key: String,
    display_name: AspectValue,
    lane: AspectValue,
}

impl GroupedRowFixture {
    pub(super) fn new(member_key: &str, display_name: &str, lane: &str) -> Self {
        Self {
            member_key: member_key.to_string(),
            display_name: crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(
                display_name,
            ),
            lane: crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(lane),
        }
    }

    pub(super) fn member_key(&self) -> &str {
        &self.member_key
    }

    pub(super) fn value_for_snapshot_read(&self, aspect_key: &str) -> AspectValue {
        match aspect_key {
            "identity.id" => crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value(
                self.member_key.as_str(),
            ),
            "profile.display_name" => self.display_name.clone(),
            "status.lane" => self.lane.clone(),
            _ => crate::runtime::WorthQueryAuthoredAspectMutation::native_string_value("unknown"),
        }
    }
}

pub(super) fn grouped_row(member_key: &str, display_name: &str, lane: &str) -> GroupedRowFixture {
    GroupedRowFixture::new(member_key, display_name, lane)
}

pub(super) fn milestone_eight_snapshot_parts() -> RelationalBridgeSnapshotIdentityParts {
    RelationalBridgeSnapshotIdentityParts::new(1, 1)
}

pub(super) fn milestone_eight_snapshot_identity() -> TruthSnapshotIdentity {
    TruthSnapshotIdentity::from_relational_snapshot(milestone_eight_snapshot_parts())
}

pub(super) fn milestone_eight_branch_identity() -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id("analysis")
}

pub(super) fn milestone_eight_record_parts(
    member_key: &str,
) -> RelationalBridgeRecordIdentityParts {
    RelationalBridgeRecordIdentityParts::entity(
        1,
        milestone_eight_fixture_position("record", member_key),
        1,
    )
}

pub(super) fn milestone_eight_head_commit_identity(
    branch_identity: &TruthBranchIdentity,
) -> TruthCommitIdentity {
    let branch_id = branch_identity
        .relational_branch_id()
        .expect("milestone eight branch head fixture must carry relational branch authority");
    TruthCommitIdentity::from_relational_commit_id(milestone_eight_fixture_position(
        "branch-head",
        branch_id,
    ))
}

pub(super) fn milestone_eight_patch_identity_for_commit(
    commit_identity: &TruthCommitIdentity,
) -> TruthPatchIdentity {
    let commit_id = commit_identity
        .relational_commit_id()
        .expect("milestone eight patch fixture must carry relational commit authority");
    TruthPatchIdentity::from_relational_patch_position(commit_id)
}

pub(super) fn milestone_eight_patch_identity_for_branch(
    branch_identity: &TruthBranchIdentity,
) -> TruthPatchIdentity {
    let branch_id = branch_identity
        .relational_branch_id()
        .expect("milestone eight branch patch fixture must carry relational branch authority");
    TruthPatchIdentity::from_relational_patch_position(milestone_eight_fixture_position(
        "branch-patch",
        branch_id,
    ))
}

pub(super) fn milestone_eight_fixture_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}

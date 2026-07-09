use crate::facade::{
    BridgeTruthViewPolicyResolution, HistoricalEvaluationDeclaration, ResolvedTruthViewPolicy,
    RuntimeBridge, TruthViewPolicyRejectionKind, TruthViewReplayContinuity,
    TruthViewRetentionAdmission, TruthViewSourceCapability,
};
use crate::snapshot::{BridgeReplayMode, BridgeTruthViewKind, BridgeTruthViewPolicyRejection};

impl RuntimeBridge {
    pub fn resolve_truth_view_policy(
        &self,
        declaration: &HistoricalEvaluationDeclaration,
    ) -> BridgeTruthViewPolicyResolution {
        if declaration.replay_mode() == BridgeReplayMode::Required
            && !self.policy.allow_replay_artifacts()
        {
            return BridgeTruthViewPolicyResolution::Rejected(
                BridgeTruthViewPolicyRejection::new(
                    declaration,
                    TruthViewPolicyRejectionKind::ReplayNotPermitted,
                    "runtime policy does not permit replay artifacts for required historical evaluation replay",
                ),
            );
        }

        match declaration.selector().view_kind() {
            BridgeTruthViewKind::CommittedSnapshot | BridgeTruthViewKind::BranchSnapshot => {
                BridgeTruthViewPolicyResolution::Admitted(ResolvedTruthViewPolicy::admitted(
                    declaration,
                    TruthViewRetentionAdmission::SnapshotResident,
                    TruthViewSourceCapability::DirectSnapshotRead,
                    match declaration.replay_mode() {
                        BridgeReplayMode::Required => TruthViewReplayContinuity::ReplayRequired,
                        BridgeReplayMode::Disabled | BridgeReplayMode::Enabled => {
                            TruthViewReplayContinuity::ReplayPermitted
                        }
                    },
                ))
            }
            BridgeTruthViewKind::HistoricalCommit | BridgeTruthViewKind::BranchCommit => {
                BridgeTruthViewPolicyResolution::Admitted(ResolvedTruthViewPolicy::admitted(
                    declaration,
                    TruthViewRetentionAdmission::HistoricalLookupRequired,
                    TruthViewSourceCapability::HistoricalLookupAndSnapshotRead,
                    match declaration.replay_mode() {
                        BridgeReplayMode::Required => TruthViewReplayContinuity::ReplayRequired,
                        BridgeReplayMode::Disabled | BridgeReplayMode::Enabled => {
                            TruthViewReplayContinuity::ReplayPermitted
                        }
                    },
                ))
            }
            BridgeTruthViewKind::BranchHead => {
                if self.truth_branch_head_source.is_none() {
                    return BridgeTruthViewPolicyResolution::Rejected(
                        BridgeTruthViewPolicyRejection::new(
                            declaration,
                            TruthViewPolicyRejectionKind::SourceCapabilityMismatch,
                            "branch-head truth views require a configured truth branch-head source",
                        ),
                    );
                }
                BridgeTruthViewPolicyResolution::Admitted(ResolvedTruthViewPolicy::admitted(
                    declaration,
                    TruthViewRetentionAdmission::HistoricalLookupRequired,
                    TruthViewSourceCapability::HistoricalLookupAndSnapshotRead,
                    match declaration.replay_mode() {
                        BridgeReplayMode::Required => TruthViewReplayContinuity::ReplayRequired,
                        BridgeReplayMode::Disabled | BridgeReplayMode::Enabled => {
                            TruthViewReplayContinuity::ReplayPermitted
                        }
                    },
                ))
            }
        }
    }
}

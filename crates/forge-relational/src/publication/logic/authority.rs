use crate::authority::commit::preparation::planning::strategy::{
    packet_width_is_profitable, MIN_PARALLEL_PACKET_WIDTH,
};
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId, HistoryShapeClassification, OrderedParentList};
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime, ReplaySchemaVersion};
use crate::publication::data::{PublicationBundle, PublicationStatus};
use crate::snapshots::data::{SnapshotHandle, SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PublicationArtifacts;
use rayon::prelude::*;
use serde_json::json;

use super::diagnostics::DiagnosticArtifactBuilder;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestPostCommitFault {
    ConsumerFailureNonAuthoritative,
}

#[cfg(test)]
static TEST_POST_COMMIT_FAULT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(test)]
fn current_test_post_commit_fault() -> Option<TestPostCommitFault> {
    match TEST_POST_COMMIT_FAULT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => Some(TestPostCommitFault::ConsumerFailureNonAuthoritative),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn with_test_post_commit_fault<T>(
    fault: TestPostCommitFault,
    run: impl FnOnce() -> T,
) -> T {
    struct ResetGuard<'a> {
        fault: &'a std::sync::atomic::AtomicU8,
        _lock: std::sync::MutexGuard<'a, ()>,
    }

    impl Drop for ResetGuard<'_> {
        fn drop(&mut self) {
            self.fault.store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let guard = crate::testing::fault_injection_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _reset = ResetGuard {
        fault: &TEST_POST_COMMIT_FAULT,
        _lock: guard,
    };
    TEST_POST_COMMIT_FAULT.store(
        match fault {
            TestPostCommitFault::ConsumerFailureNonAuthoritative => 1,
        },
        std::sync::atomic::Ordering::SeqCst,
    );
    run()
}

pub struct PublicationAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn publication_authority(&mut self) -> PublicationAuthority<'_> {
        PublicationAuthority::new(self)
    }
}

impl<'runtime> PublicationAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn push_diagnostic_artifact(
        &mut self,
        artifact: crate::diagnostics::data::RelationalDiagnosticArtifact,
    ) {
        if let Some(filtered) = self.runtime.config.diagnostics.profile.filter_artifact(artifact) {
            self.runtime.publication.diagnostics.push(filtered);
        }
    }

    pub(crate) fn prune_published_snapshot_handles_if_needed(&mut self) {
        let limit = self
            .runtime
            .config
            .publication
            .policy
            .max_published_snapshot_handles
            .max(1);
        while self.runtime.visibility.published_snapshot_handle_count() > limit {
            let Some(oldest_snapshot_id) = self.runtime.visibility.oldest_published_snapshot_id()
            else {
                break;
            };
            let _ = self
                .runtime
                .visibility
                .remove_published_handle(oldest_snapshot_id);
        }
    }

    pub(crate) fn push_bounded_diagnostic(
        &mut self,
        scope: crate::diagnostics::data::DiagnosticsScope,
        kind: crate::diagnostics::data::DiagnosticsArtifactKind,
        entries: Vec<crate::diagnostics::data::RelationalDiagnosticsEntry>,
    ) -> crate::diagnostics::data::RelationalDiagnosticArtifact {
        let artifact = crate::diagnostics::data::RelationalDiagnosticArtifact {
            scope,
            kind,
            determinism: crate::diagnostics::data::DeterminismExpectation::Required,
            entries,
        };
        let filtered = self
            .runtime
            .config
            .diagnostics
            .profile
            .filter_artifact(artifact.clone())
            .unwrap_or_else(|| crate::diagnostics::data::RelationalDiagnosticArtifact {
                scope: artifact.scope,
                kind: artifact.kind,
                determinism: artifact.determinism,
                entries: Vec::new(),
            });
        if !filtered.entries.is_empty() {
            self.runtime.publication.diagnostics.push(filtered.clone());
        }
        filtered
    }

    pub(crate) fn diagnostic(
        self,
        scope: crate::diagnostics::data::DiagnosticsScope,
    ) -> DiagnosticArtifactBuilder<'runtime> {
        DiagnosticArtifactBuilder::new(self.runtime, scope)
    }

    pub(crate) fn assemble_publication_bundle(
        &mut self,
        commit_reference: crate::history::data::CommitReference,
        version_id: crate::identity::data::VersionId,
        patch: crate::publication::data::diff::RelationalPatchRecord,
        diagnostics_summary: crate::diagnostics::data::RelationalDiagnosticArtifact,
    ) -> PublicationArtifacts {
        let snapshot_id = self.runtime.visibility.allocate_snapshot_id();
        let snapshot = SnapshotHandle {
            runtime_instance_id: self.runtime.runtime_instance_id(),
            snapshot_id,
            version_id,
            read_policy: SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        let replay = RelationalReplayRecord {
            schema_version: ReplaySchemaVersion(1),
            commit_id: commit_reference.commit_id,
            version_id,
            snapshot_id,
            patch: patch.clone(),
            schema_registry: self.runtime.config.schema.registry.clone(),
        };
        let bundle = PublicationBundle {
            commit: commit_reference,
            snapshot,
            diagnostics_summary,
            patch,
            replay,
            status: PublicationStatus::Published,
        };
        PublicationArtifacts { bundle }
    }

    pub(crate) fn publish_artifacts(
        &mut self,
        version_id: crate::identity::data::VersionId,
        artifacts: PublicationArtifacts,
    ) -> SnapshotId {
        let PublicationArtifacts { bundle } = artifacts;
        let snapshot_id = bundle.snapshot.snapshot_id;
        self.runtime
            .visibility
            .insert_published_handle(
                snapshot_id,
                crate::logic::runtime::SnapshotHandleBinding {
                    version_id,
                    read_policy: bundle.snapshot.read_policy,
                },
            );
        self.push_diagnostic_artifact(bundle.diagnostics_summary.clone());
        self.runtime.publication.replace_latest_bundle(bundle);
        self.prune_published_snapshot_handles_if_needed();
        snapshot_id
    }

    pub(crate) fn consume_post_commit_artifacts(
        &mut self,
        commit_id: CommitId,
        snapshot_id: SnapshotId,
        branch_id: BranchId,
        parents: &[CommitId],
        merge_parent_branches: &[BranchId],
        merge_base_commits: &[CommitId],
    ) {
        use crate::authority::commit::preparation::packets::post_commit::{
            PostCommitConsumerKind, PostCommitConsumerPacket,
        };
        use crate::authority::commit::preparation::reduction::keys::PostCommitReductionKey;
        const PACKET_COUNT: usize = 2;
        self.runtime
            .performance_access()
            .count_post_commit_consumer_shape(
                PACKET_COUNT,
                PACKET_COUNT,
                1,
                1,
            );

        let should_parallelize =
            matches!(
                self.runtime.config.execution.execution_model,
                RelationalExecutionModel::ParallelPostCommitConsumption
            ) && packet_width_is_profitable(PACKET_COUNT, MIN_PARALLEL_PACKET_WIDTH);

        if should_parallelize {
            self.runtime
                .performance_access()
                .count_post_commit_parallel_strategy();
        } else {
            self.runtime
                .performance_access()
                .count_post_commit_serial_strategy();
        }

        let limit = self
            .runtime
            .config
            .publication
            .policy
            .max_published_snapshot_handles
            .max(1);
        let published_count = self.runtime.visibility.published_snapshot_handle_count();
        let prune_count = published_count.saturating_sub(limit);

        if !should_parallelize {
            let entries = build_publication_diagnostic_entries(
                commit_id,
                snapshot_id,
                &branch_id,
                parents,
                merge_parent_branches,
                merge_base_commits,
            );
            self.push_bounded_diagnostic(
                DiagnosticsScope::PatchPublication,
                DiagnosticsArtifactKind::MinimalSummary,
                entries,
            );

            if prune_count != 0 {
                for snapshot_id in self.runtime.visibility.oldest_published_snapshot_ids(prune_count) {
                    let _ = self.runtime.visibility.remove_published_handle(snapshot_id);
                }
            }
            return;
        }

        let prune_ids = self
            .runtime
            .visibility
            .oldest_published_snapshot_ids(prune_count);
        let packets = [
            PostCommitConsumerPacket {
                packet_index: 0,
                kind: PostCommitConsumerKind::PublicationDiagnostic,
                reduction_key: PostCommitReductionKey::new(0, 0),
            },
            PostCommitConsumerPacket {
                packet_index: 1,
                kind: PostCommitConsumerKind::PublishedHandlePrunePlan,
                reduction_key: PostCommitReductionKey::new(1, 1),
            },
        ];
        let observation_streams = {
            packets
                .par_iter()
                .map(|packet| {
                    OrderedReductionStream::singleton(
                        packet.reduction_key.clone(),
                        post_commit_observation(
                            packet.kind,
                            commit_id,
                            snapshot_id,
                            &branch_id,
                            parents,
                            merge_parent_branches,
                            merge_base_commits,
                            &prune_ids,
                        ),
                    )
                })
                .collect::<Vec<_>>()
        };

        for (_key, observation) in canonical_merge_streams(observation_streams) {
            self.apply_post_commit_observation(observation);
        }
    }

    fn apply_post_commit_observation(
        &mut self,
        observation: crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerObservation,
    ) {
        use crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerObservation;

        match observation {
            PostCommitConsumerObservation::PublicationDiagnosticEntries(entries) => {
                self.push_bounded_diagnostic(
                    DiagnosticsScope::PatchPublication,
                    DiagnosticsArtifactKind::MinimalSummary,
                    entries,
                );
            }
            PostCommitConsumerObservation::PublishedHandlePrunePlan(snapshot_ids) => {
                for snapshot_id in snapshot_ids {
                    let _ = self.runtime.visibility.remove_published_handle(snapshot_id);
                }
            }
        }
    }
}

fn post_commit_observation(
    kind: crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerKind,
    commit_id: CommitId,
    snapshot_id: SnapshotId,
    branch_id: &BranchId,
    parents: &[CommitId],
    merge_parent_branches: &[BranchId],
    merge_base_commits: &[CommitId],
    prune_ids: &[SnapshotId],
) -> crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerObservation {
    use crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerObservation;

    match kind {
        crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerKind::PublicationDiagnostic => {
            PostCommitConsumerObservation::PublicationDiagnosticEntries(
                build_publication_diagnostic_entries(
                    commit_id,
                    snapshot_id,
                    branch_id,
                    parents,
                    merge_parent_branches,
                    merge_base_commits,
                ),
            )
        }
        crate::authority::commit::preparation::packets::post_commit::PostCommitConsumerKind::PublishedHandlePrunePlan => {
            PostCommitConsumerObservation::PublishedHandlePrunePlan(prune_ids.to_vec())
        }
    }
}

fn build_publication_diagnostic_entries(
    commit_id: CommitId,
    snapshot_id: SnapshotId,
    branch_id: &BranchId,
    parents: &[CommitId],
    merge_parent_branches: &[BranchId],
    merge_base_commits: &[CommitId],
) -> Vec<RelationalDiagnosticsEntry> {
    let authoritative_parent_list = OrderedParentList::from_authoritative(parents.to_vec());
    let history_shape = authoritative_parent_list.history_shape_classification();
    let authoritative_parent_ids = authoritative_parent_list
        .as_slice()
        .iter()
        .map(|parent| parent.0)
        .collect::<Vec<_>>();
    let merge_base_ids = merge_base_commits
        .iter()
        .map(|base| base.0)
        .collect::<Vec<_>>();
    let merge_parent_branch_ids = merge_parent_branches
        .iter()
        .map(|branch| branch.0.clone())
        .collect::<Vec<_>>();
    let publication_code = if history_shape == HistoryShapeClassification::MergeReady {
        DiagnosticCode::MergeCommitPublished
    } else {
        DiagnosticCode::CommitPublished
    };
    let mut entries = Vec::new();
    if history_shape == HistoryShapeClassification::MergeReady {
        entries.push(RelationalDiagnosticsEntry {
            code: DiagnosticCode::MergeBaseResolved,
            message: "ancestry-derived merge-base result resolved deterministically".to_string(),
            fields: json!({
                "commit_id": commit_id.0,
                "history_shape": format!("{:?}", history_shape),
                "authoritative_parent_list": authoritative_parent_ids,
                "merge_base_commit_ids": merge_base_ids,
            }),
        });
    }
    entries.push(RelationalDiagnosticsEntry {
        code: publication_code,
        message: if history_shape == HistoryShapeClassification::MergeReady {
            "merge-ready history commit published coherently".to_string()
        } else {
            "commit published coherently".to_string()
        },
        fields: json!({
            "commit_id": commit_id.0,
            "snapshot_id": snapshot_id.0,
            "branch_id": branch_id.0,
            "history_shape": format!("{:?}", history_shape),
            "parent_count": authoritative_parent_list.len(),
            "authoritative_parent_list": authoritative_parent_ids,
            "merge_parent_branches": merge_parent_branch_ids,
            "merge_base_commit_ids": merge_base_ids,
        }),
    });
    #[cfg(test)]
    if matches!(
        current_test_post_commit_fault(),
        Some(TestPostCommitFault::ConsumerFailureNonAuthoritative)
    ) {
        entries.push(RelationalDiagnosticsEntry {
            code: DiagnosticCode::PreparationFailure,
            message: "post-commit consumer failed without affecting publication".to_string(),
            fields: json!({
                "failure_class": "consumer_failure_non_authoritative",
                "commit_id": commit_id.0,
                "snapshot_id": snapshot_id.0,
            }),
        });
    }
    entries
}

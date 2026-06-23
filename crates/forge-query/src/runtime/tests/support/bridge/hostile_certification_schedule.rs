use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::projection_consumption::ProjectionFactConsumptionAttempt;
use crate::runtime::tests::support::*;
use forge_foundational::facade::InternedString;

#[derive(Clone)]
struct HostileCertificationMaintainer {
    invocations: Arc<AtomicUsize>,
    titles: &'static [&'static str],
}

impl ForgeQueryDerivedViewMaintainer for HostileCertificationMaintainer {
    fn maintain(
        &mut self,
        view: &crate::program::ForgeQueryDerivedView,
        _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let next = self.invocations.fetch_add(1, Ordering::SeqCst);
        let title = self
            .titles
            .get(next)
            .copied()
            .unwrap_or(self.titles[self.titles.len() - 1]);
        let retained_row = retained_string_test_row("title.value", title);
        materialization.replace_retained_rows([retained_row.clone()]);
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            crate::memory_workspace::admit_external_commit_label(format!(
                "hostile-certification-refresh-{}",
                next + 1
            )),
            [test_aspect_touch("title.value")],
            ForgeQueryDerivedPatchPayload::from_retained_row(retained_row),
            format!("hostile-certification-publication-{}", next + 1),
        )
    }
}

#[derive(Clone, Copy)]
struct HostileSchedule {
    steps: &'static [HostileScheduleStep],
}

#[derive(Clone, Copy)]
enum HostileScheduleStep {
    ConsumeUnpublishedDerived,
    OpenBranch(&'static str),
    DiscardPreview {
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    },
    SubmitTask {
        identity: &'static str,
        title: &'static str,
        slot: PublishedArtifactSlot,
    },
    ReconsumePublishedArtifacts {
        current_slot: PublishedArtifactSlot,
        stable_slot: Option<PublishedArtifactSlot>,
    },
    PromotePreview {
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    },
}

#[derive(Clone, Copy)]
enum PublishedArtifactSlot {
    First,
    Second,
    Third,
}

struct HostileExecutionState {
    workspace: ForgeQueryWorkspace,
    derived: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    invocations: Arc<AtomicUsize>,
    receipt_summaries: Vec<String>,
    reader_results: Vec<String>,
    published_artifacts: Vec<String>,
    preview_closeouts: Vec<String>,
    branch_bases: Vec<String>,
    authoritative_receipts: Vec<ForgeQueryWriteReceipt>,
    reader_derived_evaluation_count: usize,
    delivery_residue_count: usize,
    first_artifact: Option<ForgeQueryPublishedDerivedArtifactHandle>,
    second_artifact: Option<ForgeQueryPublishedDerivedArtifactHandle>,
    third_artifact: Option<ForgeQueryPublishedDerivedArtifactHandle>,
}

pub(in crate::runtime::tests) fn execute_runtime_hostile_schedule(
) -> RuntimeHostileCertificationArtifact {
    let schedule = hostile_schedule();
    run_hostile_schedule_steps(schedule.steps.iter().copied())
}

pub(in crate::runtime::tests) fn replay_runtime_hostile_schedule(
) -> RuntimeHostileCertificationArtifact {
    let schedule = hostile_schedule();
    let recorded_steps = schedule.steps.iter().copied().collect::<Vec<_>>();
    run_hostile_schedule_steps(recorded_steps)
}

fn hostile_schedule() -> HostileSchedule {
    HostileSchedule {
        steps: &[
            HostileScheduleStep::ConsumeUnpublishedDerived,
            HostileScheduleStep::OpenBranch("branch-a"),
            HostileScheduleStep::OpenBranch("branch-b"),
            HostileScheduleStep::DiscardPreview {
                label: "preview-discard",
                identity: "preview-discard",
                title: "Preview discard",
            },
            HostileScheduleStep::SubmitTask {
                identity: "task-1",
                title: "Task One",
                slot: PublishedArtifactSlot::First,
            },
            HostileScheduleStep::SubmitTask {
                identity: "task-2",
                title: "Task Two",
                slot: PublishedArtifactSlot::Second,
            },
            HostileScheduleStep::ReconsumePublishedArtifacts {
                current_slot: PublishedArtifactSlot::Second,
                stable_slot: Some(PublishedArtifactSlot::First),
            },
            HostileScheduleStep::PromotePreview {
                label: "preview-promote",
                identity: "task-3",
                title: "Task Three",
            },
            HostileScheduleStep::OpenBranch("branch-c"),
            HostileScheduleStep::ReconsumePublishedArtifacts {
                current_slot: PublishedArtifactSlot::Third,
                stable_slot: None,
            },
        ],
    }
}

fn run_hostile_schedule_steps(
    steps: impl IntoIterator<Item = HostileScheduleStep>,
) -> RuntimeHostileCertificationArtifact {
    let mut state = HostileExecutionState::new();
    for step in steps {
        state.apply(step);
    }
    state.finish()
}

impl HostileExecutionState {
    fn new() -> Self {
        let mut workspace = stateful_bridge_task_runtime()
            .workspace("runtime.tests.hostile-certification")
            .expect("workspace should build");
        let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
            .live_view_request(
                "tasks.hostile-certification",
                task_live_request(),
                task_schema(),
            )
            .expect("live view should declare");
        let invocations = Arc::new(AtomicUsize::new(0));
        let derived = workspace
            .computed_view(
                crate::program::ForgeQueryDerivedView::new(
                    "derived.hostile-certification",
                    [test_aspect_touch("title.value")],
                )
                .depends_on_live(&live),
                HostileCertificationMaintainer {
                    invocations: Arc::clone(&invocations),
                    titles: &["Task One", "Task Two", "Task Three"],
                },
            )
            .expect("derived view should declare");
        Self {
            workspace,
            derived,
            invocations,
            receipt_summaries: Vec::new(),
            reader_results: Vec::new(),
            published_artifacts: Vec::new(),
            preview_closeouts: Vec::new(),
            branch_bases: Vec::new(),
            authoritative_receipts: Vec::new(),
            reader_derived_evaluation_count: 0,
            delivery_residue_count: 0,
            first_artifact: None,
            second_artifact: None,
            third_artifact: None,
        }
    }

    fn apply(&mut self, step: HostileScheduleStep) {
        match step {
            HostileScheduleStep::ConsumeUnpublishedDerived => self.capture_unpublished_posture(),
            HostileScheduleStep::OpenBranch(label) => self.record_branch_basis(label),
            HostileScheduleStep::DiscardPreview {
                label,
                identity,
                title,
            } => self.record_preview_discard(label, identity, title),
            HostileScheduleStep::SubmitTask {
                identity,
                title,
                slot,
            } => self.record_submission_publication(identity, title, slot),
            HostileScheduleStep::ReconsumePublishedArtifacts {
                current_slot,
                stable_slot,
            } => self.reconsume_published_artifacts(current_slot, stable_slot),
            HostileScheduleStep::PromotePreview {
                label,
                identity,
                title,
            } => self.record_preview_promotion(label, identity, title),
        }
    }

    fn finish(mut self) -> RuntimeHostileCertificationArtifact {
        self.first_artifact = None;
        self.second_artifact = None;
        self.third_artifact = None;
        let counters = RuntimeHostileCertificationCounters::for_runtime(
            &self.workspace.runtime,
            hostile_journal_gap_count(&self.authoritative_receipts),
            self.reader_derived_evaluation_count,
            self.delivery_residue_count,
        );
        RuntimeHostileCertificationArtifact::new(
            self.receipt_summaries,
            self.reader_results,
            self.published_artifacts,
            self.preview_closeouts,
            self.branch_bases,
            counters,
        )
    }

    fn capture_unpublished_posture(&mut self) {
        let unpublished = self
            .workspace
            .shared_read_context()
            .expect("shared read context should mint")
            .published_derived_artifact(&self.derived)
            .expect("declared derived handle should mint");
        let pending_state = match hostile_consume_title_attempt(&unpublished) {
            ForgeQueryPublishedProjectionConsumption::ResultState(state) => state,
            other => panic!("expected pending published artifact posture, got {other:?}"),
        };
        self.reader_results
            .push(pending_state.result_state_for_reporting().to_string());
        self.published_artifacts
            .push(hostile_published_artifact_digest(&unpublished, None));
    }

    fn record_branch_basis(&mut self, label: &'static str) {
        let branch = self
            .workspace
            .branch(test_session_label(label))
            .expect("branch churn should admit");
        self.branch_bases.push(hostile_branch_basis_digest(&branch));
    }

    fn record_preview_discard(
        &mut self,
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    ) {
        let discarded = {
            let mut preview = self
                .workspace
                .preview(test_session_label(label))
                .expect("preview churn should admit");
            preview
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value(identity),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value(title),
                    )
                })
                .expect("preview staging should succeed");
            preview.discard()
        };
        self.delivery_residue_count += hostile_preview_delivery_residue_count(&discarded);
        self.preview_closeouts
            .push(hostile_preview_closeout_digest(&discarded));
    }

    fn record_submission_publication(
        &mut self,
        identity: &'static str,
        title: &'static str,
        slot: PublishedArtifactSlot,
    ) {
        let receipt = self
            .workspace
            .submissions()
            .expect("submission lane should mint")
            .submit(hostile_insert_task_command(identity, title))
            .expect("submission should succeed");
        self.receipt_summaries
            .push(hostile_write_receipt_digest(&receipt));
        self.authoritative_receipts.push(receipt);

        let artifact = self
            .workspace
            .shared_read_context()
            .expect("shared read context should mint")
            .published_derived_artifact(&self.derived)
            .expect("published artifact should mint");
        let consumed_title = self.consume_title_without_recompute(&artifact);
        self.reader_results.push(consumed_title.clone());
        self.published_artifacts
            .push(hostile_published_artifact_digest(
                &artifact,
                Some(consumed_title.as_str()),
            ));
        self.replace_artifact_slot(slot, artifact);
    }

    fn reconsume_published_artifacts(
        &mut self,
        current_slot: PublishedArtifactSlot,
        stable_slot: Option<PublishedArtifactSlot>,
    ) {
        if let Some(stable_slot) = stable_slot {
            let stable_artifact = self.artifact_slot(stable_slot).clone();
            let stable_title = self.consume_title_without_recompute(&stable_artifact);
            self.reader_results.push(stable_title);
        }
        let current_artifact = self.artifact_slot(current_slot).clone();
        let current_title = self.consume_title_without_recompute(&current_artifact);
        self.reader_results.push(current_title.clone());
        self.published_artifacts
            .push(hostile_published_artifact_digest(
                &current_artifact,
                Some(current_title.as_str()),
            ));
    }

    fn record_preview_promotion(
        &mut self,
        label: &'static str,
        identity: &'static str,
        title: &'static str,
    ) {
        let promoted = {
            let mut preview = self
                .workspace
                .preview(test_session_label(label))
                .expect("preview churn should admit");
            preview
                .insert("Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value(identity),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value(title),
                    )
                })
                .expect("preview promotion staging should succeed");
            preview.promote().expect("preview promotion should succeed")
        };
        self.delivery_residue_count += hostile_preview_delivery_residue_count(&promoted);
        self.preview_closeouts
            .push(hostile_preview_closeout_digest(&promoted));
        let promoted_artifact = self
            .workspace
            .shared_read_context()
            .expect("shared read context should mint")
            .published_derived_artifact(&self.derived)
            .expect("third published artifact should mint");
        self.replace_artifact_slot(PublishedArtifactSlot::Third, promoted_artifact);
    }

    fn artifact_slot(
        &self,
        slot: PublishedArtifactSlot,
    ) -> &ForgeQueryPublishedDerivedArtifactHandle {
        match slot {
            PublishedArtifactSlot::First => self.first_artifact.as_ref(),
            PublishedArtifactSlot::Second => self.second_artifact.as_ref(),
            PublishedArtifactSlot::Third => self.third_artifact.as_ref(),
        }
        .expect("requested published artifact slot should exist")
    }

    fn replace_artifact_slot(
        &mut self,
        slot: PublishedArtifactSlot,
        artifact: ForgeQueryPublishedDerivedArtifactHandle,
    ) {
        match slot {
            PublishedArtifactSlot::First => self.first_artifact = Some(artifact),
            PublishedArtifactSlot::Second => self.second_artifact = Some(artifact),
            PublishedArtifactSlot::Third => self.third_artifact = Some(artifact),
        }
    }

    fn consume_title_without_recompute(
        &mut self,
        artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    ) -> String {
        let before = self.invocations.load(Ordering::SeqCst);
        let title = match hostile_consume_title_attempt(artifact) {
            ForgeQueryPublishedProjectionConsumption::Current(
                ProjectionFactConsumptionAttempt::Admitted(completed),
            ) => completed
                .facts()
                .display_fields()
                .first()
                .and_then(|fact| match fact.value() {
                    AspectValue::String(InternedString::Raw(value)) => Some(value.as_str()),
                    AspectValue::String(InternedString::Symbol(_)) => None,
                    _ => None,
                })
                .expect("display title should be present")
                .to_string(),
            other => panic!("expected admitted published fact consumption, got {other:?}"),
        };
        let after = self.invocations.load(Ordering::SeqCst);
        self.reader_derived_evaluation_count += after.saturating_sub(before);
        title
    }
}

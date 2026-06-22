use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    compose_public_bridge_hostile_certification_digest,
    public_bridge_hostile_certification_evidence_label,
    public_bridge_hostile_published_artifact_component_digest, ForgeQueryAspectMutationBuilder,
    ForgeQueryAspectTouch, ForgeQueryCommitIdentity, ForgeQueryDerivedPatch,
    ForgeQueryDerivedPatchPayload, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView,
    ForgeQueryNativeRow, ForgeQueryPublishedDerivedArtifactHandle, ForgeQueryRetainedFieldPath,
    ForgeQueryRuntime, ForgeQueryRuntimeSupportProfile, ForgeQueryWorkspace,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt, PublicBridgeHostileCertificationComposeInput,
};
use forge_query::{
    ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    ForgeQueryPublicBridgeReaderLaneCertification, ForgeQueryPublicBridgeReaderLanePosture,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use super::reader_lane_honesty::{
    public_bridge_certification_inventory, public_bridge_direct_materialization_sabotage,
    PublicBridgePublishedProjectionReader, PublicBridgeReaderLaneHonestyArtifact,
};

#[derive(Clone)]
struct PublicHostileMaintainer {
    invocations: Arc<AtomicUsize>,
    titles: &'static [&'static str],
}

impl ForgeQueryDerivedViewMaintainer for PublicHostileMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        _delta: &forge_query::facade::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let next = self.invocations.fetch_add(1, Ordering::SeqCst);
        let title = self
            .titles
            .get(next)
            .copied()
            .unwrap_or(self.titles[self.titles.len() - 1]);
        let retained_scalar = (retained_field_path(["title", "value"]), text(title));
        materialization
            .replace_retained_scalar_row([retained_scalar.clone()])
            .expect("title row should retain scalar values");
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            ForgeQueryCommitIdentity::from_relational_commit_id((next + 1) as u64),
            [touch("title.value")],
            ForgeQueryDerivedPatchPayload::from_retained_scalar_values([retained_scalar])
                .expect("title patch payload should retain scalar values"),
            format!("public-hostile-publication-{}", next + 1),
        )
    }
}

pub type PublicBridgeHostileCertificationArtifact = PublicBridgeReaderLaneHonestyArtifact;

struct PublicBridgeHostileProjectionViews {
    derived: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    invocations: Arc<AtomicUsize>,
}

struct PublicBridgeHostileSubmissionRead {
    receipt: ForgeQueryWriteReceipt,
    title_read: ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    published_artifact_digest: String,
}

struct PublicBridgeHostilePreviewPromotionRead {
    promotion_digest: String,
    title_read: ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    published_artifact_digest: String,
}

pub fn certify_public_bridge_hostile_schedule(
    harness: &super::PublicBridgeRuntimeHarness,
    bootstrap_path: super::PublicBridgeRuntimeBootstrapPath,
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> PublicBridgeHostileCertificationArtifact {
    let mut workspace = runtime_for_bootstrap_path(harness, bootstrap_path, support_profile)
        .workspace("public.bridge.hostile-certification")
        .expect("runtime should open workspace");
    let projection_views = declare_public_bridge_hostile_projection(&mut workspace);
    let pending_digest =
        mint_pending_public_bridge_artifact_digest(&mut workspace, &projection_views.derived);
    let mut published_artifact_digests = vec![pending_digest.clone()];
    let mut projection_reads = Vec::new();

    let (branch_a_digest, branch_b_digest) = record_public_bridge_branch_churn(&mut workspace);
    let preview_discard_digest = discard_public_bridge_preview_churn(&mut workspace);

    let first = submit_task_and_consume_published_title(
        &mut workspace,
        &projection_views,
        "task-1",
        "Task One",
        "first submission should succeed",
        "published artifact should mint",
    );
    projection_reads.push(first.title_read.clone());
    published_artifact_digests.push(first.published_artifact_digest.clone());

    let second = submit_task_and_consume_published_title(
        &mut workspace,
        &projection_views,
        "task-2",
        "Task Two",
        "second submission should succeed",
        "second published artifact should mint",
    );
    projection_reads.push(second.title_read.clone());
    published_artifact_digests.push(second.published_artifact_digest.clone());

    let third = promote_preview_task_and_consume_published_title(&mut workspace, &projection_views);
    projection_reads.push(third.title_read.clone());
    published_artifact_digests.push(third.published_artifact_digest.clone());

    let digest = compose_public_bridge_hostile_certification_digest(
        PublicBridgeHostileCertificationComposeInput {
            pending_artifact: pending_digest,
            branch_basis_a: branch_a_digest,
            branch_basis_b: branch_b_digest,
            preview_discard: preview_discard_digest,
            receipt_one: first.receipt.commit_identity().evidence_identity(),
            title_one: first.title_read.consumed_title().to_string(),
            receipt_two: second.receipt.commit_identity().evidence_identity(),
            title_two: second.title_read.consumed_title().to_string(),
            preview_promote: third.promotion_digest.clone(),
            title_three: third.title_read.consumed_title().to_string(),
        },
    );
    let reader_lane =
        certify_public_bridge_reader_lane(projection_reads, published_artifact_digests);

    PublicBridgeReaderLaneHonestyArtifact::new(digest, reader_lane)
}

fn declare_public_bridge_hostile_projection(
    workspace: &mut ForgeQueryWorkspace,
) -> PublicBridgeHostileProjectionViews {
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("public.bridge.hostile-certification.tasks", |q| {
            q.from("Task")
                .select([
                    forge_query::facade::AspectFieldKey::new("identity", "id").unwrap(),
                    forge_query::facade::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(forge_query::facade::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("public-bridge-hostile-certification-tasks")
        })
        .expect("task live view should declare");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived = workspace
        .computed_view(
            ForgeQueryDerivedView::new(
                "public.bridge.hostile-certification.derived",
                [touch("title.value")],
            )
            .depends_on_live(&live),
            PublicHostileMaintainer {
                invocations: Arc::clone(&invocations),
                titles: &["Task One", "Task Two", "Task Three"],
            },
        )
        .expect("derived view should declare");
    PublicBridgeHostileProjectionViews {
        derived,
        invocations,
    }
}

fn mint_pending_public_bridge_artifact_digest(
    workspace: &mut ForgeQueryWorkspace,
    derived: &ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
) -> String {
    let pending = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(derived)
        .expect("declared derived handle should mint");
    published_artifact_digest(&pending, None)
}

fn record_public_bridge_branch_churn(workspace: &mut ForgeQueryWorkspace) -> (String, String) {
    let branch_a_digest = public_bridge_hostile_certification_evidence_label(
        workspace
            .branch(session_label("public-branch-a"))
            .expect("branch churn should admit")
            .basis_admission()
            .admission_digest(),
    );
    let branch_b_digest = public_bridge_hostile_certification_evidence_label(
        workspace
            .branch(session_label("public-branch-b"))
            .expect("branch churn should admit")
            .basis_admission()
            .admission_digest(),
    );
    (branch_a_digest, branch_b_digest)
}

fn discard_public_bridge_preview_churn(workspace: &mut ForgeQueryWorkspace) -> String {
    let mut preview = workspace
        .preview(session_label("public-preview-discard"))
        .expect("preview churn should admit");
    preview
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("preview-discard"))
                .aspect(touch("title.value"), text("Preview discard"))
        })
        .expect("preview staging should succeed");
    preview
        .discard()
        .closeout_evidence()
        .closeout_digest()
        .to_string()
}

fn submit_task_and_consume_published_title(
    workspace: &mut ForgeQueryWorkspace,
    projection_views: &PublicBridgeHostileProjectionViews,
    task_id: &str,
    task_title: &str,
    submission_expectation: &str,
    artifact_expectation: &str,
) -> PublicBridgeHostileSubmissionRead {
    let receipt = workspace
        .submissions()
        .expect("submission lane should mint")
        .submit(insert_task_command(task_id, task_title))
        .expect(submission_expectation);
    let artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&projection_views.derived)
        .expect(artifact_expectation);
    let title_read = consume_title(&artifact, &projection_views.invocations);
    let published_artifact_digest =
        published_artifact_digest(&artifact, Some(title_read.consumed_title()));
    PublicBridgeHostileSubmissionRead {
        receipt,
        title_read,
        published_artifact_digest,
    }
}

fn promote_preview_task_and_consume_published_title(
    workspace: &mut ForgeQueryWorkspace,
    projection_views: &PublicBridgeHostileProjectionViews,
) -> PublicBridgeHostilePreviewPromotionRead {
    let mut preview = workspace
        .preview(session_label("public-preview-promote"))
        .expect("preview churn should admit");
    preview
        .insert("Task", |task| {
            task.aspect(touch("identity.id"), text("task-3"))
                .aspect(touch("title.value"), text("Task Three"))
        })
        .expect("preview promotion staging should succeed");
    let promoted = preview.promote().expect("preview promotion should succeed");
    let artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&projection_views.derived)
        .expect("third published artifact should mint");
    let title_read = consume_title(&artifact, &projection_views.invocations);
    let published_artifact_digest =
        published_artifact_digest(&artifact, Some(title_read.consumed_title()));
    PublicBridgeHostilePreviewPromotionRead {
        promotion_digest: promoted.closeout_evidence().closeout_digest().to_string(),
        title_read,
        published_artifact_digest,
    }
}

fn certify_public_bridge_reader_lane(
    projection_reads: Vec<ForgeQueryPublicBridgeProjectionConsumptionEvidence>,
    published_artifact_digests: Vec<String>,
) -> ForgeQueryPublicBridgeReaderLaneCertification {
    let reader_lane = ForgeQueryPublicBridgeReaderLaneCertification::certify(
        projection_reads,
        published_artifact_digests,
        public_bridge_certification_inventory(),
        public_bridge_direct_materialization_sabotage(),
    );
    assert_eq!(
        reader_lane.posture(),
        ForgeQueryPublicBridgeReaderLanePosture::Closed
    );
    reader_lane
}

fn runtime_for_bootstrap_path(
    harness: &super::PublicBridgeRuntimeHarness,
    bootstrap_path: super::PublicBridgeRuntimeBootstrapPath,
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    match bootstrap_path {
        super::PublicBridgeRuntimeBootstrapPath::Common => {
            harness.bridge_backed_runtime_with_support(support_profile)
        }
        super::PublicBridgeRuntimeBootstrapPath::Builder => harness
            .bridge_backed_runtime_builder()
            .support_profile(support_profile)
            .build(),
    }
}

fn insert_task_command(id: &str, title: &str) -> ForgeQueryWriteCommand {
    ForgeQueryAspectMutationBuilder::new()
        .aspect(touch("identity.id"), text(id))
        .aspect(touch("title.value"), text(title))
        .build_insert("Task")
        .expect("insert command should build")
}

fn published_artifact_digest(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    title: Option<&str>,
) -> String {
    let inspection = artifact.inspect_projection_consumption();
    public_bridge_hostile_published_artifact_component_digest(
        &artifact.snapshot_identity().evidence_identity(),
        inspection
            .artifact_binding_for_reporting()
            .unwrap_or("none"),
        title.unwrap_or("none"),
    )
}

fn consume_title(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    invocations: &Arc<AtomicUsize>,
) -> ForgeQueryPublicBridgeProjectionConsumptionEvidence {
    PublicBridgePublishedProjectionReader::new(artifact).consume_title(invocations)
}

fn session_label(label: &str) -> forge_query::facade::ForgeQuerySessionLabel {
    forge_query::facade::ForgeQuerySessionLabel::scoped_strs(
        "forge-query-public-bridge-tests",
        [label],
    )
    .expect("public bridge test session label should build")
}

fn retained_field_path(
    fields: impl IntoIterator<Item = &'static str>,
) -> ForgeQueryRetainedFieldPath {
    let canonical = CanonicalFieldPath::new(
        fields
            .into_iter()
            .map(|field| FieldKey::new(field.to_string()).expect("field key should admit")),
    )
    .expect("retained field path should admit");
    ForgeQueryRetainedFieldPath::from_canonical_field_path(canonical)
}

fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_path.split('.');
    let aspect = segments
        .next()
        .and_then(|segment| AspectKey::new(segment.to_string()))
        .expect("test aspect path aspect should admit");
    let fields = segments
        .map(|segment| {
            FieldKey::new(segment.to_string()).expect("test aspect path field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::aspect(aspect)
    } else {
        ForgeQueryAspectTouch::field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test aspect path should have fields"),
        )
    }
}

fn text(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

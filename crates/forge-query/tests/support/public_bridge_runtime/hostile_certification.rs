use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryCommitIdentity, ForgeQueryDerivedPatch,
    ForgeQueryDerivedView, ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag, ForgeQueryLiveView, ForgeQueryPublishedDerivedArtifactHandle,
    ForgeQueryRuntime, ForgeQueryRuntimeSupportProfile, ForgeQueryWriteCommand,
};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
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
        materialization.replace_rows([json!({ "title": { "value": title } })]);
        ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            ForgeQueryCommitIdentity::from_external_authority_label(format!(
                "public-hostile-refresh-{}",
                next + 1
            )),
            ["title.value".to_string()],
            json!({ "published": true, "title": title }),
            format!("public-hostile-publication-{}", next + 1),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBridgeHostileCertificationArtifact {
    digest: ForgeQueryEvidenceIdentity,
}

impl PublicBridgeHostileCertificationArtifact {
    pub fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}

pub fn certify_public_bridge_hostile_schedule(
    harness: &super::PublicBridgeRuntimeHarness,
    bootstrap_path: super::PublicBridgeRuntimeBootstrapPath,
    support_profile: ForgeQueryRuntimeSupportProfile,
) -> PublicBridgeHostileCertificationArtifact {
    let mut workspace = runtime_for_bootstrap_path(harness, bootstrap_path, support_profile)
        .workspace("public.bridge.hostile-certification")
        .expect("runtime should open workspace");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("public.bridge.hostile-certification.tasks", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("public-bridge-hostile-certification-tasks")
        })
        .expect("task live view should declare");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed_view(
            ForgeQueryDerivedView::new(
                "public.bridge.hostile-certification.derived",
                ["title.value".to_string()],
            )
            .depends_on_live(&live),
            PublicHostileMaintainer {
                invocations: Arc::clone(&invocations),
                titles: &["Task One", "Task Two", "Task Three"],
            },
        )
        .expect("derived view should declare");

    let pending = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("declared derived handle should mint");
    let pending_digest = published_artifact_digest(&pending, None);

    let branch_a_digest = workspace
        .branch(session_label("public-branch-a"))
        .expect("branch churn should admit")
        .basis_admission()
        .admission_digest()
        .as_str()
        .to_string();
    let branch_b_digest = workspace
        .branch(session_label("public-branch-b"))
        .expect("branch churn should admit")
        .basis_admission()
        .admission_digest()
        .as_str()
        .to_string();

    let discarded = {
        let mut preview = workspace
            .preview(session_label("public-preview-discard"))
            .expect("preview churn should admit");
        preview
            .insert("Task", |task| {
                task.aspect("identity.id", "preview-discard")
                    .aspect("title.value", "Preview discard")
            })
            .expect("preview staging should succeed");
        preview.discard()
    };

    let first = workspace
        .submissions()
        .expect("submission lane should mint")
        .submit(insert_task_command("task-1", "Task One"))
        .expect("first submission should succeed");
    let first_artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("published artifact should mint");
    let first_title = consume_title(&first_artifact, &invocations);

    let second = workspace
        .submissions()
        .expect("submission lane should mint")
        .submit(insert_task_command("task-2", "Task Two"))
        .expect("second submission should succeed");
    let second_artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("second published artifact should mint");
    let second_title = consume_title(&second_artifact, &invocations);

    let promoted = {
        let mut preview = workspace
            .preview(session_label("public-preview-promote"))
            .expect("preview churn should admit");
        preview
            .insert("Task", |task| {
                task.aspect("identity.id", "task-3")
                    .aspect("title.value", "Task Three")
            })
            .expect("preview promotion staging should succeed");
        preview.promote().expect("preview promotion should succeed")
    };
    let third_artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("third published artifact should mint");
    let third_title = consume_title(&third_artifact, &invocations);

    let digest = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("pending_artifact"),
        &pending_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("branch_basis_a"),
        &branch_a_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("branch_basis_b"),
        &branch_b_digest,
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("preview_discard"),
        discarded.closeout_evidence().closeout_digest(),
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("receipt_one"),
        first.commit_identity().evidence_identity().as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("title_one"), &first_title)
    .field_identity(
        ForgeQueryEvidenceTag::new("receipt_two"),
        second.commit_identity().evidence_identity().as_str(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("title_two"), &second_title)
    .field_identity(
        ForgeQueryEvidenceTag::new("preview_promote"),
        promoted.closeout_evidence().closeout_digest(),
    )
    .field_shape(ForgeQueryEvidenceTag::new("title_three"), &third_title)
    .seal();

    PublicBridgeHostileCertificationArtifact { digest }
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
        .aspect("identity.id", id)
        .aspect("title.value", title)
        .build_insert("Task")
        .expect("insert command should build")
}

fn published_artifact_digest(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    title: Option<&str>,
) -> String {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("snapshot"),
        &artifact.snapshot_identity().evidence_identity(),
    )
    .field_identity(
        ForgeQueryEvidenceTag::new("binding"),
        artifact
            .published_binding()
            .map(|binding| binding.binding_digest())
            .unwrap_or("none"),
    )
    .field_shape(ForgeQueryEvidenceTag::new("title"), title.unwrap_or("none"))
    .seal()
    .as_str()
    .to_string()
}

fn consume_title(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
    invocations: &Arc<AtomicUsize>,
) -> String {
    let before = invocations.load(Ordering::SeqCst);
    let binding = artifact
        .published_binding()
        .expect("published artifact should carry a binding");
    let title = match binding
        .materialization_by_name(artifact.view_name())
        .expect("published materialization should be bound by view name")
        .rows()
        .first()
        .and_then(|row| row["title"]["value"].as_str())
    {
        Some(title) => title.to_string(),
        None => panic!("expected materialized title row"),
    };
    let after = invocations.load(Ordering::SeqCst);
    assert_eq!(after, before, "reader path must not trigger reevaluation");
    title
}

fn session_label(label: &str) -> forge_query::facade::ForgeQuerySessionLabel {
    forge_query::facade::ForgeQuerySessionLabel::scoped_strs(
        "forge-query-public-bridge-tests",
        [label],
    )
    .expect("public bridge test session label should build")
}

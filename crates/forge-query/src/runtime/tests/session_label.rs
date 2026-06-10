use super::support::*;
use crate::facade::runtime::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
    ForgeQuerySessionLabel,
};

fn session_entry_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .intent_authority(TestIntentAuthority)
        .support_profile(intent_support_profile())
        .build_backend_from_parts()
        .build()
        .expect("session-entry runtime should build")
}

#[test]
fn preview_and_branch_basis_admissions_record_canonical_session_label_identity() {
    let mut runtime = session_entry_runtime();
    let label = test_session_label("typed-session-entry");

    let preview = runtime
        .preview_with_options(
            label.clone(),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should admit typed label");
    let preview_manual =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::PreviewBasisAdmission)
            .field_identity(
                ForgeQueryEvidenceTag::new("session_label_identity"),
                label.identity_digest().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("effect_policy"),
                ForgeQueryEffectPolicy::SandboxedWriteIntent.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("authority_lane"),
                ForgeQueryAuthorityLane::PreviewTruth.as_str(),
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("evidence"),
                preview
                    .basis_admission()
                    .evidence()
                    .iter()
                    .map(String::as_str),
            )
            .seal();
    assert_eq!(preview.basis_admission().session_label(), &label);
    assert_eq!(
        preview.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(
        preview.basis_admission().admission_digest(),
        &preview_manual
    );
    drop(preview);

    let branch = runtime
        .branch_with_options(
            label.clone(),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch should admit the same typed label in its own family");
    let branch_manual =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::BranchBasisAdmission)
            .field_identity(
                ForgeQueryEvidenceTag::new("session_label_identity"),
                label.identity_digest().as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("effect_policy"),
                ForgeQueryEffectPolicy::SandboxedWriteIntent.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("authority_lane"),
                ForgeQueryAuthorityLane::BranchLocalTruth.as_str(),
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("evidence"),
                branch
                    .basis_admission()
                    .evidence()
                    .iter()
                    .map(String::as_str),
            )
            .seal();
    assert_eq!(branch.basis_admission().session_label(), &label);
    assert_eq!(
        branch.basis_admission().label_identity(),
        label.identity_digest()
    );
    assert_eq!(branch.basis_admission().admission_digest(), &branch_manual);
}

#[test]
fn equivalent_preview_session_label_replay_stops_with_typed_collision_class() {
    let mut runtime = session_entry_runtime();
    let label = test_session_label("preview-collision");

    runtime
        .preview(label.clone())
        .expect("first preview label admission should succeed")
        .discard();

    let error = match runtime.preview(label.clone()) {
        Ok(_) => panic!("re-admitting equivalent preview label should collide"),
        Err(error) => error,
    };

    match error.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane,
            label: collided,
        } => {
            assert_eq!(authority_lane, ForgeQueryAuthorityLane::PreviewTruth);
            assert_eq!(collided, &label);
        }
        other => panic!("expected typed session label collision, got {other:?}"),
    }
}

#[test]
fn session_label_collision_is_scoped_per_family_and_not_by_rendered_display() {
    let mut runtime = session_entry_runtime();
    let preview_label =
        ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let render_collision =
        ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    runtime
        .preview(preview_label.clone())
        .expect("first preview label should admit")
        .discard();
    runtime
        .preview(render_collision.clone())
        .expect("display-colliding but identity-distinct preview label should admit")
        .discard();
    runtime
        .branch(preview_label.clone())
        .expect("branch family should admit same identity independently");

    let error = match runtime.branch(preview_label.clone()) {
        Ok(_) => panic!("re-admitting the same branch label should collide"),
        Err(error) => error,
    };
    match error.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane,
            label,
        } => {
            assert_eq!(authority_lane, ForgeQueryAuthorityLane::BranchLocalTruth);
            assert_eq!(label, &preview_label);
            assert_ne!(
                preview_label.identity_digest(),
                render_collision.identity_digest()
            );
            assert_eq!(preview_label.display(), render_collision.display());
        }
        other => panic!("expected branch session label collision, got {other:?}"),
    }
}

#[test]
fn display_colliding_preview_labels_produce_distinct_closeout_digests() {
    let mut runtime = session_entry_runtime();
    let left = ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let right = ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");

    let left_outcome = runtime
        .preview(left.clone())
        .expect("left preview should admit")
        .discard();
    let right_outcome = runtime
        .preview(right.clone())
        .expect("right preview should admit")
        .discard();

    assert_eq!(left.display(), right.display());
    assert_ne!(left.identity_digest(), right.identity_digest());
    assert_ne!(
        left_outcome.closeout_evidence().closeout_digest(),
        right_outcome.closeout_evidence().closeout_digest(),
        "preview closeout evidence must preserve canonical label identity"
    );
}

#[test]
fn ordinary_runtime_entrypoints_require_typed_session_labels() {
    let runtime_sessions = include_str!("../runtime_sessions.rs");
    let workspace = include_str!("../workspace.rs");

    for required_signature in [
        "pub fn preview<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
        "pub fn branch<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
        "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
        "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: ForgeQuerySessionLabel,",
    ] {
        assert!(
            runtime_sessions.contains(required_signature)
                || workspace.contains(required_signature),
            "ordinary runtime session entrypoint must require ForgeQuerySessionLabel: {required_signature}"
        );
    }

    for forbidden_signature in [
        "pub fn preview<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
        "pub fn branch<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
        "pub fn preview_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
        "pub fn branch_with_options<'a>(\n        &'a mut self,\n        label: impl Into<String>,",
    ] {
        assert!(
            !runtime_sessions.contains(forbidden_signature) && !workspace.contains(forbidden_signature),
            "raw-string ordinary session entrypoint survived: {forbidden_signature}"
        );
    }
}

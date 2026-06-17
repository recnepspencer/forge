use super::super::intent::PrimitiveConstructionIntent;
use super::super::request::PrimitiveConstructionFamily;
use super::super::specs::{RegularPrismSpec, RegularPyramidSpec};
use super::support::branch_preview_basis::prepare_branch_preview_basis_report;
use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryPreviewOptions,
    ForgeQuerySessionLabel, ForgeQueryStopClass,
};
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[test]
fn query_basis_preview_parity_report_tracks_preview_and_branch_lane_alignment() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-parity".to_string(),
    )
    .expect("workspace");
    let report = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("basis preview parity report");

    assert_eq!(report.family(), PrimitiveConstructionFamily::RegularPyramid);
    assert!(report.parity_verified());
    assert_eq!(
        report.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        report.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert!(!report.branch_preview_contract_digest().is_empty());
    assert!(!report.preview_admission_digest().is_empty());
    assert!(!report.branch_admission_digest().is_empty());
    assert!(!report.report_digest().is_empty());
}

#[test]
fn query_basis_preview_admission_digests_match_live_query_identities() {
    let intent = PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
        sides: 5,
        radius: 1.0,
        height: 2.0,
    });
    let family = PrimitiveConstructionFamily::RegularPyramid;
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-admission-parity".to_string(),
    )
    .expect("workspace");
    let report = prepare_branch_preview_basis_report(&mut workspace, intent)
        .expect("basis preview parity report");

    let live_runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut live_workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(live_runtime),
        "worth-kernel.query-basis-admission-parity-live".to_string(),
    )
    .expect("live workspace");
    let preview_live = {
        let preview = live_workspace
            .preview_with_options(
                ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "preview"])
                    .expect("preview label"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview should admit");
        preview
            .basis_admission()
            .admission_identity()
            .terminal_projection_for_reporting()
            .to_string()
    };
    let branch_live = {
        let branch = live_workspace
            .branch_with_options(
                ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "branch"])
                    .expect("branch label"),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .expect("branch should admit");
        branch
            .basis_admission()
            .admission_identity()
            .terminal_projection_for_reporting()
            .to_string()
    };

    assert_eq!(report.preview_admission_digest(), preview_live);
    assert_eq!(report.branch_admission_digest(), branch_live);
    assert!(
        preview_live.starts_with("forge.query.evidence-identity.v1:"),
        "preview admission digest should be evidence-identity labeled"
    );
    assert!(
        branch_live.starts_with("forge.query.evidence-identity.v1:"),
        "branch admission digest should be evidence-identity labeled"
    );
    assert_ne!(preview_live, branch_live);
}

#[test]
fn query_basis_preview_parity_report_changes_digest_when_request_family_changes() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-report-digest-drift".to_string(),
    )
    .expect("workspace");
    let prism = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("prism basis report");
    let pyramid = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 5,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("pyramid basis report");

    assert_ne!(prism.report_digest(), pyramid.report_digest());
}

#[test]
fn query_basis_preview_parity_report_preserves_escalated_realization_truth() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-realization-truth".to_string(),
    )
    .expect("workspace");
    let report = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0e-200,
            height: 1.0e-200,
        }),
    )
    .expect("basis preview parity report");

    assert_eq!(
        report.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        report.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        report.stability_class(),
        Some(PrimitiveStabilityClass::StableAfterEscalation)
    );
    assert_eq!(
        report.support_normal_class(),
        Some(PrimitiveSupportNormalClass::Degenerate)
    );
    assert_eq!(
        report.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
    );
    assert_eq!(report.exhaustion_reason(), None);
}

#[test]
fn query_basis_preview_parity_report_preserves_world_collapsed_salvage_truth() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-exhaustion-truth".to_string(),
    )
    .expect("workspace");
    let report = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
            sides: 3,
            radius: 1.0,
            height: 1.0,
        })
        .at([1.0e308, 1.0e308, 1.0e308]),
    )
    .expect("basis preview parity report");

    assert_eq!(
        report.realization_strategy(),
        Some(PrimitiveRealizationStrategy::DirectWorld)
    );
    assert_eq!(
        report.attempted_realization_strategies(),
        &[PrimitiveRealizationStrategy::DirectWorld]
    );
    assert_eq!(
        report.stability_class(),
        Some(PrimitiveStabilityClass::StableDirect)
    );
    assert_eq!(
        report.normalization_disposition(),
        Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
    );
    assert_eq!(report.exhaustion_reason(), None);
}

#[test]
fn query_basis_preview_session_label_collision_uses_typed_stop_class() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-stop-class".to_string(),
    )
    .expect("workspace");
    let label = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["stop-class", "preview"])
        .expect("session label");

    {
        workspace
            .preview_with_options(
                label.clone(),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("first preview should admit");
    }
    let collision = match workspace.preview_with_options(
        label.clone(),
        ForgeQueryPreviewOptions::sandboxed_write_intent(),
    ) {
        Err(error) => error,
        Ok(_) => panic!("preview replay should collide"),
    };

    match collision.stop_class() {
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
fn query_basis_branch_session_label_collision_uses_typed_stop_class() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.query-basis-branch-stop-class".to_string(),
    )
    .expect("workspace");
    let label = ForgeQuerySessionLabel::scoped_strs("worth-kernel", ["stop-class", "branch"])
        .expect("session label");

    {
        workspace
            .branch_with_options(
                label.clone(),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .expect("first branch should admit");
    }
    let collision = match workspace.branch_with_options(
        label.clone(),
        ForgeQueryBranchOptions::sandboxed_write_intent(),
    ) {
        Err(error) => error,
        Ok(_) => panic!("branch replay should collide"),
    };

    match collision.stop_class() {
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane,
            label: collided,
        } => {
            assert_eq!(authority_lane, ForgeQueryAuthorityLane::BranchLocalTruth);
            assert_eq!(collided, &label);
        }
        other => panic!("expected typed session label collision, got {other:?}"),
    }
}

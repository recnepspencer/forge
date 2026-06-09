use super::super::intent::PrimitiveConstructionIntent;
use super::super::request::PrimitiveConstructionFamily;
use super::super::specs::{RegularPrismSpec, RegularPyramidSpec};
use super::support::branch_preview_basis::prepare_branch_preview_basis_report;
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

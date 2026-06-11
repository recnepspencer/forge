use super::super::intent::PrimitiveConstructionIntent;
use super::super::request::PrimitiveConstructionFamily;
use super::super::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, WireBodySpec,
};
use super::support::branch_preview_basis::prepare_branch_preview_basis_report;
use super::support::projection_consumption::prepare_primitive_construction_query_projection_consumption_surface_digest;
use super::support::runtime_truth::{
    prepare_primitive_construction_certification_runtime_truth,
    PrimitiveConstructionCertificationRuntimeTruth,
};
use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryPreviewOptions,
    ForgeQueryRuntimeFacadeFamily,
};
use topology::certification::milestone_one_runtime_builder;
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryReadSurface,
};
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

#[test]
fn branch_preview_query_sessions_open_preview_and_branch_lanes() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.branch-preview".to_string(),
    )
    .expect("workspace");
    let family = PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
        half_extents: [1.0, 2.0, 3.0],
    })
    .family();
    let contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .expect("branch preview contract")
        .contract_digest()
        .to_string();
    let (preview_lane, preview_evidence_count) = {
        let preview = workspace
            .preview_with_options(
                format!("worth-kernel.{}.preview", family.as_str()),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .expect("preview session");
        (
            preview.basis_admission().authority_lane(),
            preview.basis_admission().evidence().len(),
        )
    };
    let branch = workspace
        .branch_with_options(
            format!("worth-kernel.{}.branch", family.as_str()),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("branch session");

    assert_eq!(preview_lane, ForgeQueryAuthorityLane::PreviewTruth);
    assert_eq!(
        branch.basis_admission().authority_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert!(!contract_digest.is_empty());
    assert!(preview_evidence_count > 0);
    assert!(!branch.basis_admission().evidence().is_empty());
}

#[test]
fn replay_and_branch_preview_reports_cover_accepted_and_rejected_workflows() {
    let replay_request = PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
        sides: 5,
        radius: 1.0,
        height: 2.0,
    })
    .into_request();
    let runtime_truth =
        prepare_primitive_construction_certification_runtime_truth(replay_request.clone());
    let replay_truth = prepare_primitive_construction_certification_runtime_truth(replay_request);
    let runtime_family = runtime_truth.family();
    let runtime_admitted = matches!(
        runtime_truth,
        PrimitiveConstructionCertificationRuntimeTruth::Admitted(_)
    );
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five.branch-preview".to_string(),
    )
    .expect("workspace");
    let branch = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 2,
            hole_loop_edge_counts: vec![3],
        }),
    )
    .expect("query basis preview parity");

    assert_eq!(runtime_truth, replay_truth);
    assert!(runtime_admitted);
    assert!(branch.parity_verified());
    assert_eq!(runtime_family, PrimitiveConstructionFamily::RegularPyramid);
    assert_eq!(branch.family(), PrimitiveConstructionFamily::ShellWithHole);
}

#[test]
fn query_and_diagnostic_reports_cover_phase_five_runtime_and_rejection_surfaces() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five.query-and-diagnostics".to_string(),
    )
    .expect("workspace");
    let basis = prepare_branch_preview_basis_report(
        &mut workspace,
        PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        }),
    )
    .expect("basis report");
    let projection_digest =
        prepare_primitive_construction_query_projection_consumption_surface_digest(
            &mut workspace,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            }),
        )
        .expect("projection digest");
    let runtime_truths = [
        prepare_primitive_construction_certification_runtime_truth(
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            })
            .into_request(),
        ),
        prepare_primitive_construction_certification_runtime_truth(
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }).into_request(),
        ),
        prepare_primitive_construction_certification_runtime_truth(
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            })
            .into_request(),
        ),
    ];
    let accepted_count = runtime_truths
        .iter()
        .filter(|truth| {
            matches!(
                truth,
                PrimitiveConstructionCertificationRuntimeTruth::Admitted(_)
            )
        })
        .count();
    let rejected = runtime_truths
        .iter()
        .find_map(|truth| match truth {
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => Some(rejected),
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
        })
        .expect("one rejected runtime truth");
    let projection_truth = runtime_truths
        .iter()
        .find_map(|truth| match truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome)
                if outcome.family() == PrimitiveConstructionFamily::ShellWithHole =>
            {
                Some(outcome)
            }
            _ => None,
        })
        .expect("shell_with_hole admitted runtime truth");

    assert!(basis.parity_verified());
    assert!(!projection_digest.is_empty());
    assert_eq!(
        projection_truth.read_surface(),
        TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
    );
    assert_eq!(
        projection_truth.inspection_surface(),
        TopologyConstructionQueryInspectionSurface::InspectReceipt
    );
    assert_eq!(
        projection_truth.fact_provenance(),
        TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
    );
    assert_eq!(accepted_count, 2);
    assert_eq!(rejected.family(), PrimitiveConstructionFamily::WireBody);
    assert_eq!(rejected.rejection_class().as_str(), "invalid_request");
    assert_eq!(rejected.rejection_locality().as_str(), "admission");
}

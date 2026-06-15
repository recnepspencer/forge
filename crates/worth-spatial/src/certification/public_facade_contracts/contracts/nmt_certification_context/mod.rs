use topology::facade::{NmtTopologyScopeKind, NmtTopologyScopeSet};
use worth_kernel::workload_composition::{GrazingBasketStackSpec, WorkloadCatalog};
use worth_spatial::facade::nmt_certification_context::{
    NmtBossCloseoutDenial, NmtBossCloseoutReceipt, NmtBossId, NmtBossOutcomeMatrixEvidence,
    NmtCertificationDenialKind, NmtCertifiedScopeSet,
};
use worth_spatial::facade::user_response::WorthUserOutcomeKind;

use super::public_api_planar_overlap::metaboss::{
    mixed_surface_kill_box::subject::mixed_surface_closeout_evidence,
    nmt_radial_fan::subject::radial_fan_closeout_evidence,
};

#[test]
fn nmt_scope_context_compiles_scope_local_projection_and_replay_authority() {
    run_with_real_workload_stack(|| {
        let built = WorkloadCatalog::grazing_open_shell_basket_stack(
            GrazingBasketStackSpec::new().layers(6).strips_per_layer(12),
        )
        .declared("scope-local NMT authority proof")
        .build()
        .expect("NMT basket stack catalog must build");
        let topology = built
            .topology_construction()
            .expect("basket catalog exposes topology construction");
        let scopes = NmtTopologyScopeSet::from_construction(topology)
            .expect("topology construction produces layer scopes");

        let certified = NmtCertifiedScopeSet::from_platform_evidence(
            topology,
            built.workload().evidence_ledger(),
            built.bound_geometry(),
            built.projected_workload(),
            built.transform_receipts(),
            built
                .replay_receipts()
                .expect("basket exposes retained replay"),
            scopes,
        )
        .compile()
        .expect("NMT certified scopes compile from receipt-backed platform evidence");

        assert_eq!(certified.scopes().len(), 6);
        let first = certified.layer(0).expect("first layer scope");
        assert_eq!(
            first.topology_scope().kind(),
            NmtTopologyScopeKind::OpenLayer
        );
        assert_ne!(
            first.projection().scope_projection_identity(),
            first.projection().parent_projection_identity(),
            "scope projection identity must not be the aggregate projection identity"
        );
        assert_ne!(
            first.retained_replay().scope_replay_identity(),
            first.retained_replay().parent_replay_identity(),
            "scope replay identity must not be the aggregate replay identity"
        );
        assert_eq!(
            first.projection().counters().scope_projected_faces(),
            first.topology_scope().counters().face_count()
        );
    });
}

#[test]
fn nmt_scope_context_rejects_cross_scope_projection_replay_surface_and_parity() {
    run_with_real_workload_stack(|| {
        let built = WorkloadCatalog::grazing_open_shell_basket_stack(
            GrazingBasketStackSpec::new().layers(4).strips_per_layer(8),
        )
        .declared("cross-scope NMT authority proof")
        .build()
        .expect("NMT basket stack catalog must build");
        let topology = built
            .topology_construction()
            .expect("basket catalog exposes topology construction");
        let scopes = NmtTopologyScopeSet::from_construction(topology)
            .expect("topology construction produces layer scopes");
        let certified = NmtCertifiedScopeSet::from_platform_evidence(
            topology,
            built.workload().evidence_ledger(),
            built.bound_geometry(),
            built.projected_workload(),
            built.transform_receipts(),
            built
                .replay_receipts()
                .expect("basket exposes retained replay"),
            scopes,
        )
        .compile()
        .expect("NMT certified scopes compile from receipt-backed platform evidence");
        let source = certified.layer(0).expect("source layer");
        let target = certified.layer(1).expect("target layer");

        assert_eq!(
            certified
                .attempt_cross_scope_projection(source.projection(), target)
                .expect_err("projection from another scope must deny")
                .kind(),
            &NmtCertificationDenialKind::CrossScopeProjection
        );
        assert_eq!(
            certified
                .attempt_cross_scope_retained_replay(source.retained_replay(), target)
                .expect_err("retained replay from another scope must deny")
                .kind(),
            &NmtCertificationDenialKind::CrossScopeRetainedReplay
        );
        assert_eq!(
            certified
                .attempt_cross_scope_surface_support(source.surface_support(), target)
                .expect_err("surface support from another scope must deny")
                .kind(),
            &NmtCertificationDenialKind::CrossScopeSurfaceSupport
        );
        assert_eq!(
            certified
                .attempt_cross_scope_parity(source.parity(), target)
                .expect_err("parity from another scope must deny")
                .kind(),
            &NmtCertificationDenialKind::CrossScopeParity
        );
    });
}

#[test]
fn nmt_scope_predicate_basis_uses_context_boundary_frame_motion_and_precision() {
    run_with_real_workload_stack(|| {
        let built = WorkloadCatalog::open_shell_nmt_edge_fan(4)
            .with_retained_replay_artifacts()
            .declared("NMT fan predicate basis proof")
            .build()
            .expect("NMT fan catalog must build");
        let topology = built
            .topology_construction()
            .expect("fan catalog exposes topology construction");
        let scopes =
            NmtTopologyScopeSet::from_construction(topology).expect("fan scope set compiles");
        let certified = NmtCertifiedScopeSet::from_platform_evidence(
            topology,
            built.workload().evidence_ledger(),
            built.bound_geometry(),
            built.projected_workload(),
            built.transform_receipts(),
            built
                .replay_receipts()
                .expect("fan exposes retained replay"),
            scopes,
        )
        .compile()
        .expect("fan certified scope compiles");
        let fan = certified
            .single_scope(NmtTopologyScopeKind::OpenRadialFan)
            .expect("fan scope");
        let boundary = fan.boundary_identity();
        let basis = fan
            .predicate_basis_for_boundary(&boundary)
            .expect("predicate basis must come from certified scope context");

        assert_eq!(
            basis.scope_identity(),
            fan.topology_scope().scope_identity()
        );
        assert_eq!(
            basis.boundary_identity(),
            fan.topology_scope().open_boundary_identity()
        );
        assert_eq!(
            basis.local_frame_identity(),
            fan.projection().local_frame_identity()
        );
        assert_eq!(
            basis.motion_identity(),
            fan.motion().scope_motion_identity()
        );
        assert!(basis
            .precision_policy_identity()
            .contains("local-feature-scale"));
    });
}

#[test]
fn nmt_boss_closeout_rejects_missing_required_outcome_kind() {
    run_with_real_workload_stack(|| {
        let closeout = radial_fan_closeout_evidence("nmt-closeout-missing-kind");
        let admitted_only = NmtBossOutcomeMatrixEvidence::from_outcomes(vec![closeout
            .matrix
            .outcomes()[0]
            .clone()]);

        let denial = NmtBossCloseoutReceipt::from_certified_scope_set(
            NmtBossId::OpenRadialFan,
            &closeout.certified_scopes,
            &admitted_only,
        )
        .expect_err("closeout must reject a matrix missing required branches");

        assert_eq!(
            denial,
            NmtBossCloseoutDenial::MissingOutcomeKind(WorthUserOutcomeKind::Unsupported)
        );
    });
}

#[test]
fn nmt_boss_closeout_rejects_wrong_certified_scope_set() {
    run_with_real_workload_stack(|| {
        let radial = radial_fan_closeout_evidence("nmt-closeout-radial-authority");
        let mixed = mixed_surface_closeout_evidence("nmt-closeout-wrong-authority");

        let denial = NmtBossCloseoutReceipt::from_certified_scope_set(
            NmtBossId::OpenRadialFan,
            &mixed.certified_scopes,
            &radial.matrix,
        )
        .expect_err("radial fan closeout must not accept open sheet authority");

        assert!(matches!(
            denial,
            NmtBossCloseoutDenial::WrongCertifiedScopeSet {
                boss: NmtBossId::OpenRadialFan,
                ..
            }
        ));
    });
}

#[test]
fn nmt_boss_closeout_cannot_be_called_without_certified_scope_set() {
    let missing = NmtCertifiedScopeSet::from_certified_open_class_members(&[])
        .expect_err("empty certified scope members cannot create closeout authority");

    assert_eq!(
        missing.kind(),
        &NmtCertificationDenialKind::MissingScopeEvidence
    );
}

#[test]
fn nmt_boss_modules_do_not_import_legacy_authority_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/certification/public_facade_contracts/contracts/planar_overlap/metaboss");
    for module in [
        "nmt_radial_fan",
        "mixed_surface_kill_box",
        "open_class_triad_parity",
        "grazing_basket_stack",
    ] {
        let module_root = root.join(module);
        for entry in std::fs::read_dir(&module_root).expect("NMT boss module directory exists") {
            let path = entry.expect("NMT boss source entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("NMT boss source reads");
            for banned in ["proof_fixture", "orient_basis", "admitted_handle"] {
                assert!(
                    !source.contains(banned),
                    "{module} must not import legacy authority fixture `{banned}`"
                );
            }
        }
    }
}

fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("nmt-scope-real-workload".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("NMT scope test thread starts")
        .join()
        .expect("NMT scope test thread passes");
}

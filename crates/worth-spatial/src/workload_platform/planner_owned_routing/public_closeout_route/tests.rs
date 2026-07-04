use super::current::{
    current_evidence_lookup_public_closeout, current_evidence_lookup_public_closeout_route_input,
    current_evidence_lookup_public_closeout_with_selected_route_support,
};
use super::error::EvidenceLookupPublicCloseoutErrorKind;
use super::input::SelectedEvidenceLookupPublicCloseoutRouteSupport;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn spatial_public_closeout_route_explanation_consumes_planner_route_products_without_evidence_rescan(
) {
    let route_input =
        current_evidence_lookup_public_closeout_route_input().expect("public closeout route input");
    let packet = route_input.route_packet();
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let seed = closeout.milestone_twelve_seed();

    assert_eq!(
        seed.selected_route_family_identity(),
        packet.route_family_identity()
    );
    assert_eq!(
        seed.selected_compiled_product_identity_digest(),
        packet.compiled_product_identity_digest()
    );
    assert_eq!(
        seed.selected_equivalence_family_identity(),
        packet.selected_equivalence_family_identity()
    );
    assert_eq!(
        seed.selected_reuse_basis_identity_digest(),
        packet.selected_reuse_basis_identity_digest()
    );
    assert_eq!(packet.lowering_raw_row_revisit_count(), 0);
    assert_eq!(packet.lowering_right_receipt_revisit_count(), 0);
    assert_eq!(packet.lowering_caller_owned_revisit_count(), 0);
}

#[test]
fn spatial_closeout_denial_localizes_family_or_support_mismatch() {
    let route_input =
        current_evidence_lookup_public_closeout_route_input().expect("public closeout route input");
    let support = route_input.selected_route_support().clone();

    let family_error = current_evidence_lookup_public_closeout_with_selected_route_support(
        SelectedEvidenceLookupPublicCloseoutRouteSupport::new(
            support.route_family_identity().to_string(),
            support.stage_receipt_family_identity().to_string(),
            support.selected_lookup_plan_digest().to_string(),
            support.lookup_execution_receipt_digest().to_string(),
            support.lookup_product_output_digest().to_string(),
            support.compiled_product_identity_digest().to_string(),
            support.equivalence_policy_identity_digest().to_string(),
            "foreign-selected-family".to_string(),
            support.selected_reuse_basis_identity_digest().to_string(),
        ),
    )
    .expect_err("family mismatch must deny");
    assert_eq!(
        family_error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteFamily
    );

    let support_error = current_evidence_lookup_public_closeout_with_selected_route_support(
        SelectedEvidenceLookupPublicCloseoutRouteSupport::new(
            support.route_family_identity().to_string(),
            support.stage_receipt_family_identity().to_string(),
            support.selected_lookup_plan_digest().to_string(),
            support.lookup_execution_receipt_digest().to_string(),
            support.lookup_product_output_digest().to_string(),
            support.compiled_product_identity_digest().to_string(),
            support.equivalence_policy_identity_digest().to_string(),
            support.selected_equivalence_family_identity().to_string(),
            "foreign-selected-reuse-basis".to_string(),
        ),
    )
    .expect_err("support mismatch must deny");
    assert_eq!(
        support_error.kind(),
        EvidenceLookupPublicCloseoutErrorKind::MismatchedSelectedRouteSupport
    );
}

#[test]
fn spatial_closeout_digests_bind_lookup_authority_chain() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");

    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .milestone_eleven_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .query_surface_matrix_digest(),
        closeout.query_surface_matrix().matrix_digest()
    );
    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .query_consumer_kit_closeout_digest(),
        closeout.query_consumer_kit().closeout_digest()
    );
    assert!(!closeout.query_boundary_support_digest().is_empty());
    assert_eq!(
        closeout.milestone_twelve_seed().source_firewall_digest(),
        closeout.source_firewall_report().firewall_digest()
    );
    assert_eq!(
        closeout.milestone_twelve_seed().residue_audit_digest(),
        closeout.residue_audit_digest()
    );
    assert_eq!(
        closeout.milestone_twelve_seed().family_coverage_digest(),
        closeout.family_coverage_digest()
    );
}

#[test]
fn public_closeout_residue_manifest_derives_from_live_query_consumer_report() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let manifest = super::current_evidence_lookup_public_closeout_residue_manifest();

    assert_eq!(
        manifest.len(),
        closeout.query_consumer_kit().query_residue_rows().len()
    );
    assert!(manifest.iter().all(|row| !row.source_path().is_empty()));
    assert!(manifest.iter().all(|row| !row.current_surface().is_empty()));
    assert!(manifest.iter().all(|row| !row.blocker().is_empty()));
    assert!(manifest.iter().all(|row| !row.removal_trigger().is_empty()));
    assert!(manifest
        .iter()
        .filter(|row| {
            row.disposition()
                == crate::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueDisposition::QueryGap
        })
        .all(|row| row.query_gap_kind().is_some()));
}

#[test]
fn ordinary_spatial_sources_do_not_import_legacy_public_closeout_lane() {
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "workload_platform::planner_owned_routing::public_closeout_route",
        &[
            "workload_platform/planner_owned_routing/public_closeout_route/",
            "certification/",
        ],
    );

    assert!(
        offenders.is_empty(),
        "ordinary spatial sources must import facade::evidence_lookup_public_closeout instead of the displaced planner-owned public-closeout lane: {offenders:?}"
    );
}

#[test]
fn ordinary_spatial_sources_do_not_import_legacy_public_closeout_facade() {
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "facade::planner_owned_routing::public_closeout_route",
        &[
            "certification/public_facade_contracts/compile_fail/evidence_lookup_public_closeout/",
            "workload_platform/planner_owned_routing/public_closeout_route/tests.rs",
        ],
    );

    assert!(
        offenders.is_empty(),
        "ordinary spatial sources must import facade::evidence_lookup_public_closeout instead of the displaced planner-owned public-closeout facade: {offenders:?}"
    );
}

#[test]
fn legacy_public_closeout_lane_has_no_live_current_authority_exports() {
    let planner_facade_root = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src/facade/planner_owned_routing/mod.rs"),
    )
    .expect("planner-owned routing facade should load");
    let facade_root =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/facade/mod.rs"))
            .expect("facade root should load");

    assert!(
        !planner_facade_root.contains("pub mod public_closeout_route;"),
        "planner-owned routing facade must not preserve public public-closeout exports"
    );
    assert!(
        facade_root.contains("pub mod evidence_lookup_public_closeout;"),
        "spatial facade must expose the cut-over evidence_lookup_public_closeout lane"
    );
}

#[test]
fn displaced_public_closeout_route_helpers_are_deleted() {
    let displaced_helper_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/workload_platform/evidence_lookup_public_closeout");
    assert!(
        !displaced_helper_root.exists(),
        "displaced public-closeout helper siblings must be deleted after planner-owned cutover"
    );
}

fn collect_legacy_imports(
    root: PathBuf,
    legacy_import: &str,
    allowed_relative_prefixes: &[&str],
) -> Vec<String> {
    let mut offenders = Vec::new();
    collect_legacy_imports_in_dir(
        &root,
        &root,
        legacy_import,
        allowed_relative_prefixes,
        &mut offenders,
    );
    offenders
}

fn collect_legacy_imports_in_dir(
    root: &Path,
    dir: &Path,
    legacy_import: &str,
    allowed_relative_prefixes: &[&str],
    offenders: &mut Vec<String>,
) {
    for entry in fs::read_dir(dir).expect("legacy import scan should read directory") {
        let path = entry.expect("legacy import scan entry").path();
        if path.is_dir() {
            collect_legacy_imports_in_dir(
                root,
                &path,
                legacy_import,
                allowed_relative_prefixes,
                offenders,
            );
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .expect("scanned file should stay under root")
            .to_string_lossy()
            .replace('\\', "/");
        if allowed_relative_prefixes
            .iter()
            .any(|prefix| relative.starts_with(prefix))
        {
            continue;
        }
        let source = fs::read_to_string(&path).expect("legacy import scan should read file");
        if source.contains(legacy_import) {
            offenders.push(relative);
        }
    }
}

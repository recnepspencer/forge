use std::fs;
use std::path::{Path, PathBuf};

use super::rows::{
    PlanarBooleanOverlapRegionExtractionArtifactOwnerRow,
    PlanarBooleanOverlapRegionExtractionLegacySurfaceRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionExtractionPathDenial {
    LocalTouchedGraphClone,
    LocalSelectedRouteClone,
    LocalQueryPostureClone,
    LocalRepresentativeCoverageClone,
    InPlaceLoopReconstructionRefactor,
    UnexpectedHelperPath,
    MissingDeclaredOwnerPath,
    MissingDeclaredConsumerPath,
    ArtifactOwnerPathMismatch,
    ArtifactConsumerPathMismatch,
    ResurrectedLegacyWrapperPath,
}

const SPATIAL_ALLOWED_TOP_LEVEL_PATHS: &[&str] = &[
    "adjacency_index",
    "arrangement_graph",
    "contact_area_classification",
    "containment_winding_field",
    "directory_cutover_map",
    "identity_naming_lineage",
    "island_components",
    "legacy_operator_surface",
    "mod.rs",
    "overlap_ledger",
    "participation",
    "post_admission_normalization",
    "pre_region_normalization",
    "readiness_boundary",
    "region_candidate_boundary",
    "replay_closeout",
    "shared_area_admission",
];
const TOPO_ALLOWED_TOP_LEVEL_PATHS: &[&str] = &[
    "classification.rs",
    "closeout.rs",
    "lane_honesty.rs",
    "mod.rs",
    "operator_row.rs",
    "phase_2_inventory",
    "registry.rs",
    "registry_identity.rs",
    "required_phase_2_operator_lanes.rs",
    "required_phase_2_rows.rs",
    "required_phase_2_validator_lanes.rs",
    "tests.rs",
    "validator_row.rs",
];
const TOPO_PHASE_TWO_INVENTORY_ALLOWED_PATHS: &[&str] =
    &["mod.rs", "operator_rows.rs", "validator_rows.rs"];
const KERNEL_ALLOWED_TOP_LEVEL_PATHS: &[&str] = &["mod.rs", "registration_contract.rs"];

pub(crate) fn certify_overlap_lane_relative_path(
    relative_path: &str,
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    let path = relative_path.replace('\\', "/");
    if path.contains("/touched_graph/")
        || path.contains("/touched_graph_parity/")
        || path.contains("/local_readiness/")
    {
        return Err(PlanarBooleanOverlapRegionExtractionPathDenial::LocalTouchedGraphClone);
    }
    if path.contains("/selected_route/") {
        return Err(PlanarBooleanOverlapRegionExtractionPathDenial::LocalSelectedRouteClone);
    }
    if path.contains("/query_posture/") {
        return Err(PlanarBooleanOverlapRegionExtractionPathDenial::LocalQueryPostureClone);
    }
    if path.contains("/representative_family_coverage/") {
        return Err(
            PlanarBooleanOverlapRegionExtractionPathDenial::LocalRepresentativeCoverageClone,
        );
    }
    if path.contains("planar_boolean_loop_reconstruction/overlap_region") {
        return Err(
            PlanarBooleanOverlapRegionExtractionPathDenial::InPlaceLoopReconstructionRefactor,
        );
    }
    Ok(())
}

pub(crate) fn scan_live_phase_two_family(
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    let spatial_root = spatial_overlap_lane_root();
    scan_root_with_allowlist(&spatial_root, SPATIAL_ALLOWED_TOP_LEVEL_PATHS, None)?;

    let topo_root =
        workspace_root().join("crates/worth-topo/src/topology_operators/overlap_region_blueprint");
    scan_root_with_allowlist(
        &topo_root,
        TOPO_ALLOWED_TOP_LEVEL_PATHS,
        Some(("phase_2_inventory", TOPO_PHASE_TWO_INVENTORY_ALLOWED_PATHS)),
    )?;

    let kernel_root = workspace_root().join(
        "crates/worth-kernel/src/workload_composition/planar_boolean_overlap_region_extraction",
    );
    scan_root_with_allowlist(&kernel_root, KERNEL_ALLOWED_TOP_LEVEL_PATHS, None)
}

pub(crate) fn certify_phase_two_artifact_contracts(
    rows: &[PlanarBooleanOverlapRegionExtractionArtifactOwnerRow],
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    let workspace_root = workspace_root();
    for row in rows {
        verify_artifact_source(
            &workspace_root,
            row.owning_source_path(),
            row.artifact_name(),
            PlanarBooleanOverlapRegionExtractionPathDenial::MissingDeclaredOwnerPath,
            PlanarBooleanOverlapRegionExtractionPathDenial::ArtifactOwnerPathMismatch,
        )?;
        verify_artifact_source(
            &workspace_root,
            row.consuming_source_path(),
            row.artifact_name(),
            PlanarBooleanOverlapRegionExtractionPathDenial::MissingDeclaredConsumerPath,
            PlanarBooleanOverlapRegionExtractionPathDenial::ArtifactConsumerPathMismatch,
        )?;
    }
    Ok(())
}

pub(crate) fn certify_deleted_legacy_surface_contracts(
    rows: &[PlanarBooleanOverlapRegionExtractionLegacySurfaceRow],
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    let workspace_root = workspace_root();
    for row in rows {
        if row.must_be_absent() && workspace_root.join(row.source_path()).exists() {
            return Err(
                PlanarBooleanOverlapRegionExtractionPathDenial::ResurrectedLegacyWrapperPath,
            );
        }
    }
    Ok(())
}

fn scan_relative_path(
    lane_root: &Path,
    path: &Path,
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    let relative = path
        .strip_prefix(lane_root)
        .expect("scanned path should remain within overlap lane")
        .to_string_lossy()
        .replace('\\', "/");
    certify_overlap_lane_relative_path(&relative)?;
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("overlap lane directory should load") {
            let entry = entry.expect("overlap lane child entry should load");
            scan_relative_path(lane_root, &entry.path())?;
        }
    }
    Ok(())
}

fn scan_root_with_allowlist(
    root: &Path,
    allowed_top_level_paths: &[&str],
    nested_allowlist: Option<(&str, &[&str])>,
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    for entry in fs::read_dir(root).expect("phase 2 root should exist") {
        let entry = entry.expect("phase 2 root entry should load");
        let name = entry.file_name().to_string_lossy().replace('\\', "/");
        if !allowed_top_level_paths.contains(&name.as_str()) {
            return Err(PlanarBooleanOverlapRegionExtractionPathDenial::UnexpectedHelperPath);
        }
        if let Some((nested_dir, nested_allowed_paths)) = nested_allowlist {
            if name == nested_dir {
                scan_root_with_allowlist(&entry.path(), nested_allowed_paths, None)?;
                continue;
            }
        }
        scan_relative_path(root, &entry.path())?;
    }
    Ok(())
}

fn verify_artifact_source(
    workspace_root: &Path,
    source_path: &str,
    artifact_name: &str,
    missing_denial: PlanarBooleanOverlapRegionExtractionPathDenial,
    mismatch_denial: PlanarBooleanOverlapRegionExtractionPathDenial,
) -> Result<(), PlanarBooleanOverlapRegionExtractionPathDenial> {
    certify_overlap_lane_relative_path(source_path)?;
    let source = workspace_root.join(source_path);
    if !source.exists() {
        return Err(missing_denial);
    }
    let contents = fs::read_to_string(source).expect("artifact source should be readable");
    if !contents.contains(artifact_name) {
        return Err(mismatch_denial);
    }
    Ok(())
}

fn spatial_overlap_lane_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/workload_platform/planar_boolean_overlap_region_extraction")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("worth-spatial crate should have a parent directory")
        .parent()
        .expect("workspace should sit above crates")
        .to_path_buf()
}

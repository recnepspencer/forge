use super::{export_block, read_repository_document};
use crate::durable_publication_boundary_gate::public_api::locked_surfaces::PHASE_TEN_SURFACES;

pub(super) fn assert_reachability(durability_exports: &str) {
    for surface in PHASE_TEN_SURFACES {
        if is_recovery_dependency_surface(surface) {
            continue;
        }
        let type_or_function = surface.split("::").next().unwrap_or(surface);
        if surface.contains("::") {
            continue;
        }
        assert!(
            durability_exports.contains(type_or_function),
            "Phase 10 Store surface `{surface}` is hidden from physical_runtime"
        );
    }

    let closeout = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/closeout/handoff.rs",
    )
    .expect("read durability closeout facade owner");
    for method in [
        "pub const fn recovery_handoff",
        "pub fn into_recovery_handoff",
    ] {
        assert!(closeout.contains(method));
    }

    let termination = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/termination/outcome.rs",
    )
    .expect("read Store termination outcome owner");
    for method in [
        "pub const fn recovery_handoff",
        "pub fn into_recovery_handoff",
    ] {
        assert!(termination.contains(method));
    }

    let shutdown = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_shutdown.rs",
    )
    .expect("read serving shutdown owner");
    for method in [
        "pub const fn durability_closeout",
        "pub const fn performance",
    ] {
        assert!(shutdown.contains(method));
    }

    let runtime = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    )
    .expect("read physical runtime facade");
    let exports = export_block(&runtime, "pub use durability::{");
    for surface in PHASE_TEN_SURFACES {
        if !surface.contains("::") && !is_recovery_dependency_surface(surface) {
            assert!(exports.contains(surface));
        }
    }

    let wal = read_repository_document("workspaces/worth-store/crates/worth-store-wal/src/lib.rs")
        .expect("read WAL facade");
    for surface in [
        "InterruptedWalTail",
        "VerifiedWalActiveTail",
        "inspect_verified_wal_active_tail",
    ] {
        assert!(wal.contains(surface));
    }

    let truncation = read_repository_document(
        "workspaces/worth-store/crates/worth-store-physical-backend/src/filesystem_media/artifact_tree/durable_truncation.rs",
    )
    .expect("read durable artifact truncation owner");
    assert!(truncation.contains("pub fn truncate_file_durably"));
}

fn is_recovery_dependency_surface(surface: &str) -> bool {
    matches!(
        surface,
        "ArtifactTreeMedia::truncate_file_durably"
            | "InterruptedWalTail"
            | "VerifiedWalActiveTail"
            | "inspect_verified_wal_active_tail"
    )
}

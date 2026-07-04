use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn query_backed_cutover_lane_is_the_public_authority_owner() {
    let source = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/facade.rs"))
        .expect("topology facade should load");

    assert!(
        source.contains("crate::projection::query_backed_consumer_cutover"),
        "topology facade must source ordinary query-backed authority from projection::query_backed_consumer_cutover"
    );
}

#[test]
fn ordinary_topology_sources_do_not_import_legacy_planner_owned_query_backed_lane() {
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "projection::planner_owned_routing::query_backed_read_family",
        &[
            "projection/planner_owned_routing/query_backed_read_family/",
            "certification/",
        ],
    );

    assert!(
        offenders.is_empty(),
        "ordinary topology sources must import projection::query_backed_consumer_cutover instead of the displaced planner-owned query-backed lane: {offenders:?}"
    );
}

#[test]
fn legacy_diagnostic_surfaces_lane_is_deleted() {
    let projection_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/projection");
    let projection_mod =
        fs::read_to_string(projection_root.join("mod.rs")).expect("projection root should load");

    assert!(
        !projection_mod.contains("mod diagnostic_surfaces;"),
        "projection root must not keep the deleted diagnostic_surfaces lane mounted"
    );
    assert!(
        !projection_root.join("diagnostic_surfaces").exists(),
        "legacy diagnostic_surfaces directory must be deleted instead of surviving as a tiny residue shell"
    );
}

#[test]
fn ordinary_topology_sources_do_not_import_legacy_diagnostic_surfaces_lane() {
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "projection::diagnostic_surfaces",
        &[
            "certification/",
            "projection/query_backed_consumer_cutover/tests.rs",
        ],
    );

    assert!(
        offenders.is_empty(),
        "ordinary topology sources must import planner_owned_routing::diagnostic_projection_input instead of the deleted diagnostic_surfaces lane: {offenders:?}"
    );
}

#[test]
fn ordinary_topology_sources_do_not_import_planner_owned_diagnostic_projection_input() {
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "projection::planner_owned_routing::diagnostic_projection_input",
        &[
            "projection/planner_owned_routing/diagnostic_projection_input/",
            "certification/",
            "projection/query_backed_consumer_cutover/tests.rs",
            "projection/runtime_boundary/declared_query_surfaces/tests.rs",
            "derived_topology/compiled_product_consumer_cutover/tests.rs",
        ],
    );

    assert!(
        offenders.is_empty(),
        "ordinary topology sources must import runtime_boundary::diagnostic_projection instead of the capped planner-owned diagnostic residue lane: {offenders:?}"
    );
}

#[test]
fn certification_support_surfaces_do_not_import_planner_owned_diagnostic_projection_input_wrapper()
{
    let offenders = collect_legacy_imports(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        "projection::planner_owned_routing::diagnostic_projection_input::",
        &[
            "projection/planner_owned_routing/diagnostic_projection_input/report_types.rs",
            "projection/planner_owned_routing/diagnostic_projection_input/source.rs",
            "projection/query_backed_consumer_cutover/tests.rs",
        ],
    )
    .into_iter()
    .filter(|relative| {
        let source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join(relative),
        )
        .expect("wrapper import scan should read file");
        source.contains(
            "projection::planner_owned_routing::diagnostic_projection_input::build_derived",
        ) || source.contains(
            "projection::planner_owned_routing::diagnostic_projection_input::derive_topology",
        ) || source
            .contains("projection::planner_owned_routing::diagnostic_projection_input::Derived")
    })
    .collect::<Vec<_>>();

    assert!(
        offenders.is_empty(),
        "certification and support callers must import exact residue submodules instead of the planner-owned diagnostic wrapper path: {offenders:?}"
    );
}

#[test]
fn public_derived_read_diagnostic_input_adapter_lane_is_deleted() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let lib_source = fs::read_to_string(src_root.join("lib.rs")).expect("topology lib should load");
    let facade_source =
        fs::read_to_string(src_root.join("facade.rs")).expect("topology facade should load");

    assert!(
        !lib_source.contains("pub mod derived_read_diagnostic_input;"),
        "topology crate root must not keep the public derived_read_diagnostic_input adapter lane mounted"
    );
    assert!(
        !src_root.join("derived_read_diagnostic_input").exists(),
        "public derived_read_diagnostic_input adapter directory must be deleted instead of surviving as compatibility residue"
    );
    assert!(
        !facade_source.contains("crate::derived_read_diagnostic_input"),
        "topology facade must not re-export the deleted derived_read_diagnostic_input compatibility adapter lane"
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

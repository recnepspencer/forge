use std::path::Path;

use super::workspace_source::{
    occurrence_count, production_rust_sources, read, workspace_relative,
};
use crate::workspace_root;

const PHYSICAL_RUNTIME_ROOT: &str = "crates/worth-store/src/physical_runtime";
const BOOTSTRAP_ROUTE: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/record_frame_reader.rs";
const DIRECT_SOURCE: &str = "crates/worth-store/src/physical_runtime/record_serving/residency/frame_loading/read_source/direct.rs";

#[test]
fn direct_frame_source_has_one_bootstrap_construction_owner() {
    let sources =
        production_rust_sources(&workspace_root().join(PHYSICAL_RUNTIME_ROOT)).expect("sources");
    let mut constructors = Vec::new();
    for source in sources {
        let text = read(&source).expect("read physical runtime source");
        let count = occurrence_count(&text, "DirectFrameReadSource::new");
        constructors.extend(std::iter::repeat_n(workspace_relative(&source), count));
    }
    assert!(
        !constructors.is_empty() && constructors.iter().all(|site| site == BOOTSTRAP_ROUTE),
        "direct frame reads must have one bootstrap owner and no serving bypass: {constructors:?}"
    );

    let route = read(&workspace_root().join(BOOTSTRAP_ROUTE)).expect("read bootstrap route");
    inspect_route_arms(Path::new(BOOTSTRAP_ROUTE), &route)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn direct_source_visibility_and_effect_remain_narrow() {
    let source = read(&workspace_root().join(DIRECT_SOURCE)).expect("read direct source");
    assert!(
        source.contains(
            "pub(in crate::physical_runtime::record_serving::residency) struct DirectFrameReadSource"
        ),
        "direct source must remain private to the bootstrap-owning residency boundary"
    );
    assert!(
        source.contains("PhysicalRecordArtifactTree::new(self.media)")
            && source.contains(".read_exact_at(self.artifact, self.offset, target)"),
        "direct source must remain the explicit bootstrap media adapter"
    );
}

#[test]
fn direct_media_gate_rejects_serving_and_foreign_constructors() {
    let serving_mutant = r#"
match &self.route {
    RecordFrameReadRoute::Serving { frame_ports, source } => {
        loader.load_exact(&DirectFrameReadSource::new(media), artifact, offset, length)
    }
}
"#;
    let denial = inspect_route_arms(Path::new(BOOTSTRAP_ROUTE), serving_mutant)
        .expect_err("serving direct-source bypass must be denied");
    assert!(denial.contains("serving arm"));

    let denial = inspect_constructor_site(
        Path::new("crates/worth-store/src/physical_runtime/record_serving/access/bypass.rs"),
        "DirectFrameReadSource::new(media)",
    )
    .expect_err("foreign direct-source construction must be denied");
    assert!(denial.contains("outside bootstrap route"));
}

fn inspect_constructor_site(path: &Path, source: &str) -> Result<(), String> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if source.contains("DirectFrameReadSource::new") && normalized != BOOTSTRAP_ROUTE {
        return Err(format!(
            "physical residency boundary: DirectFrameReadSource constructed outside bootstrap route at {}",
            path.display()
        ));
    }
    Ok(())
}

fn inspect_route_arms(path: &Path, source: &str) -> Result<(), String> {
    let mut route = None;
    for (index, line) in source.lines().enumerate() {
        if line.contains("RecordFrameReadRoute::Bootstrap") {
            route = Some("bootstrap");
        } else if line.contains("RecordFrameReadRoute::Serving") {
            route = Some("serving");
        }
        if line.contains("DirectFrameReadSource::new") && route != Some("bootstrap") {
            return Err(format!(
                "physical residency boundary: direct media construction in serving arm at {}:{}",
                path.display(),
                index + 1
            ));
        }
    }
    Ok(())
}

use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const SERVING_CAPABILITY: &str =
    "crates/worth-store/src/physical_runtime/record_serving/residency/capability.rs";

#[test]
fn serving_residency_exposes_composed_operations_not_raw_frame_ports() {
    let source = read(&workspace_root().join(SERVING_CAPABILITY))
        .expect("read serving residency capability");
    inspect_serving_capability(Path::new(SERVING_CAPABILITY), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn serving_capability_gate_rejects_raw_port_and_loader_mutants() {
    for mutant in [
        "fn frame_ports(&self) -> &RecordFramePorts { &self.frame_ports }",
        "fn loader(&self) -> &dyn FrameLoadPort { self.frame_ports.loader() }",
    ] {
        let denial = inspect_serving_capability(Path::new("controlled_mutant.rs"), mutant)
            .expect_err("a serving capability escape must be denied");
        assert!(denial.contains("raw residency authority"));
    }
}

fn inspect_serving_capability(path: &Path, source: &str) -> Result<(), String> {
    if source.contains("fn frame_ports(") || source.contains("fn loader(") {
        return Err(format!(
            "physical residency boundary: raw residency authority escapes {}",
            path.display()
        ));
    }
    if path == Path::new(SERVING_CAPABILITY)
        && (!source.contains("fn begin_candidate_publication")
            || !source.contains("StoreCandidateFramePublicationSession::begin("))
    {
        return Err(
            "physical residency boundary: candidate publication lost its composed operation"
                .to_owned(),
        );
    }
    Ok(())
}

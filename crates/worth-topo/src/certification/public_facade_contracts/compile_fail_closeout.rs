use std::path::Path;

use super::phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPublicFacadeCompileFailCloseoutErrorKind {
    MissingFixture,
    MissingExpectedDiagnostic,
    EmptyExpectedDiagnostic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPublicFacadeCompileFailCloseoutError {
    kind: TopologyPublicFacadeCompileFailCloseoutErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPublicFacadeCompileFailCloseout {
    fixture_paths: Vec<String>,
    covered_fence_classes: Vec<String>,
    closeout_digest: String,
}

pub fn current_topology_public_facade_compile_fail_closeout(
) -> Result<TopologyPublicFacadeCompileFailCloseout, TopologyPublicFacadeCompileFailCloseoutError> {
    let mut fence_proof_parts = Vec::new();
    for fence in phase_fifteen_topology_compile_fail_fences() {
        let fixture_path = crate_relative_path(fence.fixture_path());
        if !fixture_path.exists() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!(
                    "phase 15 topology compile-fail fixture missing: {}",
                    fence.fixture_path()
                ),
            ));
        }
        let stderr_path = crate_relative_path(&fence.stderr_path());
        if !stderr_path.exists() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "phase 15 topology compile-fail diagnostic missing: {}",
                    stderr_path.display()
                ),
            ));
        }
        let fixture_source = std::fs::read_to_string(&fixture_path).map_err(|error| {
            TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!(
                    "phase 15 topology compile-fail fixture unreadable: {} ({error})",
                    fixture_path.display()
                ),
            )
        })?;
        let expected_diagnostic = std::fs::read_to_string(&stderr_path).map_err(|error| {
            TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "phase 15 topology compile-fail diagnostic unreadable: {} ({error})",
                    stderr_path.display()
                ),
            )
        })?;
        if expected_diagnostic.trim().is_empty() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::EmptyExpectedDiagnostic,
                format!(
                    "phase 15 topology compile-fail diagnostic must be non-empty: {}",
                    stderr_path.display()
                ),
            ));
        }
        let fixture_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:phase-fifteen-compile-fail-fixture:v1".to_string(),
                format!("path:{}", fence.fixture_path()),
                fixture_source,
            ],
        );
        let diagnostic_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:phase-fifteen-compile-fail-diagnostic:v1".to_string(),
                format!("path:{}", fence.stderr_path()),
                expected_diagnostic,
            ],
        );
        fence_proof_parts.push(format!(
            "{}:{}:{}",
            fence.fence_class(),
            fixture_digest,
            diagnostic_digest
        ));
    }

    let fixture_paths = phase_fifteen_topology_compile_fail_fences()
        .iter()
        .map(|fence| fence.fixture_path().to_string())
        .collect::<Vec<_>>();
    let covered_fence_classes = phase_fifteen_topology_compile_fail_fences()
        .iter()
        .map(|fence| fence.fence_class().to_string())
        .collect::<Vec<_>>();
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &fence_proof_parts
            .into_iter()
            .chain(std::iter::once(
                "worth-topo:phase-fifteen-public-facade-compile-fail-closeout:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    Ok(TopologyPublicFacadeCompileFailCloseout {
        fixture_paths,
        covered_fence_classes,
        closeout_digest,
    })
}

impl TopologyPublicFacadeCompileFailCloseoutError {
    fn new(kind: TopologyPublicFacadeCompileFailCloseoutErrorKind, detail: String) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> TopologyPublicFacadeCompileFailCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl TopologyPublicFacadeCompileFailCloseout {
    pub fn fixture_paths(&self) -> &[String] {
        &self.fixture_paths
    }

    pub fn covered_fence_classes(&self) -> &[String] {
        &self.covered_fence_classes
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn crate_relative_path(relative_path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

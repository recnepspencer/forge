use std::path::Path;

use super::phase_fifteen_fixture_inventory::phase_fifteen_topology_compile_fail_fences;
use super::phase_fourteen_fixture_inventory::phase_fourteen_topology_compile_fail_fences;
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
    closeout_from_inventory(None)
}

pub fn topology_public_facade_compile_fail_closeout_excluding_fence_class_for_tests(
    excluded_fence_class: &str,
) -> Result<TopologyPublicFacadeCompileFailCloseout, TopologyPublicFacadeCompileFailCloseoutError> {
    closeout_from_inventory(Some(excluded_fence_class))
}

fn closeout_from_inventory(
    excluded_fence_class: Option<&str>,
) -> Result<TopologyPublicFacadeCompileFailCloseout, TopologyPublicFacadeCompileFailCloseoutError> {
    let mut fence_proof_parts = Vec::new();
    let fences = phase_fifteen_topology_compile_fail_fences()
        .iter()
        .map(|fence| {
            (
                fence.fixture_path(),
                fence.stderr_path(),
                fence.fence_class(),
                "phase 15 topology",
            )
        })
        .chain(
            phase_fourteen_topology_compile_fail_fences()
                .iter()
                .map(|fence| {
                    (
                        fence.fixture_path(),
                        fence.stderr_path(),
                        fence.fence_class(),
                        "phase 14 topology",
                    )
                }),
        )
        .filter(|(_, _, fence_class, _)| excluded_fence_class != Some(*fence_class))
        .collect::<Vec<_>>();
    for fence in &fences {
        let fixture_path = crate_relative_path(fence.0);
        if !fixture_path.exists() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!("{} compile-fail fixture missing: {}", fence.3, fence.0),
            ));
        }
        let stderr_path = crate_relative_path(&fence.1);
        if !stderr_path.exists() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "{} compile-fail diagnostic missing: {}",
                    fence.3,
                    stderr_path.display()
                ),
            ));
        }
        let fixture_source = std::fs::read_to_string(&fixture_path).map_err(|error| {
            TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!(
                    "{} compile-fail fixture unreadable: {} ({error})",
                    fence.3,
                    fixture_path.display()
                ),
            )
        })?;
        let expected_diagnostic = std::fs::read_to_string(&stderr_path).map_err(|error| {
            TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "{} compile-fail diagnostic unreadable: {} ({error})",
                    fence.3,
                    stderr_path.display()
                ),
            )
        })?;
        if expected_diagnostic.trim().is_empty() {
            return Err(TopologyPublicFacadeCompileFailCloseoutError::new(
                TopologyPublicFacadeCompileFailCloseoutErrorKind::EmptyExpectedDiagnostic,
                format!(
                    "{} compile-fail diagnostic must be non-empty: {}",
                    fence.3,
                    stderr_path.display()
                ),
            ));
        }
        let fixture_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:public-facade-compile-fail-fixture:v2".to_string(),
                format!("path:{}", fence.0),
                fixture_source,
            ],
        );
        let diagnostic_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:public-facade-compile-fail-diagnostic:v2".to_string(),
                format!("path:{}", fence.1),
                expected_diagnostic,
            ],
        );
        fence_proof_parts.push(format!(
            "{}:{}:{}",
            fence.2, fixture_digest, diagnostic_digest
        ));
    }

    let fixture_paths = fences
        .iter()
        .map(|fence| fence.0.to_string())
        .collect::<Vec<_>>();
    let covered_fence_classes = fences
        .iter()
        .map(|fence| fence.2.to_string())
        .collect::<Vec<_>>();
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &fence_proof_parts
            .into_iter()
            .chain(std::iter::once(
                "worth-topo:public-facade-compile-fail-closeout:v2".to_string(),
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

use std::path::Path;

use super::phase_fifteen_fixture_inventory::phase_fifteen_spatial_compile_fail_fences;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialPublicFacadeCompileFailCloseoutErrorKind {
    MissingFixture,
    MissingExpectedDiagnostic,
    EmptyExpectedDiagnostic,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialPublicFacadeCompileFailCloseoutError {
    kind: SpatialPublicFacadeCompileFailCloseoutErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialPublicFacadeCompileFailCloseout {
    fixture_paths: Vec<String>,
    covered_fence_classes: Vec<String>,
    closeout_digest: String,
}

pub fn current_spatial_public_facade_compile_fail_closeout(
) -> Result<SpatialPublicFacadeCompileFailCloseout, SpatialPublicFacadeCompileFailCloseoutError> {
    closeout_from_inventory(None)
}

pub fn spatial_public_facade_compile_fail_closeout_excluding_fence_class_for_tests(
    excluded_fence_class: &str,
) -> Result<SpatialPublicFacadeCompileFailCloseout, SpatialPublicFacadeCompileFailCloseoutError> {
    closeout_from_inventory(Some(excluded_fence_class))
}

fn closeout_from_inventory(
    excluded_fence_class: Option<&str>,
) -> Result<SpatialPublicFacadeCompileFailCloseout, SpatialPublicFacadeCompileFailCloseoutError> {
    let mut fence_proof_parts = Vec::new();
    let fences = phase_fifteen_spatial_compile_fail_fences()
        .iter()
        .filter(|fence| excluded_fence_class != Some(fence.fence_class()))
        .collect::<Vec<_>>();
    for fence in &fences {
        let fixture_path = crate_relative_path(fence.fixture_path());
        if !fixture_path.exists() {
            return Err(SpatialPublicFacadeCompileFailCloseoutError::new(
                SpatialPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!(
                    "phase 15 spatial compile-fail fixture missing: {}",
                    fence.fixture_path()
                ),
            ));
        }
        let stderr_path = crate_relative_path(&fence.stderr_path());
        if !stderr_path.exists() {
            return Err(SpatialPublicFacadeCompileFailCloseoutError::new(
                SpatialPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "phase 15 spatial compile-fail diagnostic missing: {}",
                    stderr_path.display()
                ),
            ));
        }
        let fixture_source = std::fs::read_to_string(&fixture_path).map_err(|error| {
            SpatialPublicFacadeCompileFailCloseoutError::new(
                SpatialPublicFacadeCompileFailCloseoutErrorKind::MissingFixture,
                format!(
                    "phase 15 spatial compile-fail fixture unreadable: {} ({error})",
                    fixture_path.display()
                ),
            )
        })?;
        let expected_diagnostic = std::fs::read_to_string(&stderr_path).map_err(|error| {
            SpatialPublicFacadeCompileFailCloseoutError::new(
                SpatialPublicFacadeCompileFailCloseoutErrorKind::MissingExpectedDiagnostic,
                format!(
                    "phase 15 spatial compile-fail diagnostic unreadable: {} ({error})",
                    stderr_path.display()
                ),
            )
        })?;
        if expected_diagnostic.trim().is_empty() {
            return Err(SpatialPublicFacadeCompileFailCloseoutError::new(
                SpatialPublicFacadeCompileFailCloseoutErrorKind::EmptyExpectedDiagnostic,
                format!(
                    "phase 15 spatial compile-fail diagnostic must be non-empty: {}",
                    stderr_path.display()
                ),
            ));
        }
        let fixture_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:phase-fifteen-compile-fail-fixture:v1".to_string(),
                format!("path:{}", fence.fixture_path()),
                fixture_source,
            ],
        );
        let diagnostic_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:phase-fifteen-compile-fail-diagnostic:v1".to_string(),
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

    let fixture_paths = fences
        .iter()
        .map(|fence| fence.fixture_path().to_string())
        .collect::<Vec<_>>();
    let covered_fence_classes = fences
        .iter()
        .map(|fence| fence.fence_class().to_string())
        .collect::<Vec<_>>();
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &fence_proof_parts
            .into_iter()
            .chain(std::iter::once(
                "worth-spatial:phase-fifteen-public-facade-compile-fail-closeout:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    );

    Ok(SpatialPublicFacadeCompileFailCloseout {
        fixture_paths,
        covered_fence_classes,
        closeout_digest,
    })
}

impl SpatialPublicFacadeCompileFailCloseoutError {
    fn new(kind: SpatialPublicFacadeCompileFailCloseoutErrorKind, detail: String) -> Self {
        Self { kind, detail }
    }

    pub const fn kind(&self) -> SpatialPublicFacadeCompileFailCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl SpatialPublicFacadeCompileFailCloseout {
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

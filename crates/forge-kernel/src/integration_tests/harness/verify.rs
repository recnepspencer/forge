//! Fluent verification chain for solid assertions.
//!
//! DOMAIN: Accumulates all assertion failures before panicking, so you
//! see every violation at once instead of fixing them one at a time.
//! Integrates with `dump` module to auto-export OBJ on failure.
//!
//! All checks compare actual results against test-provided expected values.
//! No redundant validation — that's what `commit()` is for.
//!
//! ```rust,ignore
//! verify(&envelope)
//!     .faces(6)
//!     .volume_approx(8.0, 1e-6)
//!     .pass();
//! ```

use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::{solid_volume, GeometryView};

/// Create a verifier for a `SolidEnvelope`.
pub fn verify(env: &SolidEnvelope) -> Verifier<'_> {
    Verifier {
        env,
        failures: Vec::new(),
        test_name: String::new(),
    }
}

/// Fluent assertion builder that accumulates failures.
pub struct Verifier<'a> {
    env: &'a SolidEnvelope,
    failures: Vec<String>,
    test_name: String,
}

impl<'a> Verifier<'a> {
    /// Set a test name for diagnostics and OBJ dump filenames.
    pub fn named(mut self, name: &str) -> Self {
        self.test_name = name.to_string();
        self
    }

    // ── Expectation checks (test-specific expected values) ────────────────

    /// Assert exact face count.
    pub fn faces(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().face_count();
        if actual != expected {
            self.failures
                .push(format!("Faces: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact vertex count.
    pub fn vertices(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().vertex_count();
        if actual != expected {
            self.failures
                .push(format!("Vertices: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact edge count.
    pub fn edges(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().edge_count();
        if actual != expected {
            self.failures
                .push(format!("Edges: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact halfedge count.
    pub fn half_edges(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().half_edge_count();
        if actual != expected {
            self.failures
                .push(format!("HalfEdges: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact loop count.
    pub fn loops(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().loop_count();
        if actual != expected {
            self.failures
                .push(format!("Loops: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact shell count.
    pub fn shells(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().shell_count();
        if actual != expected {
            self.failures
                .push(format!("Shells: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert exact body count.
    pub fn bodies(mut self, expected: usize) -> Self {
        let actual = self.env.topology().arena().body_count();
        if actual != expected {
            self.failures
                .push(format!("Bodies: {actual}, expected {expected}"));
        }
        self
    }

    /// Assert the solid's volume is approximately `expected ± tol`.
    ///
    /// Delegates to `geometry::facade::solid_volume`.
    pub fn volume_approx(mut self, expected: f64, tol: f64) -> Self {
        let volume = solid_volume(self.env.topology().arena(), self.env.geometry());
        if (volume - expected).abs() > tol {
            self.failures.push(format!(
                "Volume: {volume:.6}, expected {expected:.6} ± {tol:.2e} (diff: {:.2e})",
                (volume - expected).abs()
            ));
        }
        self
    }

    // ── Decision log checks ──────────────────────────────────────────────

    /// Assert decisions are well-formed (non-empty log).
    pub fn well_formed(mut self, log: &forge_core::DecisionLog) -> Self {
        if log.decisions().next().is_none() {
            self.failures.push("Decisions: log is empty".to_string());
        }
        self
    }

    /// Assert the decision log has at least `n` decisions.
    pub fn min_decisions(mut self, log: &forge_core::DecisionLog, min: usize) -> Self {
        let count = log.decisions().count();
        if count < min {
            self.failures.push(format!(
                "Decisions: {count} decisions, expected at least {min}"
            ));
        }
        self
    }

    /// Consume the verifier and panic if any checks failed.
    ///
    /// On failure, dumps the solid to OBJ for visual inspection.
    pub fn pass(self) {
        if self.failures.is_empty() {
            return;
        }

        let name = if self.test_name.is_empty() {
            "unnamed_test".to_string()
        } else {
            self.test_name.clone()
        };

        // Try to dump OBJ for visual debugging
        let dump_msg = match super::dump::dump_to_obj(self.env, &name) {
            Ok(path) => format!("\n\nMesh dumped to: {}", path.display()),
            Err(e) => format!("\n\n(OBJ dump failed: {e})"),
        };

        panic!(
            "{} verification failure(s):{}{}",
            self.failures.len(),
            self.failures
                .iter()
                .enumerate()
                .map(|(i, f)| format!("\n  {}. {}", i + 1, f))
                .collect::<String>(),
            dump_msg
        );
    }
}

//! Architectural boundary enforcement tests.
//!
//! DOMAIN: Compile-time-like checks that enforce layering rules.
//! These are the "authoritarian" guards — any violation fails CI.
//!
//! Rules enforced:
//! 1. No direct `forge_math::linalg::` access (route through forge-geom)
//! 2. No direct `forge_math::predicates::` access (route through forge-geom)
//! 3. No ad-hoc floating-point math in kernel orchestration code
//!
//! Exempted paths:
//! - `_deprecated/` — legacy code being phased out
//! - `architecture_guard` — this file
//! - `geometry/data/position.rs` — `Rational` type import (data, not computation)
//! - `geometry/contracts/` — `Rational` type in trait signatures
//! - `context/tracing/` — `PrecisionEscalation` type in decision logging
//! - `proof/tests/` — proof system tests exercise math directly by design
//! - `geometry/logic/source_adapter.rs` — `GeometrySource` bridge trait
//! - `operations/boolean/classify_faces.rs` — `PrecisionEscalation` type import

use std::path::Path;

/// Walk a directory recursively, collecting all `.rs` file paths.
fn collect_rs_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_rs_files(&path));
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    files
}

/// Returns true if the given source line is actual code
/// (not a doc comment, inline comment, or block comment).
fn is_code_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with("//") {
        return false;
    }
    if trimmed.starts_with('*') || trimmed.starts_with("/*") {
        return false;
    }
    true
}

fn normalize_rel_path(rel_path: &str) -> String {
    rel_path.replace('\\', "/")
}

/// Whether a file path should be exempt from architecture guards.
fn is_exempt_path(rel_path: &str) -> bool {
    // Deprecated code being phased out
    if rel_path.contains("_deprecated") {
        return true;
    }
    // This file itself
    if rel_path.contains("architecture_guard") {
        return true;
    }
    // Proof system tests exercise math directly by design
    if rel_path.contains("proof/tests") {
        return true;
    }
    false
}

/// Paths that are allowed to import forge_math types (not computation).
/// These import data types like `Rational`, `PrecisionEscalation`, etc.
fn is_forge_math_type_import_allowed(rel_path: &str) -> bool {
    // ExactPosition stores Rational — data type, not computation
    if rel_path.ends_with("geometry/data/position.rs") {
        return true;
    }
    // GeometryView trait uses Rational in signatures
    if rel_path.contains("geometry/contracts/") {
        return true;
    }
    // Decision logging uses PrecisionEscalation type
    if rel_path.contains("context/tracing/") {
        return true;
    }
    // GeometrySource bridge trait
    if rel_path.ends_with("geometry/logic/source_adapter.rs") {
        return true;
    }
    // classify_faces uses PrecisionEscalation type
    if rel_path.ends_with("operations/boolean/classify_faces.rs") {
        return true;
    }
    false
}

/// Paths that are allowed to interact with low-level forge-signal runtime types.
fn is_signal_contract_path_allowed(rel_path: &str) -> bool {
    if rel_path.ends_with("engine/feature_tree.rs") {
        return true;
    }
    if rel_path.ends_with("engine/tests.rs") {
        return true;
    }
    false
}

/// Paths that are allowed to interact with raw projected-topology internals.
fn is_projection_contract_path_allowed(rel_path: &str) -> bool {
    if rel_path.contains("engine/output/spec_envelope/") {
        return true;
    }
    if rel_path.ends_with("engine/output/testing/spec_envelope.rs") {
        return true;
    }
    false
}

// ── Guard 1: No direct forge_math::linalg:: access ─────────────────────

/// No direct `forge_math::linalg::` calls allowed in kernel code.
/// All linear algebra must route through `forge-geom` facade.
#[test]
fn no_forge_math_linalg_bypass() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rs_files = collect_rs_files(&kernel_src);

    let mut violations = Vec::new();

    for file in &rs_files {
        let rel_path = file
            .strip_prefix(&kernel_src)
            .unwrap_or(file)
            .to_string_lossy();
        let rel_path = normalize_rel_path(&rel_path);

        if is_exempt_path(&rel_path) {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("forge_math::linalg::") && is_code_line(line) {
                violations.push(format!("  {}:{}: {}", rel_path, line_num + 1, line.trim()));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: direct forge_math::linalg access in kernel.\n\
             Route through forge-geom facade instead.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

// ── Guard 2: No direct forge_math::predicates:: access ──────────────────

/// No direct `forge_math::predicates::` calls in non-test kernel code.
/// Predicates (orient3d, incircle) must be consumed through `forge-geom`
/// or `forge-spatial`, never called directly from the orchestration layer.
#[test]
fn no_forge_math_predicates_bypass() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rs_files = collect_rs_files(&kernel_src);

    let mut violations = Vec::new();

    for file in &rs_files {
        let rel_path = file
            .strip_prefix(&kernel_src)
            .unwrap_or(file)
            .to_string_lossy();
        let rel_path = normalize_rel_path(&rel_path);

        if is_exempt_path(&rel_path) {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("forge_math::predicates::") && is_code_line(line) {
                violations.push(format!("  {}:{}: {}", rel_path, line_num + 1, line.trim()));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: direct forge_math::predicates access in kernel.\n\
             Route through forge-geom or forge-spatial instead.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

// ── Guard 3: No ad-hoc floating-point math ──────────────────────────────

/// Bans ad-hoc f64 math methods in non-test kernel code.
/// These indicate someone is doing geometry inline instead of
/// through the proper forge-math → forge-geom pipeline.
///
/// Allowed:
/// - Test code (integration_tests/, proof/tests/, #[cfg(test)])
/// - Configuration code (tolerance scaling uses simple arithmetic)
/// - Comments and doc strings
#[test]
fn no_adhoc_float_math() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rs_files = collect_rs_files(&kernel_src);

    // Methods that are red flags for ad-hoc geometry in orchestration code.
    // .sqrt(), .sin(), .cos(), .atan2(), .powi() etc.
    let banned_methods = [
        ".sqrt()", ".sin()", ".cos()", ".tan()", ".asin()", ".acos()", ".atan(", ".atan2(",
        ".powi(", ".powf(", ".hypot(",
    ];

    let mut violations = Vec::new();

    for file in &rs_files {
        let rel_path = file
            .strip_prefix(&kernel_src)
            .unwrap_or(file)
            .to_string_lossy();
        let rel_path = normalize_rel_path(&rel_path);

        if is_exempt_path(&rel_path) {
            continue;
        }
        // Integration tests exercise production code, not orchestrators
        if rel_path.contains("integration_tests") {
            continue;
        }
        // Configuration tolerance scaling uses simple arithmetic
        if rel_path.contains("configuration/") {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Skip #[cfg(test)] module sections
        let mut in_test_cfg = false;
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed == "#[cfg(test)]" {
                in_test_cfg = true;
                continue;
            }
            if in_test_cfg {
                continue;
            }
            if !is_code_line(line) {
                continue;
            }

            for method in &banned_methods {
                if line.contains(method) {
                    violations.push(format!(
                        "  {}:{}: {} [banned: {}]",
                        rel_path,
                        line_num + 1,
                        trimmed,
                        method
                    ));
                }
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: ad-hoc floating-point math in kernel code.\n\
             These operations belong in forge-math or forge-geom.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

// ── Guard 4: No raw forge-signal evaluation bypass in kernel code ─────────

#[test]
fn no_raw_signal_runtime_bypass_outside_feature_tree() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rs_files = collect_rs_files(&kernel_src);

    let banned_patterns = [
        "use forge_signal::facade::evaluate",
        "use forge_signal::facade::mark_dirty",
        "forge_signal::facade::evaluate(",
        "forge_signal::facade::mark_dirty(",
        "evaluate(&mut graph",
        "mark_dirty(&mut graph",
    ];

    let mut violations = Vec::new();

    for file in &rs_files {
        let rel_path = file
            .strip_prefix(&kernel_src)
            .unwrap_or(file)
            .to_string_lossy();
        let rel_path = normalize_rel_path(&rel_path);

        if is_exempt_path(&rel_path) || is_signal_contract_path_allowed(&rel_path) {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if !is_code_line(line) {
                continue;
            }
            if banned_patterns.iter().any(|pattern| line.contains(pattern)) {
                violations.push(format!("  {}:{}: {}", rel_path, line_num + 1, line.trim()));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: raw forge-signal evaluation/invalidation bypass in kernel.\n\
             Route runtime work through FeatureTree/SignalRuntime instead.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

// ── Guard 5: Feature registration must expose signal policy explicitly ────

#[test]
fn feature_modules_define_explicit_signal_policy() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let required_files = [
        "operations/primitives/mod.rs",
        "operations/boolean/feature.rs",
    ];

    let mut violations = Vec::new();

    for rel_path in required_files {
        let file = kernel_src.join(rel_path);
        let content = std::fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("required file missing for guard: {}", rel_path));

        if !content.contains("fn signal_policy(&self) -> FeatureSignalPolicy") {
            violations.push(format!(
                "  {}: missing explicit signal_policy() override",
                rel_path
            ));
        }
        if !content.contains("FeatureSignalPolicy::core()") {
            violations.push(format!(
                "  {}: missing explicit FeatureSignalPolicy::core()",
                rel_path
            ));
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: feature modules must classify signal policy explicitly.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

// ── Guard 6: Feature graph wiring must use aspect-aware dependencies ──────

#[test]
fn feature_tree_uses_aspect_aware_dependency_bindings() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let file = kernel_src.join("engine/feature_tree.rs");
    let content = std::fs::read_to_string(&file)
        .expect("engine/feature_tree.rs must exist for signal contract guard");

    assert!(
        content.contains("feature.dependency_bindings()"),
        "FeatureTree must wire dependencies through dependency_bindings()"
    );
    assert!(
        !content.contains("feature.dependencies()"),
        "FeatureTree must not wire signal dependencies through legacy dependencies()"
    );
}

// ── Guard 7: Raw projected topology stays behind SpecEnvelope ─────────────

#[test]
fn raw_projected_topology_access_stays_inside_spec_envelope() {
    let kernel_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let rs_files = collect_rs_files(&kernel_src);

    let banned_patterns = [
        "ProjectedTopology",
        "ProjectionBuilder",
        "ProjectedTopologyQueries",
    ];

    let mut violations = Vec::new();

    for file in &rs_files {
        let rel_path = file
            .strip_prefix(&kernel_src)
            .unwrap_or(file)
            .to_string_lossy();
        let rel_path = normalize_rel_path(&rel_path);

        if is_exempt_path(&rel_path) || is_projection_contract_path_allowed(&rel_path) {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if !is_code_line(line) {
                continue;
            }
            if banned_patterns.iter().any(|pattern| line.contains(pattern)) {
                violations.push(format!("  {}:{}: {}", rel_path, line_num + 1, line.trim()));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: raw projected-topology access leaked outside SpecEnvelope.\n\
             Route projected reads through SpecEnvelope instead.\n\
             Violations:\n{}\n",
            violations.join("\n")
        );
    }
}

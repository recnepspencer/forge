//! Architectural boundary enforcement tests.
//!
//! DOMAIN: Compile-time-like checks that enforce layering rules.

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

/// No direct `forge_math::linalg::` calls allowed in non-deprecated kernel code.
/// Any allowed crossing point should be explicitly exempted in this test.
///
/// The `_deprecated/` directories are exempt since they are being phased out.
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

        if rel_path.contains("_deprecated") {
            continue;
        }
        // Legacy allowlist slot: keep this exemption if a dedicated geometry
        // facade file is reintroduced as the single crossing point.
        if rel_path.ends_with("geom_facade.rs") {
            continue;
        }
        if rel_path.contains("architecture_guard") {
            continue;
        }

        let content = match std::fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("forge_math::linalg::") && is_code_line(line) {
                violations.push(format!(
                    "  {}:{}: {}",
                    rel_path,
                    line_num + 1,
                    line.trim()
                ));
            }
        }
    }

    if !violations.is_empty() {
        panic!(
            "\n\nARCHITECTURE VIOLATION: direct forge_math::linalg access is not allowed in \
             non-deprecated kernel modules.\n\
             Violations found:\n{}\n\n\
             Fix: route calls through the approved kernel geometry abstraction layer.\n",
            violations.join("\n")
        );
    }
}

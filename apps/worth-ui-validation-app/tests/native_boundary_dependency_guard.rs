#[allow(dead_code)]
mod support;

use support::native_boundary_scanning::{line_declares_forge_query_dependency, strip_toml_comment};

#[test]
fn forge_query_dependency_guard_rejects_direct_renamed_and_target_specific_forms() {
    let hostile_lines = [
        "forge-query = { workspace = true }",
        "[dependencies.forge-query]",
        "[dev-dependencies.forge-query]",
        "[target.'cfg(windows)'.dependencies.forge-query]",
        "query = { package = \"forge-query\", workspace = true }",
        "query = { package=\"forge-query\", workspace = true }",
        "forge-query = { workspace = true } # even when followed by a comment",
    ];

    for line in hostile_lines {
        assert!(
            line_declares_forge_query_dependency(strip_toml_comment(line)),
            "guard missed direct Forge Query dependency form: {line}"
        );
    }

    let safe_lines = [
        "worth-ui = { workspace = true }",
        "egui = { workspace = true }",
        "# forge-query = { workspace = true }",
        "description = \"mentions forge-query in prose only\"",
    ];

    for line in safe_lines {
        assert!(
            !line_declares_forge_query_dependency(strip_toml_comment(line)),
            "guard rejected non-dependency manifest line: {line}"
        );
    }
}

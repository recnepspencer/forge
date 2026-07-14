//! Production-binary enforcement for universal worth-proof law-substrate admission.

mod authority_sealing_fixture;

use authority_sealing_fixture::AuthoritySealingTestRepository;

fn assert_bc7002(label: &str, config: &str) {
    let repo = AuthoritySealingTestRepository::create(label);
    repo.assemble_with_lib_source_and_config("pub fn seed() {}\n", config);
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(!ok, "{label} must fail law-substrate admission:\n{output}");
    assert!(
        output.contains("BC7002_LAW_SUBSTRATE_CONFIG"),
        "{label}: expected BC7002, got:\n{output}"
    );
}

fn fixture_config(law_substrates_toml: &str) -> String {
    // Temp path is unused by config builders; create/drop a throwaway root.
    let repo = AuthoritySealingTestRepository::create("config-builder");
    let config = repo.config_with_law_substrates(law_substrates_toml);
    repo.cleanup();
    config
}

#[test]
fn missing_worth_proof_row_is_denied_by_production_entrypoint() {
    assert_bc7002("missing-worth-proof", &fixture_config(""));
}

#[test]
fn partial_worth_proof_bands_is_denied_by_production_entrypoint() {
    // Deliberately omit `pack` from naming.bands set.
    assert_bc7002(
        "partial-bands",
        &fixture_config(
            r#"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["schema", "entry", "derived", "cert"]
"#,
        ),
    );
}

#[test]
fn partial_worth_proof_tiers_is_denied_by_production_entrypoint() {
    assert_bc7002(
        "partial-tiers",
        &fixture_config(
            r#"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth"]
bands = ["schema", "entry", "derived", "cert", "pack"]
"#,
        ),
    );
}

#[test]
fn duplicate_worth_proof_rows_are_denied_by_production_entrypoint() {
    assert_bc7002(
        "duplicate-worth-proof",
        &fixture_config(
            r#"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["schema", "entry", "derived", "cert", "pack"]

[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["schema", "entry", "derived", "cert", "pack"]
"#,
        ),
    );
}

#[test]
fn unadmitted_substrate_dependency_edge_is_denied() {
    let repo = AuthoritySealingTestRepository::create("unadmitted-edge");
    // Universal worth-proof row is complete; a second substrate is band-restricted.
    // Entry-band consumer depending on that substrate must fail closed at the edge.
    let config = repo.config_with_law_substrates(
        r#"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["schema", "entry", "derived", "cert", "pack"]

[[law_substrates]]
package = "restricted-substrate"
tiers = ["worth"]
bands = ["schema"]
"#,
    );
    repo.assemble_with_substrate_dependency("pub fn seed() {}\n", &config, "restricted-substrate");
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(!ok, "unadmitted substrate edge must fail:\n{output}");
    assert!(
        output.contains("BC7002_LAW_SUBSTRATE_CONFIG"),
        "expected BC7002 for unadmitted edge, got:\n{output}"
    );
    assert!(
        output.contains("restricted-substrate")
            && (output.contains("not admitted") || output.contains("tier")),
        "expected substrate edge denial message, got:\n{output}"
    );
}

#[test]
fn complete_universal_worth_proof_row_passes() {
    let repo = AuthoritySealingTestRepository::create("universal-ok");
    repo.assemble_with_lib_source("pub fn seed() {}\n");
    let (ok, output) = repo.run_boundary_check();
    repo.cleanup();
    assert!(ok, "complete universal substrate must pass:\n{output}");
    assert!(
        !output.contains("BC7002_LAW_SUBSTRATE_CONFIG"),
        "unexpected BC7002:\n{output}"
    );
}

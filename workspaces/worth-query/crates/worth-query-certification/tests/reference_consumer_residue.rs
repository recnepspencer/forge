use std::{fs, path::PathBuf};
use worth_query::facade::consumer_kit::{
    downstream_authority_adoption, worth_query_consumer_residue_certification_evidence,
};

#[test]
fn residue_detectors_reject_every_registered_hostile_shape_without_false_positives() {
    for evidence in worth_query_consumer_residue_certification_evidence() {
        assert!(
            evidence.satisfied(),
            "residue detector case {} is not trustworthy",
            evidence.case_id()
        );
    }
}

#[test]
fn worth_ui_production_consumers_have_no_competing_query_authority_residue() {
    let root = workspace_root();
    let proof = downstream_authority_adoption("worth-ui-reference-consumer")
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-query-binding/src"))
        .required_root(root.join("workspaces/worth-ui/crates/worth-ui-runtime/src"))
        .evaluate()
        .expect("reference consumer production sources must parse");

    proof.assert_adopted();
    assert!(proof.deletion_receipt().is_some());
    let report = proof.residue_report();
    for required_suffix in [
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/lib.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/lib.rs",
    ] {
        assert!(
            report
                .audited_source_paths()
                .iter()
                .any(|path| path.ends_with(required_suffix)),
            "the production audit omitted its required source root sentinel {required_suffix}"
        );
    }
}

#[test]
fn retired_graph_obligation_authority_has_one_destination_and_exact_residue() {
    let root = workspace_root();
    let query = root.join("workspaces/worth-query/crates/worth-query");
    let host = root.join("workspaces/worth-query/crates/worth-query-host");
    for retired in [
        query.join("src/runtime/mutation/graph_composition/obligation"),
        query.join("src/consumer_kit/graph_obligation_adoption"),
        query.join("docs/authoring/graph-obligation-consumer-kit.md"),
    ] {
        assert!(
            !retired.exists(),
            "retired authority remains at {}",
            retired.display()
        );
    }
    for destination in [
        root.join("workspaces/worth-query/crates/worth-query-installation/src/graph_obligation"),
        root.join("workspaces/worth-query/crates/worth-query-installation/src/graph_obligation/adoption.rs"),
        query.join("docs/domain-capabilities/canonical-graph-obligation-progression.md"),
    ] {
        assert!(destination.exists(), "canonical destination missing at {}", destination.display());
    }

    let monolith_source = rust_source_under(query.join("src"));
    for retired_symbol in [
        "WorthQueryGraphObligationIndex",
        "WorthQueryGraphObligationExecutionInput",
        "WorthQueryGraphObligationExecutionResultEnvelope",
        "WorthQueryGraphObligationConsumerKit",
        "WorthQueryGraphObligationOrchestrationDispatch",
        "compose_graph_with_invariant_pack",
        "compose_read_with_invariant_pack",
        "define_read_family_with_invariant_pack",
    ] {
        assert!(
            !monolith_source.contains(retired_symbol),
            "retired monolith authority `{retired_symbol}` remains in production source"
        );
    }

    let host_facade = fs::read_to_string(host.join("src/facade.rs"))
        .expect("host facade source should remain readable");
    assert_eq!(
        host_facade
            .matches("inspect_installed_graph_obligations")
            .count(),
        1,
        "the host must expose exactly one read-only obligation adoption path"
    );
    assert!(
        !host.join("src/graph_obligation_adoption.rs").exists(),
        "the re-export-only host audience must not own adoption behavior"
    );

    for package in [
        "worth-query-installation",
        "worth-query-admission",
        "worth-query-execution",
        "worth-query-publication",
        "worth-query-host",
    ] {
        let manifest_path = root
            .join("workspaces/worth-query/crates")
            .join(package)
            .join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path)
            .unwrap_or_else(|error| panic!("{} should read: {error}", manifest_path.display()));
        assert!(
            !manifest.contains("\nworth-query =")
                && !manifest.contains("\n[dependencies.worth-query]"),
            "destination package `{package}` must not depend on the monolith"
        );
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("certification crate remains under workspaces/worth-query/crates")
        .to_path_buf()
}

fn rust_source_under(root: PathBuf) -> String {
    let mut pending = vec![root];
    let mut source = String::new();
    while let Some(path) = pending.pop() {
        let mut entries = fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("{} should read: {error}", path.display()))
            .collect::<Result<Vec<_>, _>>()
            .expect("source directory entries should read");
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                source.push_str(
                    &fs::read_to_string(&path)
                        .unwrap_or_else(|error| panic!("{} should read: {error}", path.display())),
                );
            }
        }
    }
    source
}

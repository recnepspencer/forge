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

#[test]
fn application_disclosure_has_one_owner_and_publication_has_no_policy_lane() {
    let root = workspace_root();
    let execution =
        root.join("workspaces/worth-query/crates/worth-query-execution/src/domain_computation");
    let authorization = execution.join("authorization");
    let application_query = execution.join("primary_graph/application_query");
    let publication_root =
        root.join("workspaces/worth-query/crates/worth-query-publication/src/domain_computation");

    let receipt_sources = format!(
        "{}\n{}",
        rust_source_under(authorization.clone()),
        rust_source_under(application_query)
    );
    assert_eq!(
        receipt_sources
            .matches("pub struct WorthQueryApplicationDisclosureReceipt {")
            .count(),
        1,
        "the terminal disclosure receipt must have one authorization-owned definition"
    );
    assert!(
        !rust_source_under(authorization).contains("primary_graph::application_query"),
        "authorization must not import its application-query audience"
    );

    let publication_source = format!(
        "{}\n{}",
        production_source_without_docs(publication_root.join("application_result.rs")),
        production_source_without_docs(publication_root.join("application_result")),
    );
    for forbidden in [
        "AspectValue",
        "ProjectionMask",
        "DiagnosticMask",
        "redact",
        "classification",
        "disclosure_value",
    ] {
        assert!(
            !publication_source.contains(forbidden),
            "publication must not own protected-value or disclosure-policy token `{forbidden}`"
        );
    }
    assert_eq!(
        publication_source
            .matches("WorthQueryAdmittedDisclosedApplicationResult")
            .count(),
        2,
        "publication imports and accepts only the admitted shape, then stores publication-owned rows"
    );
}

#[test]
fn conditional_host_surface_has_no_lower_runtime_or_replacement_lane() {
    let root = workspace_root();
    let host = root.join("workspaces/worth-query/crates/worth-query-host");
    let manifest =
        fs::read_to_string(host.join("Cargo.toml")).expect("host manifest should remain readable");
    let source = rust_source_under(host.join("src"));

    for forbidden_dependency in ["worth-signal", "worth-runtime-bridge", "worth-relational"] {
        assert!(
            !manifest.contains(forbidden_dependency),
            "host manifest exposes forbidden lower runtime `{forbidden_dependency}`"
        );
    }
    for forbidden_surface in [
        "SignalGraph",
        "BridgeConditionalProviderSet",
        "replace_conditional_provider",
        "replace_named_clock",
        "schedule_temporal_wake",
        "invoke_conditional_operation",
    ] {
        assert!(
            !source.contains(forbidden_surface),
            "host facade exposes forbidden conditional surface `{forbidden_surface}`"
        );
    }
}

#[test]
fn phase_ten_contracts_the_audience_facades_and_documentation_route() {
    let root = workspace_root();
    let snapshots = fs::read_to_string(root.join("tools/boundary-check/snapshots/facades.toml"))
        .expect("facade snapshot should remain readable");
    for required in [
        "package = \"worth-query-decl\"",
        "package = \"worth-query-host\"",
        "package = \"worth-query-host::primary_graph\"",
        "package = \"worth-query-host::provisional_aftermath\"",
        "\"worth_query_conditional_node\"",
        "\"WorthQueryConditionalClockObservationReceipt\"",
    ] {
        assert!(
            snapshots.contains(required),
            "contracted facade snapshot omitted `{required}`"
        );
    }

    let bank_certification = fs::read_to_string(
        root.join("workspaces/worth-query-bank-world/crates/bank-estate-certification/src/lib.rs"),
    )
    .expect("Bank certification crate root should remain readable");
    assert!(
        bank_certification.contains("include_str!(")
            && bank_certification.contains("ordinary-application-front-door.md"),
        "Bank certification must doctest the canonical Markdown itself"
    );

    let query_docs = root.join("workspaces/worth-query/crates/worth-query/docs");
    let front_door =
        fs::read_to_string(query_docs.join("foundations/ordinary-application-front-door.md"))
            .expect("ordinary front-door guide should remain readable");
    for required in [
        "worth_query_decl::facade",
        "worth_query_host::facade",
        "worth_query_replay::facade",
        "Conditional providers and managed clocks are stable",
        "Linear undo and redo remain provisional experiments",
        "BankReadControls::current(request_scope, 32, 20_000)",
    ] {
        assert!(
            front_door.contains(required),
            "ordinary front-door guide omitted `{required}`"
        );
    }

    let index = fs::read_to_string(query_docs.join("README.md"))
        .expect("documentation index should remain readable");
    let orientation = fs::read_to_string(query_docs.join("AI_README.md"))
        .expect("AI orientation should remain readable");
    let host =
        fs::read_to_string(root.join("workspaces/worth-query/crates/worth-query-host/README.md"))
            .expect("host README should remain readable");
    let declaration =
        fs::read_to_string(root.join("workspaces/worth-query/crates/worth-query-decl/README.md"))
            .expect("declaration README should remain readable");
    for (name, document) in [
        ("documentation index", index),
        ("AI orientation", orientation),
        ("host README", host),
        ("declaration README", declaration),
    ] {
        assert!(
            document.contains("Ordinary Application Front Door"),
            "{name} must route readers to the ordinary front door"
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
    if root.is_file() {
        return fs::read_to_string(&root)
            .unwrap_or_else(|error| panic!("{} should read: {error}", root.display()));
    }
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

fn production_source_without_docs(root: PathBuf) -> String {
    rust_source_under(root)
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            !line.starts_with("///") && !line.starts_with("//!")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

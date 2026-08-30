use std::fs;
use std::path::PathBuf;

use serde_yaml_ng::{Mapping, Value};

const WORKFLOW: &str = ".github/workflows/publish-worth-query-release.yml";

#[test]
fn reusable_surface_is_narrow_and_bound_to_the_protected_environment() {
    let root = parsed_workflow();
    let root = mapping(&root);
    let triggers = mapping(required(root, "on"));
    assert_eq!(triggers.len(), 1);
    let call = mapping(required(triggers, "workflow_call"));
    assert!(!call.contains_key(Value::String("secrets".to_owned())));

    let inputs = mapping(required(call, "inputs"));
    assert_eq!(
        inputs.keys().map(string).collect::<Vec<_>>(),
        [
            "expected_package_identity",
            "release_name",
            "release_version"
        ]
    );
    let permissions = mapping(required(root, "permissions"));
    assert_eq!(string(required(permissions, "actions")), "read");
    assert_eq!(string(required(permissions, "contents")), "write");
    assert_eq!(permissions.len(), 2);

    let publish = mapping(required(mapping(required(root, "jobs")), "publish"));
    assert_eq!(
        string(required(mapping(required(publish, "environment")), "name")),
        "worth-query-release"
    );
    assert_eq!(integer(required(publish, "timeout-minutes")), 20);
    let concurrency = mapping(required(publish, "concurrency"));
    assert!(!boolean(required(concurrency, "cancel-in-progress")));

    let job_env = mapping(required(publish, "env"));
    assert_eq!(
        string(required(job_env, "SIGNING_PAYLOAD_ARTIFACT")),
        "worth-query-release-signing-payload"
    );
    assert_eq!(
        string(required(job_env, "EXPECTED_SIGNATURE_PROTOCOL_IDENTITY")),
        "worth.release.ed25519"
    );
    assert_eq!(string(required(job_env, "EXPECTED_SIGNATURE_BYTES")), "64");
}

#[test]
fn execution_order_pins_actions_and_admits_only_the_exact_tagged_context() {
    let root = parsed_workflow();
    let publish = publish_job(&root);
    let steps = sequence(required(publish, "steps"));
    let names = steps
        .iter()
        .map(|step| string(required(mapping(step), "name")))
        .collect::<Vec<_>>();
    assert_ordered(
        &names,
        &[
            "Admit the protected same-repository tag context",
            "Build the release ceremony before exposing signing secrets",
            "Preflight and stage the exact bytes permitted to reach the signer",
            "Sign and independently verify the admitted payload",
            "Finalize and freshly readmit the signed envelope",
            "Stage the complete human GitHub release as a draft",
            "Publish the complete human GitHub release without latest-name authority",
        ],
    );

    for step in steps {
        let step = mapping(step);
        if let Some(action) = optional(step, "uses") {
            assert_full_commit_action_reference(string(action));
        }
        if let Some(run) = optional(step, "run") {
            assert!(
                !string(run).contains("${{"),
                "untrusted workflow expressions must enter shell through step env"
            );
        }
    }

    let context = named_step(steps, "Admit the protected same-repository tag context");
    let context_run = string(required(context, "run"));
    for required_check in [
        "GITHUB_REPOSITORY\" = \"$workflow_repository",
        "GITHUB_SHA\" = \"$workflow_revision",
        "workflow_file\" = \".github/workflows/publish-worth-query-release.yml",
        "GITHUB_REF_TYPE\" = \"tag",
        "GITHUB_REF_PROTECTED\" = \"true",
    ] {
        assert!(context_run.contains(required_check));
    }
    let context_env = mapping(required(context, "env"));
    assert!(string(required(context_env, "JOB_CONTEXT_JSON")).contains("toJSON(job)"));
}

#[test]
fn artifact_intake_and_signature_shape_are_bounded_before_publication() {
    let root = parsed_workflow();
    let publish = publish_job(&root);
    let steps = sequence(required(publish, "steps"));
    let metadata = named_step(steps, "Admit the bounded same-run artifact metadata");
    let metadata_run = string(required(metadata, "run"));
    for contract in [
        "actions/runs/$GITHUB_RUN_ID/artifacts",
        ".expired == false",
        "test \"$matches\" = \"1\"",
        "test \"$artifact_bytes\" -le \"$MAXIMUM_ARTIFACT_BYTES\"",
    ] {
        assert!(metadata_run.contains(contract));
    }

    let download = named_step(steps, "Download the fixed signing-payload artifact");
    assert_eq!(
        string(required(mapping(required(download, "with")), "name")),
        "worth-query-release-signing-payload"
    );
    let inventory = named_step(steps, "Require the exact artifact file inventory");
    let inventory_run = string(required(inventory, "run"));
    assert!(inventory_run.contains("test \"${#entries[@]}\" -eq 1"));
    assert!(inventory_run.contains("test -f \"$payload\""));
    assert!(inventory_run.contains("test ! -L \"$payload\""));

    let preflight = named_step(
        steps,
        "Preflight and stage the exact bytes permitted to reach the signer",
    );
    let finalize = named_step(steps, "Finalize and freshly readmit the signed envelope");
    for step in [preflight, finalize] {
        assert!(string(required(step, "run")).contains("--expected-signature-bytes"));
    }
}

#[test]
fn signing_secrets_are_isolated_and_publication_grants_no_latest_authority() {
    let source = workflow_source();
    let root: Value = serde_yaml_ng::from_str(&source).unwrap();
    let publish = publish_job(&root);
    let steps = sequence(required(publish, "steps"));
    let sign = named_step(steps, "Sign and independently verify the admitted payload");
    let sign_env = mapping(required(sign, "env"));
    assert_eq!(sign_env.len(), 2);
    assert!(string(required(sign_env, "PRIVATE_KEY_PEM"))
        .contains("WORTH_QUERY_RELEASE_ED25519_PRIVATE_KEY_PEM"));
    assert!(string(required(sign_env, "PUBLIC_KEY_PEM"))
        .contains("WORTH_QUERY_RELEASE_ED25519_PUBLIC_KEY_PEM"));
    let sign_run = string(required(sign, "run"));
    assert!(sign_run.contains("pkeyutl -sign -rawin"));
    assert!(sign_run.contains("pkeyutl -verify -rawin -pubin"));
    assert!(sign_run.contains("wc -c"));
    assert!(sign_run.contains("-eq 64"));
    for step in steps {
        let serialized = serde_yaml_ng::to_string(step).unwrap();
        if string(required(mapping(step), "name"))
            != "Sign and independently verify the admitted payload"
        {
            assert!(!serialized.contains("WORTH_QUERY_RELEASE_ED25519_PRIVATE_KEY_PEM"));
            assert!(!serialized.contains("WORTH_QUERY_RELEASE_ED25519_PUBLIC_KEY_PEM"));
        }
    }

    let draft_step = named_step(steps, "Stage the complete human GitHub release as a draft");
    let draft_run = string(required(draft_step, "run"));
    assert!(draft_run.contains("--draft"));
    assert!(draft_run.contains("--verify-tag"));
    assert!(draft_run.contains("--latest=false"));
    assert!(!draft_run.contains("--clobber"));
    let publish_step = named_step(
        steps,
        "Publish the complete human GitHub release without latest-name authority",
    );
    let publish_run = string(required(publish_step, "run"));
    assert!(publish_run.contains("--draft=false"));
    assert!(publish_run.contains("--latest=false"));
    assert!(!publish_run.contains("--clobber"));
    assert!(!source.contains("workflow_run:"));
    assert!(!source.contains("workflow_dispatch:"));
}

fn parsed_workflow() -> Value {
    serde_yaml_ng::from_str(&workflow_source()).unwrap()
}

fn publish_job(root: &Value) -> &Mapping {
    mapping(required(
        mapping(required(mapping(root), "jobs")),
        "publish",
    ))
}

fn workflow_source() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    fs::read_to_string(root.join(WORKFLOW)).unwrap()
}

fn named_step<'a>(steps: &'a [Value], expected: &str) -> &'a Mapping {
    steps
        .iter()
        .map(mapping)
        .find(|step| string(required(step, "name")) == expected)
        .unwrap_or_else(|| panic!("missing workflow step: {expected}"))
}

fn assert_ordered(observed: &[&str], required_names: &[&str]) {
    let mut previous = None;
    for required_name in required_names {
        let position = observed
            .iter()
            .position(|name| name == required_name)
            .unwrap_or_else(|| panic!("missing workflow step: {required_name}"));
        if let Some(previous) = previous {
            assert!(position > previous, "workflow step is out of order");
        }
        previous = Some(position);
    }
}

fn assert_full_commit_action_reference(reference: &str) {
    let (_, revision) = reference
        .rsplit_once('@')
        .unwrap_or_else(|| panic!("external action is not pinned: {reference}"));
    assert_eq!(revision.len(), 40, "action revision is not a full SHA");
    assert!(
        revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "action revision is not hexadecimal"
    );
}

fn required<'a>(mapping: &'a Mapping, key: &str) -> &'a Value {
    optional(mapping, key).unwrap_or_else(|| panic!("missing YAML key: {key}"))
}

fn optional<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a Value> {
    mapping.get(Value::String(key.to_owned()))
}

fn mapping(value: &Value) -> &Mapping {
    value.as_mapping().expect("expected YAML mapping")
}

fn sequence(value: &Value) -> &[Value] {
    value.as_sequence().expect("expected YAML sequence")
}

fn string(value: &Value) -> &str {
    value.as_str().expect("expected YAML string")
}

fn integer(value: &Value) -> i64 {
    value.as_i64().expect("expected YAML integer")
}

fn boolean(value: &Value) -> bool {
    value.as_bool().expect("expected YAML boolean")
}

use super::{value_row, AuthoritySealingTestRepository, VALUE_GOVERNED};

#[test]
fn generated_consumer_forbids_macro_wrapped_unsafe_witnesses() {
    let output = denied_output(
        "macro-unsafe-witness",
        r#"
macro_rules! manufacture {
    () => {{ unsafe { core::mem::zeroed() } }};
}
pub(crate) fn sealed() -> worth_proof::Sealed { manufacture!() }
"#,
        None,
        None,
    );
    assert!(output.contains("usage of an `unsafe` block"), "{output}");
}

#[test]
fn host_witness_runtime_has_a_bounded_deadline() {
    let output = denied_output(
        "bounded-witness-runtime",
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {
    std::thread::sleep(std::time::Duration::from_millis(750));
    worth_proof::issue()
}
"#,
        Some(100),
        None,
    );
    assert!(
        output.contains("public-value witness runtime timed out after 100 ms"),
        "blocking witness failed for the wrong reason:\n{output}"
    );
    assert!(output.contains("stdout:\n") && output.contains("stderr:\n"));
}

#[test]
fn nonterminating_witness_is_denied_by_the_runtime_deadline() {
    let output = denied_output(
        "nonterm-runtime",
        "pub(crate) fn sealed() -> worth_proof::Sealed { loop {} }",
        Some(100),
        None,
    );
    assert!(
        output.contains("public-value witness runtime timed out after 100 ms"),
        "nonterminating witness failed for the wrong reason:\n{output}"
    );
}

#[test]
fn finite_high_volume_output_is_denied_at_the_configured_cap() {
    let output = denied_output(
        "finite-output-overflow",
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {
    print!("{}", "x".repeat(131072));
    worth_proof::issue()
}
"#,
        None,
        Some(32768),
    );
    assert!(
        output.contains("exceeded configured output limit of 32768 bytes"),
        "high-volume witness failed for the wrong reason:\n{output}"
    );
    assert!(output.contains("captured output was truncated"), "{output}");
}

#[test]
fn runtime_timeout_terminates_descendants_that_hold_captured_stdout() {
    let repository = AuthoritySealingTestRepository::create("process-tree-output-holder");
    let marker = repository.public_value_fixture_path("descendant-survived");
    let marker_source = marker.to_string_lossy().replace('\\', "/");
    let witness = format!(
        r#"
pub(crate) fn sealed() -> worth_proof::Sealed {{
    #[cfg(windows)]
    let _descendant = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", "$until=(Get-Date).AddMilliseconds(600); while((Get-Date)-lt $until){{[Console]::Out.Write('x'); Start-Sleep -Milliseconds 1}}; Set-Content -LiteralPath '{marker_source}' -Value survived"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("spawn output-holding descendant");
    #[cfg(unix)]
    let _descendant = std::process::Command::new("sh")
        .args(["-c", "i=0; while [ $i -lt 600 ]; do printf x; sleep 0.001; i=$((i+1)); done; echo survived >'{marker_source}'"])
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("spawn output-holding descendant");
    worth_proof::issue()
}}
"#
    );
    repository.assemble_public_value_witness_contract(VALUE_GOVERNED, &witness, value_row(), "");
    repository.replace_public_value_config("host_timeout_ms = 30000", "host_timeout_ms = 100");
    let (ok, output) = repository.run_boundary_check();
    std::thread::sleep(std::time::Duration::from_millis(800));
    let descendant_survived = marker.exists();
    repository.cleanup();
    assert!(!ok, "output-holding descendant must time out:\n{output}");
    assert!(
        output.contains("public-value witness runtime timed out after 100 ms"),
        "descendant fixture failed for the wrong reason:\n{output}"
    );
    assert!(
        !descendant_survived,
        "process-tree timeout left its descendant alive"
    );
}

fn denied_output(
    label: &str,
    witness: &str,
    timeout_ms: Option<u64>,
    max_output_bytes: Option<usize>,
) -> String {
    let repository = AuthoritySealingTestRepository::create(label);
    repository.assemble_public_value_witness_contract(VALUE_GOVERNED, witness, value_row(), "");
    if let Some(timeout_ms) = timeout_ms {
        repository.replace_public_value_config(
            "host_timeout_ms = 30000",
            &format!("host_timeout_ms = {timeout_ms}"),
        );
    }
    if let Some(max_output_bytes) = max_output_bytes {
        repository.replace_public_value_config(
            "max_output_bytes = 65536",
            &format!("max_output_bytes = {max_output_bytes}"),
        );
    }
    let (ok, output) = repository.run_boundary_check();
    repository.cleanup();
    assert!(!ok, "{label} must fail closed:\n{output}");
    output
}

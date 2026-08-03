use std::path::{Path, PathBuf};

use super::workspace_source::{read, workspace_relative};
use crate::workspace_root;

const QUEUE_ROOT: &str = "crates/worth-store-io-scheduler/src/queue_execution";

#[test]
fn queue_authority_stages_are_move_owned() {
    for (relative, type_name) in [
        ("policy/work_declaration.rs", "QueueWorkDeclaration"),
        ("admission/policy_receipt.rs", "QueuePolicyAdmissionReceipt"),
        ("admission/request.rs", "QueueExecutionAdmissionRequest"),
        ("execution/plan.rs", "AdmittedQueueExecutionPlan"),
        ("execution/plan.rs", "QueueExecutionReadyPlan"),
    ] {
        let path = queue_path(relative);
        let source = read(&path).expect("read queue authority source");
        reject_clone_or_copy(&path, &source, type_name).unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn queue_progression_consumes_each_prior_authority_stage() {
    let policy_path = queue_path("admission/policy_receipt.rs");
    let policy = read(&policy_path).expect("read queue policy receipt");
    inspect_policy_receipt(&policy_path, &policy).unwrap_or_else(|denial| panic!("{denial}"));

    let request_path = queue_path("admission/request.rs");
    let request = read(&request_path).expect("read queue admission request");
    inspect_admission_request(&request_path, &request).unwrap_or_else(|denial| panic!("{denial}"));

    let plan_path = queue_path("execution/plan.rs");
    let plan = read(&plan_path).expect("read queue execution plan");
    inspect_ready_plan(&plan_path, &plan).unwrap_or_else(|denial| panic!("{denial}"));

    let execution_path = queue_path("execution/backend_completion.rs");
    let execution = read(&execution_path).expect("read queue execution entry");
    inspect_execution_entry(&execution_path, &execution)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn queue_policy_gate_rejects_clone_borrow_and_parallel_work_mutants() {
    let policy_path = queue_path("admission/policy_receipt.rs");
    let policy = read(&policy_path).expect("read queue policy receipt");
    let cloneable = mutate_once(&policy, "#[derive(Debug)]", "#[derive(Clone, Debug)]");
    expect_denial(
        inspect_policy_receipt(&policy_path, &cloneable),
        "cloneable policy receipt",
    );
    let borrowed = mutate_once(
        &policy,
        "work: QueueWorkDeclaration,",
        "work: &'static QueueWorkDeclaration,",
    );
    expect_denial(
        inspect_policy_receipt(&policy_path, &borrowed),
        "borrowed work declaration",
    );
    let parallel = mutate_once(
        &policy,
        "work: QueueWorkDeclaration,",
        "work: QueueWorkDeclaration,\n    copied_work: QueueWorkDeclaration,",
    );
    expect_denial(
        inspect_policy_receipt(&policy_path, &parallel),
        "parallel work declaration",
    );
}

#[test]
fn queue_progression_gate_rejects_borrowed_or_cloneable_authority_mutants() {
    let request_path = queue_path("admission/request.rs");
    let request = read(&request_path).expect("read queue admission request");
    let borrowed_policy = mutate_once(
        &request,
        "policy_receipt: QueuePolicyAdmissionReceipt,",
        "policy_receipt: &'a QueuePolicyAdmissionReceipt,",
    );
    expect_denial(
        inspect_admission_request(&request_path, &borrowed_policy),
        "borrowed policy receipt",
    );

    let plan_path = queue_path("execution/plan.rs");
    let plan = read(&plan_path).expect("read queue execution plan");
    let cloneable_ready = mutate_once(
        &plan,
        "pub struct QueueExecutionReadyPlan",
        "#[derive(Clone)]\npub struct QueueExecutionReadyPlan",
    );
    expect_denial(
        inspect_ready_plan(&plan_path, &cloneable_ready),
        "cloneable ready plan",
    );
    let flattened_ready = mutate_once(
        &plan,
        "admitted: AdmittedQueueExecutionPlan,",
        "work: QueueWorkDeclaration,",
    );
    expect_denial(
        inspect_ready_plan(&plan_path, &flattened_ready),
        "ready plan without exact admitted plan",
    );

    let execution_path = queue_path("execution/backend_completion.rs");
    let execution = read(&execution_path).expect("read queue execution entry");
    let borrowed_ready = mutate_once(
        &execution,
        "plan: QueueExecutionReadyPlan,",
        "plan: &QueueExecutionReadyPlan,",
    );
    expect_denial(
        inspect_execution_entry(&execution_path, &borrowed_ready),
        "borrowed ready plan execution",
    );
}

fn inspect_policy_receipt(path: &Path, source: &str) -> Result<(), String> {
    reject_clone_or_copy(path, source, "QueuePolicyAdmissionReceipt")?;
    require_normalized(
        path,
        source,
        "pub struct QueuePolicyAdmissionReceipt { work: QueueWorkDeclaration, foundational: FoundationalPolicyAdmissionReceipt, }",
        "policy receipt must own exactly one work declaration",
    )?;
    require_normalized(
        path,
        source,
        "pub fn admit_queue_policy_receipt( work: QueueWorkDeclaration,",
        "policy admission must consume its work declaration",
    )
}

fn inspect_admission_request(path: &Path, source: &str) -> Result<(), String> {
    reject_clone_or_copy(path, source, "QueueExecutionAdmissionRequest")?;
    require_normalized(
        path,
        source,
        "pub struct QueueExecutionAdmissionRequest<'a> { backend: &'a IoSchedulerBackendCapabilityAdmission, policy_receipt: QueuePolicyAdmissionReceipt, }",
        "execution request must borrow backend capability and own one policy receipt",
    )?;
    require_normalized(
        path,
        source,
        "pub fn admit_queue_execution_plan( request: QueueExecutionAdmissionRequest<'_>,",
        "queue admission must consume its execution request",
    )
}

fn inspect_ready_plan(path: &Path, source: &str) -> Result<(), String> {
    reject_clone_or_copy(path, source, "AdmittedQueueExecutionPlan")?;
    reject_clone_or_copy(path, source, "QueueExecutionReadyPlan")?;
    require_normalized(
        path,
        source,
        "pub struct QueueExecutionReadyPlan { admitted: AdmittedQueueExecutionPlan, progression: QueueExecutionProgression, }",
        "ready plan must own the exact admitted plan",
    )
}

fn inspect_execution_entry(path: &Path, source: &str) -> Result<(), String> {
    require_normalized(
        path,
        source,
        "pub fn execute_ready_queue_plan( plan: QueueExecutionReadyPlan,",
        "ready queue plan must be consumed by execution",
    )
}

fn reject_clone_or_copy(path: &Path, source: &str, type_name: &str) -> Result<(), String> {
    let marker = format!("pub struct {type_name}");
    let offset = source
        .find(&marker)
        .ok_or_else(|| denial(path, "required queue type is absent"))?;
    let preceding = &source[..offset];
    let derive_offset = preceding
        .rfind("#[derive(")
        .ok_or_else(|| denial(path, "queue type has no auditable derive list"))?;
    let derive_and_gap = &preceding[derive_offset..];
    let derive_end = derive_and_gap
        .find(")]")
        .map(|end| end + 2)
        .ok_or_else(|| denial(path, "queue type has an incomplete derive list"))?;
    if !derive_and_gap[derive_end..].trim().is_empty() {
        return Err(denial(
            path,
            "queue type derive list is not adjacent to its declaration",
        ));
    }
    let derive = &derive_and_gap[..derive_end];
    if derive.contains("Clone") || derive.contains("Copy") {
        return Err(denial(
            path,
            &format!("`{type_name}` must remain move-owned"),
        ));
    }
    Ok(())
}

fn require_normalized(
    path: &Path,
    source: &str,
    required: &str,
    reason: &str,
) -> Result<(), String> {
    let normalized = source.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.contains(required) {
        Ok(())
    } else {
        Err(denial(path, reason))
    }
}

fn expect_denial(result: Result<(), String>, mutant: &str) {
    let denial = match result {
        Ok(()) => panic!("{mutant} must be denied"),
        Err(denial) => denial,
    };
    assert!(denial.contains("queue authority boundary"), "{denial}");
}

fn mutate_once(source: &str, from: &str, to: &str) -> String {
    let mutated = source.replacen(from, to, 1);
    assert_ne!(mutated, source, "mutant source must change");
    mutated
}

fn denial(path: &Path, reason: &str) -> String {
    format!(
        "Phase 8 queue authority boundary: {reason} at {}",
        workspace_relative(path)
    )
}

fn queue_path(relative: &str) -> PathBuf {
    workspace_root().join(QUEUE_ROOT).join(relative)
}

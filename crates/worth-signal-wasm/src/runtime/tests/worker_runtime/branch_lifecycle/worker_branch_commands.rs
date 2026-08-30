use crate::expression::model::SignalValue;
use crate::runtime::tests::support::{RuntimeCore, RuntimePolicySpec};
use crate::runtime::tests::worker_runtime::fixtures::portable_counter_graph::{
    define_portable_counter_graph, set_counter, worker_shell_with_counter_graph,
};
use crate::runtime::worker_host::{
    WorkerApplyTransactionToBranchRequest, WorkerBranchRetirementReason, WorkerForkBranchRequest,
    WorkerRetireBranchRequest,
};

const WORKER_BRIDGE_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/package-src/product/entrypoint/bridge/worker_runtime_bridge_worker.ts"
));

#[test]
fn worker_branch_commands_isolate_ten_siblings_and_preserve_authored_values() {
    let mut shell = worker_shell_with_counter_graph();
    let main = shell.current_branch();
    let main_basis = shell.worker_branch_basis(main.id.0).unwrap();
    let mut branches = Vec::new();

    for index in 0..10_u64 {
        let fork = shell
            .fork_worker_branch(WorkerForkBranchRequest {
                name: format!("effect-{index}"),
                parent_branch_id: main.id.0,
                expected_parent_basis: main_basis.clone(),
            })
            .unwrap();
        let applied = shell
            .apply_transaction_to_worker_branch(WorkerApplyTransactionToBranchRequest {
                branch_id: fork.branch.id.0,
                expected_basis: fork.created_basis,
                transaction_ops: set_counter((index + 10) as f64),
            })
            .unwrap();
        assert_eq!(applied.active_branch_id_before, main.id.0);
        assert_eq!(applied.active_branch_id_after, main.id.0);
        assert_eq!(
            applied.after_basis.authored_graph_generation,
            applied.before_basis.authored_graph_generation + 1
        );
        assert_eq!(
            applied.after_basis.native_head_generation,
            applied.before_basis.native_head_generation + 1
        );
        branches.push((fork.branch, applied));
    }

    assert_eq!(shell.current_branch().id, main.id);
    assert_eq!(
        shell.read_value("counter").unwrap(),
        SignalValue::Number(1.0)
    );

    let stale = shell
        .apply_transaction_to_worker_branch(WorkerApplyTransactionToBranchRequest {
            branch_id: branches[0].0.id.0,
            expected_basis: branches[0].1.before_basis.clone(),
            transaction_ops: set_counter(999.0),
        })
        .unwrap_err();
    assert!(stale.message.contains("stale worker branch basis"));

    for (index, (branch, _)) in branches.iter().enumerate() {
        shell.switch_branch(branch.id.0).unwrap();
        assert_eq!(
            shell.read_value("doubleCounter").unwrap(),
            SignalValue::Number(((index as u64 + 10) * 2) as f64)
        );
    }
    shell.switch_branch(main.id.0).unwrap();

    for (branch, _) in branches {
        let _retained_snapshot = shell.branch_snapshot(branch.id.0).unwrap();
        let retirement_basis = shell.worker_branch_basis(branch.id.0).unwrap();
        let retired = shell
            .retire_worker_branch(WorkerRetireBranchRequest {
                branch_id: branch.id.0,
                expected_basis: retirement_basis,
                reason: WorkerBranchRetirementReason::Rejected,
            })
            .unwrap();
        assert_eq!(retired.retired_branch_id, branch.id.0);
        assert_eq!(retired.reclaimed_branch_state_count, 1);
        assert_eq!(retired.retained_proof_record_count, 1);
    }
    assert_eq!(shell.branches(), vec![main]);
}

#[test]
fn worker_targeted_commands_match_compatibility_branch_truth() {
    let mut worker = worker_shell_with_counter_graph();
    let mut compatibility = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    define_portable_counter_graph(&mut compatibility);
    let worker_main = worker.current_branch();
    let compatibility_main = compatibility.current_branch();
    let worker_main_basis = worker.worker_branch_basis(worker_main.id.0).unwrap();
    let worker_fork = worker
        .fork_worker_branch(WorkerForkBranchRequest {
            name: "parity".to_owned(),
            parent_branch_id: worker_main.id.0,
            expected_parent_basis: worker_main_basis,
        })
        .unwrap();
    let compatibility_fork = compatibility.create_branch("parity".to_owned()).unwrap();

    let transaction = set_counter(37.0);
    worker
        .apply_transaction_to_worker_branch(WorkerApplyTransactionToBranchRequest {
            branch_id: worker_fork.branch.id.0,
            expected_basis: worker_fork.created_basis,
            transaction_ops: transaction.clone(),
        })
        .unwrap();
    compatibility
        .switch_branch(compatibility_fork.id.0)
        .unwrap();
    compatibility.apply_transaction(transaction).unwrap();
    compatibility
        .switch_branch(compatibility_main.id.0)
        .unwrap();

    assert_eq!(worker.current_branch().id, worker_main.id);
    worker.switch_branch(worker_fork.branch.id.0).unwrap();
    compatibility
        .switch_branch(compatibility_fork.id.0)
        .unwrap();
    assert_eq!(
        worker.read_value("doubleCounter").unwrap(),
        compatibility.read_value("doubleCounter").unwrap()
    );
}

#[test]
fn worker_targeted_transaction_denies_the_active_branch_without_movement() {
    let mut shell = worker_shell_with_counter_graph();
    let main = shell.current_branch();
    let before = shell.worker_branch_basis(main.id.0).unwrap();

    let denial = shell
        .apply_transaction_to_worker_branch(WorkerApplyTransactionToBranchRequest {
            branch_id: main.id.0,
            expected_basis: before.clone(),
            transaction_ops: set_counter(99.0),
        })
        .unwrap_err();

    assert!(denial.message.contains("denies active branch target"));
    assert_eq!(shell.worker_branch_basis(main.id.0).unwrap(), before);
    assert_eq!(
        shell.read_value("counter").unwrap(),
        SignalValue::Number(1.0)
    );
}

#[test]
fn worker_bridge_has_no_javascript_branch_authority_fallback() {
    assert!(WORKER_BRIDGE_SOURCE.contains("worker-first execution does not fall back"));
    assert!(!WORKER_BRIDGE_SOURCE.contains("const branchState"));
    assert!(!WORKER_BRIDGE_SOURCE.contains("createWorkerBranchState"));
    assert!(!WORKER_BRIDGE_SOURCE.contains("createWorkerRuntimeMirror"));
    for method in [
        "workerBranchBasis",
        "forkBranch",
        "applyTransactionToBranch",
        "retireBranch",
    ] {
        assert!(WORKER_BRIDGE_SOURCE.contains(method));
    }
}

A queue work declaration is move-owned and cannot be cloned into two policy
admissions.

```compile_fail
use worth_store_io_scheduler::QueueWorkDeclaration;

fn duplicate_declaration(work: QueueWorkDeclaration) {
    let _duplicate = work.clone();
}
```

A typed physical foreground declaration is consumed by lowering and cannot
mint two queue work declarations from one reservation.

```compile_fail
use worth_store_io_scheduler::{
    lower_physical_foreground_work, PhysicalForegroundWorkDeclaration,
};

fn lower_twice(declaration: PhysicalForegroundWorkDeclaration) {
    let _first = lower_physical_foreground_work(declaration);
    let _second = lower_physical_foreground_work(declaration);
}
```

A policy receipt owns its exact work declaration and cannot be cloned.

```compile_fail
use worth_store_io_scheduler::QueuePolicyAdmissionReceipt;

fn duplicate_policy(receipt: QueuePolicyAdmissionReceipt) {
    let _duplicate = receipt.clone();
}
```

An execution-admission request owns its policy receipt and cannot be cloned.

```compile_fail
use worth_store_io_scheduler::QueueExecutionAdmissionRequest;

fn duplicate_request(request: QueueExecutionAdmissionRequest<'_>) {
    let _duplicate = request.clone();
}
```

One policy-admitted request cannot enter queue admission twice.

```compile_fail
use worth_store_io_scheduler::{
    admit_queue_execution_plan, IoSchedulerBackendCapabilityAdmission,
    QueueExecutionAdmissionRequest, QueuePolicyAdmissionReceipt,
};

fn admit_twice(
    policy: QueuePolicyAdmissionReceipt,
    backend: &IoSchedulerBackendCapabilityAdmission,
) {
    let request = QueueExecutionAdmissionRequest::new(policy, backend);
    let _first = admit_queue_execution_plan(request);
    let _second = admit_queue_execution_plan(request);
}
```

The admitted stage itself remains move-owned.

```compile_fail
use worth_store_io_scheduler::AdmittedQueueExecutionPlan;

fn duplicate_admitted(plan: AdmittedQueueExecutionPlan) {
    let _duplicate = plan.clone();
}
```

A ready plan is consumed by execution and cannot be executed twice.

```compile_fail
use worth_store_io_scheduler::{execute_ready_queue_plan, QueueExecutionReadyPlan};
use worth_store_physical_backend::BackendQueueExecutionCompletion;

fn execute_twice(
    plan: QueueExecutionReadyPlan,
    first: BackendQueueExecutionCompletion,
    second: BackendQueueExecutionCompletion,
) {
    let _first = execute_ready_queue_plan(plan, first);
    let _second = execute_ready_queue_plan(plan, second);
}
```

Maintenance allocation authority is exact-scope typed. A Store-minted Recovery
allocation cannot enter a Maintenance envelope:

```compile_fail
use worth_store::physical_runtime::RecoveryPhysicalAllocation;
use worth_store_maintenance::CompactionPlanningMemoryEnvelope;

fn cannot_substitute_scope<'runtime>(
    allocation: RecoveryPhysicalAllocation<'runtime>,
) {
    let _envelope = CompactionPlanningMemoryEnvelope::from_store_allocation(allocation);
}
```

Projecting an envelope into a queue report does not erase the issuing runtime
lifetime:

```compile_fail
use worth_store::physical_runtime::MaintenancePhysicalAllocation;
use worth_store_maintenance::{
    CompactionPlanningMemoryEnvelope, MaintenanceQueueLayoutReport,
};

fn cannot_escape_runtime<'runtime>(
    allocation: MaintenancePhysicalAllocation<'runtime>,
) -> MaintenanceQueueLayoutReport<'static> {
    CompactionPlanningMemoryEnvelope::from_store_allocation(allocation)
        .project_maintenance_queue_layout()
}
```

The issuing runtime cannot close while a Maintenance queue report still owns
its exact allocation authority:

```compile_fail
use std::num::NonZeroU64;
use worth_store::physical_runtime::ServingPhysicalRuntime;
use worth_store_maintenance::ImportExportMemoryEnvelope;

fn cannot_close_while_maintenance_authority_is_live(runtime: ServingPhysicalRuntime) {
    let allocation = runtime
        .physical_allocations()
        .admit_maintenance(NonZeroU64::MIN)
        .unwrap();
    let report = ImportExportMemoryEnvelope::from_store_allocation(allocation)
        .project_maintenance_queue_layout();
    let _closed = runtime.close();
    drop(report);
}
```

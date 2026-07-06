Test-support harness references cannot satisfy production admission APIs:

```compile_fail
use forge_store_buffer_pool::BufferPoolEntry;
use forge_store_readiness::S2PhysicalSubstrateReadiness;
use forge_store_test_support::harness_physical_reference;

let _entry = BufferPoolEntry::admit(
    harness_physical_reference(1),
    S2PhysicalSubstrateReadiness::from_admitted_physical_substrate_closeout(
        todo!(),
        todo!(),
    ),
);
```
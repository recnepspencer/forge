Courtroom-only harness references cannot satisfy production physical-reference APIs:

```compile_fail
use forge_store_test_support::test_authority::harness_physical_reference;
use forge_store_physical_format::PhysicalReference;

let _: PhysicalReference = harness_physical_reference(1);
```

Courtroom-only harness references do not expose a production extraction lane:

```compile_fail
use forge_store_test_support::test_authority::HarnessPhysicalReference;

let reference = HarnessPhysicalReference::for_courtroom_replay(1);
let _raw = reference.as_physical_reference();
```

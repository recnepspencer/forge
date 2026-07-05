Foreground reservations cannot be forged from raw labels or copied authority
fields.

```compile_fail
use forge_store_io_scheduler::foreground_reservation::ForegroundReservationReceipt;

let _forged = ForegroundReservationReceipt {
    state: todo!(),
    lane: todo!(),
    backend_requirement: todo!(),
    backend_profile: todo!(),
    backend_evidence_class: todo!(),
    envelope: todo!(),
    counters: todo!(),
    security_scope_identity: None,
};
```

Raw foreground labels do not satisfy reservation admission.

```compile_fail
use forge_store_io_scheduler::foreground_reservation::{
    admit_foreground_reservation, ForegroundIoLaneKind,
};

let raw_label = ForegroundIoLaneKind::PointRead;
let _reservation = admit_foreground_reservation(raw_label);
```

Copied S.5 counters cannot replace the scheduler readiness admission.

```compile_fail
use forge_store_io_scheduler::foreground_reservation::ForegroundReservationAdmissionRequest;

let copied_s5_counters = 7_u64;
let _request = ForegroundReservationAdmissionRequest::new(
    todo!(),
    todo!(),
    &copied_s5_counters,
    todo!(),
    todo!(),
    todo!(),
);
```

Copied security fields cannot replace the S.5.1 security-scope admission.

```compile_fail
use forge_store_io_scheduler::foreground_reservation::ForegroundReservationAdmissionRequest;
use forge_store_security::StoreSecurityScopeIdentity;

let copied_identity: StoreSecurityScopeIdentity = todo!();
let request = ForegroundReservationAdmissionRequest::new(
    todo!(),
    todo!(),
    todo!(),
    todo!(),
    todo!(),
    todo!(),
);
let _request = ForegroundReservationAdmissionRequest::new(
    todo!(),
    todo!(),
    todo!(),
    &copied_identity,
    todo!(),
    todo!(),
);
```

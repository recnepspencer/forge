use std::sync::Arc;

use crate::state::SignalBranchId;

use super::super::{
    SignalBranchCellAdmissionDenial, SignalBranchRegistry, SignalBranchRegistryDenial,
    SignalOwnerLifecycleState, SignalOwnerServiceCounters,
};

#[test]
fn registry_and_cell_reject_foreign_and_expired_admissions_before_contact() {
    let counters = Arc::new(SignalOwnerServiceCounters::default());
    let lifecycle = SignalOwnerLifecycleState::new(51, Arc::clone(&counters));
    let owner_admission = lifecycle.admit(51).expect("owner admits work");
    let registry = SignalBranchRegistry::new(&lifecycle, 1, 1);
    let cell = registry
        .reserve(&owner_admission, SignalBranchId(1))
        .expect("owner reserves identity")
        .install(0_u64)
        .expect("owner installs state");

    let foreign_lifecycle =
        SignalOwnerLifecycleState::new(52, Arc::new(SignalOwnerServiceCounters::default()));
    let foreign_admission = foreign_lifecycle
        .admit(52)
        .expect("foreign owner admits itself");
    let next_incarnation =
        SignalOwnerLifecycleState::new(51, Arc::new(SignalOwnerServiceCounters::default()));
    let expired_admission = next_incarnation
        .admit(51)
        .expect("new lifecycle admits itself");

    assert_eq!(
        registry
            .lookup(&foreign_admission, SignalBranchId(1))
            .unwrap_err(),
        SignalBranchRegistryDenial::ForeignOwner
    );
    assert_eq!(
        registry
            .lookup(&expired_admission, SignalBranchId(1))
            .unwrap_err(),
        SignalBranchRegistryDenial::ExpiredAdmission
    );
    assert_eq!(
        cell.with_state(&foreign_admission, |_, _| ()).unwrap_err(),
        SignalBranchCellAdmissionDenial::ForeignOwner
    );
    assert_eq!(
        cell.with_state(&expired_admission, |_, _| ()).unwrap_err(),
        SignalBranchCellAdmissionDenial::ExpiredLifecycle
    );
    let snapshot = counters.snapshot();
    assert_eq!(snapshot.branch_registry_lookups(), 0);
    assert_eq!(snapshot.target_cell_contacts(), 0);
    assert_eq!(snapshot.target_cell_waits(), 0);
}

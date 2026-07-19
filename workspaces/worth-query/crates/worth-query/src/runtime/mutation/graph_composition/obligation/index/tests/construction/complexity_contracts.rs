use std::collections::BTreeSet;

use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationIndexComplexityContractStatus,
};

#[test]
fn index_exposes_named_verified_complexity_contracts() {
    let index = WorthQueryGraphObligationIndex::empty();
    let contracts = index.complexity_contracts();

    assert_eq!(contracts.len(), 2);
    assert!(contracts.iter().all(|contract| contract.status()
        == WorthQueryGraphObligationIndexComplexityContractStatus::Verified));
    assert_eq!(
        contracts
            .iter()
            .map(|contract| contract.name())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "graph-obligation-dispatch-selection",
            "graph-obligation-index-build",
        ])
    );
    assert!(contracts.iter().any(|contract| {
        contract.name() == "graph-obligation-dispatch-selection"
            && contract
                .counter_basis()
                .contains("attempted_bucket_lookup_count")
            && contract
                .counter_basis()
                .contains("registration_full_scan_count")
    }));
}

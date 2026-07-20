use super::super::*;

fn stable(byte: u8) -> StableStoreIdentity {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).expect("nonzero identity");
    StableStoreIdentity::from_published_record(proposed)
}

#[test]
fn boundary_bridge_requires_a_fresh_matching_store_validation() {
    let current = stable(7);
    let boundary = StoreNamespaceIdentityBoundary::from_validated_identity(current);
    assert_eq!(boundary.identity(), current);

    let bridged = boundary.bridge_trust_boundary();
    assert_eq!(bridged.observed_identity_bytes(), [7; 16]);
    assert_eq!(
        bridged.readmit_after_validation(stable(8)),
        Err(StoreNamespaceIdentityReadmissionDenial::IdentityChanged {
            observed: [7; 16],
            current: [8; 16],
        })
    );

    let readmitted = StoreNamespaceIdentityBoundary::from_validated_identity(current)
        .bridge_trust_boundary()
        .readmit_after_validation(current)
        .expect("fresh matching Store validation readmits identity meaning");
    assert_eq!(readmitted.identity(), current);
}

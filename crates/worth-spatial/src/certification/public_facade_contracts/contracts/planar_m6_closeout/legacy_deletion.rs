use worth_spatial::facade::planar_m6_closeout::{
    M6PlanarCloseoutQueryCertification, M6ShortcutDeletionFamily,
};

use super::fixture::{closeout_contracts, complete_certification};

#[test]
fn m6_legacy_deletion_blocks_kernel_local_planar_shortcuts() {
    let contracts = closeout_contracts("m6-closeout-legacy-deletion");
    let receipt = M6PlanarCloseoutQueryCertification::from_certification(complete_certification(
        "m6-closeout-legacy-deletion",
    ))
    .compile(&contracts)
    .expect("M6 legacy deletion closeout plan")
    .certify()
    .expect("M6 legacy deletion closeout receipt");

    assert!(receipt.proves_no_kernel_local_planar_shortcuts());
    assert_eq!(
        receipt.counters().legacy_deletion_rows(),
        M6ShortcutDeletionFamily::ALL.len()
    );
    assert_eq!(
        receipt.counters().rejected_shortcut_rows(),
        M6ShortcutDeletionFamily::ALL.len()
    );
}

use worth_ui_host_contract::{
    UiGlyphRasterDemandIdentity, UiGlyphRasterPinTransitionView, UiGlyphRasterTransactionPending,
};

use crate::native::physical_work_signal::{
    UiNativePhysicalPresentationBasis, UiNativePhysicalSignalOwner,
};

use super::UiNativeTextAtlasGpuPages;

fn admitted_basis(
    seed: u8,
) -> crate::native::physical_work_signal::UiNativePhysicalSignalExternalBasis {
    let mut owner = UiNativePhysicalSignalOwner::new();
    let planning = owner
        .admit_atlas_planning(
            UiNativePhysicalPresentationBasis::test(),
            &[],
            UiGlyphRasterPinTransitionView::from_text_mechanics(&[], &[]),
        )
        .unwrap();
    let token = owner.take_ready_atlas_planning(planning).unwrap();
    owner
        .bind_atlas_upload(
            token,
            UiGlyphRasterTransactionPending::from_text_mechanics(
                UiGlyphRasterDemandIdentity::from_text_mechanics([seed; 32]),
                u64::from(seed),
                u64::from(seed) + 10,
                1,
            ),
        )
        .unwrap()
        .external_basis()
}

#[test]
pub(crate) fn physical_transaction_correlation_rebinds_to_the_current_signal_attempt() {
    let first = admitted_basis(7);
    let successor = admitted_basis(8);
    assert_ne!(first, successor);
    let mut gpu = UiNativeTextAtlasGpuPages::new();
    gpu.bind_transaction_correlation(91, first).unwrap();
    assert!(gpu.rebind_transaction_correlation(91, successor));
    assert_eq!(gpu.transaction_correlation_basis(91), Some(successor));
    assert!(!gpu.rebind_transaction_correlation(92, first));
}

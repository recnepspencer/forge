use worth_store::{
    ReadCompatibilityReceipt, SubscriptionSupportCompatibilityDecision,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SupportCompatibilityReceiptWitness,
    SupportDecodedRowSemanticAccess, SupportFamilyVersionWindow, SupportManifestAdmissionWitness,
};

fn main() {
    let window = SupportFamilyVersionWindow::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        1,
        2,
    )
    .unwrap();
    let manifest =
        SupportManifestAdmissionWitness::new(window, "manifest:external", "compatibility:external")
            .unwrap();
    let _semantic =
        SupportDecodedRowSemanticAccess::from_manifest_admission(manifest, "semantic:external")
            .unwrap();
    let _decision = SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
        "classifier-equivalence:external",
    )
    .unwrap();
}

fn attempt_receipt_witness(receipt: &ReadCompatibilityReceipt) {
    let _receipt_witness = SupportCompatibilityReceiptWitness::from_read_receipt(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        receipt,
    )
    .unwrap();
}

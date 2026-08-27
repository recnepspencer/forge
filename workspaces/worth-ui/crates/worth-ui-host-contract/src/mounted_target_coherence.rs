#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedTargetCoherenceDenial {
    ForeignFrame,
    ForeignInstance,
}

pub(crate) fn validate_mounted_target_coherence(
    presentation: crate::UiHostObservationPresentationBasis,
    mounted_instance: crate::UiMountedInstanceIdentity,
    node_receipt: crate::UiMountedNodeReceiptIdentity,
) -> Result<(), UiMountedTargetCoherenceDenial> {
    if node_receipt.frame() != presentation.frame() {
        return Err(UiMountedTargetCoherenceDenial::ForeignFrame);
    }
    if node_receipt.mounted_instance() != mounted_instance {
        return Err(UiMountedTargetCoherenceDenial::ForeignInstance);
    }
    Ok(())
}

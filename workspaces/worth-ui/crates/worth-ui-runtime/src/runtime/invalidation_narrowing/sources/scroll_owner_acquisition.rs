use super::UiAdmittedScrollInvalidationBinding;

pub(super) fn acquire_exact(
    binding: Option<&UiAdmittedScrollInvalidationBinding>,
    requested: crate::runtime::UiScrollReceiptActivationKey,
    mismatch: Option<crate::runtime::UiScrollOwnerAcquisitionDenial>,
    authority_probes: u16,
) -> Result<crate::runtime::UiActivatedScrollOwner, crate::runtime::UiScrollOwnerAcquisitionDenial>
{
    let first = binding.ok_or_else(|| {
        mismatch.unwrap_or(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)
    })?;
    let active_key = first
        .receipt_key()
        .ok_or(crate::runtime::UiScrollOwnerAcquisitionDenial::ReceiptNotActive)?;
    if active_key != &requested {
        return Err(active_key.mismatch_denial(&requested));
    }
    Ok(crate::runtime::UiActivatedScrollOwner::seal(
        crate::runtime::UiActivatedScrollProjectionTarget::new(
            first.target().primary().graph_node_identity(),
            first.contract().graph_generation(),
            first.contract().identity_digest(),
        ),
        active_key.clone(),
        authority_probes,
    ))
}

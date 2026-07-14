pub(crate) fn select_replan_neighborhoods(
    plan: &crate::runtime::UiNarrowedAllocationFramePlan,
    authority: &crate::runtime::UiAllocationInvalidationAuthority,
) -> Result<super::UiAdmittedReplanNeighborhoodSet, super::UiReplanLocalityDenial> {
    authority.seal_replan_transaction_basis(plan)
}

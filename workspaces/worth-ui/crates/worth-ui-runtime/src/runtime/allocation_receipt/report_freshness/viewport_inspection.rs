pub(super) fn attach_viewport_inspection(
    committed: super::UiCommittedAllocationReplan,
    basis: &crate::runtime::UiViewportResizeCommitBasis,
) -> super::UiCommittedAllocationReplan {
    let evidence = crate::evidence::UiViewportResizeEvidence::from_committed(basis, &committed);
    committed.with_viewport_evidence(evidence)
}

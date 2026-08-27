pub(super) fn staged_batch_digest(
    proposal: super::super::UiServiceProposalIdentity,
    facts: &[super::super::UiServiceProducedFactReference],
    work: &[super::super::UiServiceMountedWorkReference],
) -> u64 {
    let mut digest = proposal
        .diagnostic_value()
        .wrapping_mul(0x9e37_79b9_7f4a_7c15);
    for value in facts
        .iter()
        .map(|reference| reference.diagnostic_value())
        .chain(work.iter().map(|reference| reference.diagnostic_value()))
    {
        digest = digest.rotate_left(11) ^ value.wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    digest
}

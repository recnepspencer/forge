pub(crate) fn descriptor_semantics_version_for_envelopes(
    checkpoint_envelopes: &[crate::history::data::CanonicalCommitEnvelope],
    tail_log: &[crate::history::data::CanonicalCommitEnvelope],
) -> crate::schema::data::DescriptorSemanticsVersion {
    tail_log
        .last()
        .or_else(|| checkpoint_envelopes.last())
        .map(|envelope| envelope.descriptor_semantics_version)
        .unwrap_or_else(crate::schema::data::DescriptorSemanticsVersion::default)
}

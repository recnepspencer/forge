impl super::FrozenIntentExecutionBindings {
    pub(crate) fn digest_basis(&self) -> u64 {
        let mut digest = crate::capability::UiIntentSemanticDigest::new(0xd78d_852e_4a91_1c63)
            .usize("binding-count", self.descriptors.len());
        for descriptor in &self.descriptors {
            digest = digest
                .field("binding", &[])
                .field("intent-id", descriptor.intent.as_str().as_bytes())
                .field(
                    "payload-schema-id",
                    descriptor.payload.stable_identity().as_bytes(),
                )
                .u16("payload-schema-version", descriptor.payload.version())
                .field(
                    "outcome-schema-id",
                    descriptor.outcome.stable_identity().as_bytes(),
                )
                .u16("outcome-schema-version", descriptor.outcome.version())
                .field(
                    "execution-destination",
                    descriptor.destination.digest_basis().as_bytes(),
                )
                .u16("provider-version", descriptor.provider_version.get())
                .field("binding-support", descriptor.support.digest_tag());
        }
        digest.finish()
    }
}

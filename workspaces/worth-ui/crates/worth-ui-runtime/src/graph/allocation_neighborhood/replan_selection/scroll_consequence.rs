#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollReplanConsequence {
    evidence: crate::evidence::UiScrollOwnedAllocationEvidence,
    neighborhood_identity_digests: Box<[u64]>,
}

impl UiScrollReplanConsequence {
    pub(in crate::graph::allocation_neighborhood) fn seal(
        binding: &crate::runtime::UiAdmittedScrollInvalidationBinding,
    ) -> Result<Self, super::UiReplanLocalityDenial> {
        let target = binding.target();
        let mut digests = std::iter::once(target.primary())
            .chain(target.widened().iter())
            .map(|item| item.neighborhood_identity().identity_digest())
            .collect::<Vec<_>>();
        digests.sort_unstable();
        digests.dedup();
        if digests.is_empty() {
            return Err(super::UiReplanLocalityDenial::EmptyScrollConsequence);
        }
        let evidence = crate::evidence::UiScrollOwnedAllocationEvidence::from_contract(
            binding.contract(),
            binding.cause(),
            binding.authority_probes(),
        );
        Ok(Self {
            evidence,
            neighborhood_identity_digests: digests.into_boxed_slice(),
        })
    }

    pub(crate) fn evidence(&self) -> &crate::evidence::UiScrollOwnedAllocationEvidence {
        &self.evidence
    }
    pub(crate) fn neighborhood_identity_digests(&self) -> &[u64] {
        &self.neighborhood_identity_digests
    }
    pub(crate) fn identity_digest(&self) -> u64 {
        let mut digest = crate::declaration::stable_text_digest("worth-ui.scroll-consequence");
        digest ^= self.evidence.contract_identity_digest().rotate_left(11);
        digest ^= match self.evidence.cause() {
            crate::evidence::UiScrollOwnedExtentCause::HostContainerViewport => 1,
            crate::evidence::UiScrollOwnedExtentCause::QueryContentExtent => 2,
        };
        for neighborhood in &self.neighborhood_identity_digests {
            digest = digest.wrapping_mul(0x100000001b3);
            digest ^= neighborhood.rotate_left(29);
        }
        digest
    }
}

#[derive(Clone, Copy)]
pub(in crate::harness::adapter::adapter_impl) struct AuthorityDenialZeroResidueProof {
    authoritative_commit_count: usize,
    authoritative_artifact_count: usize,
    retained_writeback_bundle_count: usize,
    loop_side_effect_count: usize,
}

impl AuthorityDenialZeroResidueProof {
    pub(in crate::harness::adapter::adapter_impl) fn no_authority_residue() -> Self {
        Self {
            authoritative_commit_count: 0,
            authoritative_artifact_count: 0,
            retained_writeback_bundle_count: 0,
            loop_side_effect_count: 0,
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_commit_count(self) -> usize {
        self.authoritative_commit_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn authoritative_artifact_count(self) -> usize {
        self.authoritative_artifact_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn retained_writeback_bundle_count(
        self,
    ) -> usize {
        self.retained_writeback_bundle_count
    }

    pub(in crate::harness::adapter::adapter_impl) fn loop_side_effect_count(self) -> usize {
        self.loop_side_effect_count
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopIdentityMintingCounters {
    admitted_loops_considered: usize,
    denied_candidates_indexed: usize,
    split_name_rows_indexed: usize,
    loop_identities_minted: usize,
    propagated_name_rows_emitted: usize,
    subshape_signature_rows_emitted: usize,
    missing_name_seed_denials: usize,
    foreign_lineage_denials: usize,
    dangling_name_reference_denials: usize,
    duplicate_propagated_name_denials: usize,
}

impl PlanarBooleanLoopIdentityMintingCounters {
    pub(crate) fn considered_admitted_loop(&mut self) {
        self.admitted_loops_considered += 1;
    }

    pub(crate) fn indexed_denied_candidates(&mut self, count: usize) {
        self.denied_candidates_indexed += count;
    }

    pub(crate) fn indexed_split_name_rows(&mut self, count: usize) {
        self.split_name_rows_indexed += count;
    }

    pub(crate) fn minted_loop_identity(&mut self) {
        self.loop_identities_minted += 1;
    }

    pub(crate) fn emitted_propagated_name_row(&mut self) {
        self.propagated_name_rows_emitted += 1;
    }

    pub(crate) fn emitted_subshape_signature_row(&mut self) {
        self.subshape_signature_rows_emitted += 1;
    }

    pub(crate) fn denied_missing_name_seed(&mut self) {
        self.missing_name_seed_denials += 1;
    }

    pub(crate) fn denied_foreign_lineage(&mut self) {
        self.foreign_lineage_denials += 1;
    }

    pub(crate) fn denied_dangling_name_reference(&mut self) {
        self.dangling_name_reference_denials += 1;
    }

    pub(crate) fn denied_duplicate_propagated_name(&mut self) {
        self.duplicate_propagated_name_denials += 1;
    }

    pub fn admitted_loops_considered(self) -> usize {
        self.admitted_loops_considered
    }

    pub fn denied_candidates_indexed(self) -> usize {
        self.denied_candidates_indexed
    }

    pub fn split_name_rows_indexed(self) -> usize {
        self.split_name_rows_indexed
    }

    pub fn loop_identities_minted(self) -> usize {
        self.loop_identities_minted
    }

    pub fn propagated_name_rows_emitted(self) -> usize {
        self.propagated_name_rows_emitted
    }

    pub fn subshape_signature_rows_emitted(self) -> usize {
        self.subshape_signature_rows_emitted
    }

    pub fn missing_name_seed_denials(self) -> usize {
        self.missing_name_seed_denials
    }

    pub fn foreign_lineage_denials(self) -> usize {
        self.foreign_lineage_denials
    }

    pub fn dangling_name_reference_denials(self) -> usize {
        self.dangling_name_reference_denials
    }

    pub fn duplicate_propagated_name_denials(self) -> usize {
        self.duplicate_propagated_name_denials
    }
}

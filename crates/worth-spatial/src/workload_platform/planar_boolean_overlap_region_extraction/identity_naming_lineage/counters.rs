#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionIdentityLineageCounters {
    canonical_rows_examined: usize,
    identity_rows_admitted: usize,
    persistent_name_rows_admitted: usize,
    subshape_signature_rows_admitted: usize,
    rows_denied: usize,
}

impl PlanarBooleanOverlapRegionIdentityLineageCounters {
    pub(crate) fn examined_canonical_row(&mut self) {
        self.canonical_rows_examined += 1;
    }

    pub(crate) fn admitted_identity_row(&mut self) {
        self.identity_rows_admitted += 1;
    }

    pub(crate) fn admitted_persistent_name_row(&mut self, count: usize) {
        self.persistent_name_rows_admitted += count;
    }

    pub(crate) fn admitted_subshape_signature_row(&mut self) {
        self.subshape_signature_rows_admitted += 1;
    }

    pub(crate) fn denied_row(&mut self) {
        self.rows_denied += 1;
    }

    pub fn canonical_rows_examined(self) -> usize {
        self.canonical_rows_examined
    }

    pub fn identity_rows_admitted(self) -> usize {
        self.identity_rows_admitted
    }

    pub fn persistent_name_rows_admitted(self) -> usize {
        self.persistent_name_rows_admitted
    }

    pub fn subshape_signature_rows_admitted(self) -> usize {
        self.subshape_signature_rows_admitted
    }

    pub fn rows_denied(self) -> usize {
        self.rows_denied
    }
}

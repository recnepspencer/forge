#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanPreRegionNormalizationCounters {
    admitted_normalizations: usize,
    denied_normalizations: usize,
    examined_lineage_rows: usize,
}

impl PlanarBooleanPreRegionNormalizationCounters {
    pub(crate) fn admitted_normalization(&mut self) {
        self.admitted_normalizations += 1;
    }

    pub(crate) fn denied_normalization(&mut self) {
        self.denied_normalizations += 1;
    }

    pub(crate) fn examined_lineage_row(&mut self) {
        self.examined_lineage_rows += 1;
    }
}

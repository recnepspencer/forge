use super::row::PlanarBooleanSplitDecisionRow;

pub struct PlanarBooleanSplitArtifactDecisionRows<'a> {
    rows: &'a [PlanarBooleanSplitDecisionRow],
    indexes: &'a [usize],
    position: usize,
}

impl<'a> PlanarBooleanSplitArtifactDecisionRows<'a> {
    pub(crate) fn new(rows: &'a [PlanarBooleanSplitDecisionRow], indexes: &'a [usize]) -> Self {
        Self {
            rows,
            indexes,
            position: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.indexes.is_empty()
    }
}

impl<'a> Iterator for PlanarBooleanSplitArtifactDecisionRows<'a> {
    type Item = &'a PlanarBooleanSplitDecisionRow;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.indexes.get(self.position).copied() {
            self.position += 1;
            if let Some(row) = self.rows.get(index) {
                return Some(row);
            }
        }
        None
    }
}

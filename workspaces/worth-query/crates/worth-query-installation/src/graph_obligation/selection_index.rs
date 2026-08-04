use super::{WorthQueryInstalledGraphObligation, WorthQueryInstalledGraphObligationKind};

const KIND_COUNT: usize = WorthQueryInstalledGraphObligationKind::ALL.len();

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryInstalledGraphObligationSelectionIndex {
    ranges: [(usize, usize); KIND_COUNT],
}

impl WorthQueryInstalledGraphObligationSelectionIndex {
    pub(super) fn build(rows: &[WorthQueryInstalledGraphObligation]) -> Self {
        let mut ranges = [(0, 0); KIND_COUNT];
        let mut cursor = 0;
        for kind in WorthQueryInstalledGraphObligationKind::ALL {
            let start = cursor;
            while rows.get(cursor).is_some_and(|row| row.kind() == kind) {
                cursor += 1;
            }
            ranges[kind.index()] = (start, cursor);
        }
        debug_assert_eq!(cursor, rows.len(), "obligation rows are kind ordered");
        Self { ranges }
    }

    pub(super) fn select<'a>(
        &self,
        rows: &'a [WorthQueryInstalledGraphObligation],
        kind: WorthQueryInstalledGraphObligationKind,
    ) -> &'a [WorthQueryInstalledGraphObligation] {
        let (start, end) = self.ranges[kind.index()];
        &rows[start..end]
    }
}

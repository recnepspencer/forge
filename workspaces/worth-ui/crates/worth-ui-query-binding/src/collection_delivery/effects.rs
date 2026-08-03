use super::WorthUiCollectionRowReference;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionAllocationPolicy {
    PreserveAdmittedRows,
    ReleaseOutsideWindow,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthUiCollectionGraphEffect {
    Insert {
        row: WorthUiCollectionRowReference,
        at: usize,
    },
    Remove {
        row: WorthUiCollectionRowReference,
        from: usize,
    },
    Move {
        row: WorthUiCollectionRowReference,
        from: usize,
        to: usize,
    },
    Update {
        row: WorthUiCollectionRowReference,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthUiCollectionMeasurementEffect {
    RowChanged(WorthUiCollectionRowReference),
    ChangedNativeFacts { count: usize },
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorthUiCollectionAllocationEffect {
    RowPreservationCandidate(WorthUiCollectionRowReference),
    WindowShift {
        policy: WorthUiCollectionAllocationPolicy,
    },
}

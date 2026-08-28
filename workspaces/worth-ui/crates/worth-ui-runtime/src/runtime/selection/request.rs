#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiSelectionRequest {
    SelectSingle(super::UiSelectionStableKey),
    ToggleMultiple(super::UiSelectionStableKey),
    Add(super::UiSelectionStableKey),
    Remove(super::UiSelectionStableKey),
    SelectRange {
        target: super::UiSelectionStableKey,
        extend: bool,
    },
    Clear,
    SelectAll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiSelectionRequestDenial {
    UnknownOwner,
    StaleOwnerIncarnation,
    CatalogUnavailable,
    CatalogCapacityExceeded,
    DuplicateCatalogKey,
    ForeignItemKeyFamily,
    UnknownKey,
    RangeNotSupported,
    MultipleNotSupported,
    MissingRangeAnchor,
    PartialCatalogSelectAllDenied,
    RevisionExhausted,
    CounterOverflow,
}

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
    RevisionExhausted,
    CounterOverflow,
}

impl UiSelectionRequest {
    pub(in crate::runtime) const fn application_item_key(
        self,
    ) -> Option<crate::runtime::UiApplicationItemKey> {
        match self {
            Self::SelectSingle(key)
            | Self::ToggleMultiple(key)
            | Self::Add(key)
            | Self::Remove(key)
            | Self::SelectRange { target: key, .. } => Some(key.application_key()),
        }
    }
}

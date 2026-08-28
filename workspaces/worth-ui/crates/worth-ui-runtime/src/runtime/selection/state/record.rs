use std::collections::{BTreeMap, BTreeSet};

use super::UiSelectionOwnerRecord;

pub(super) fn empty_record(
    registration: &super::super::UiSelectionRegistration,
) -> UiSelectionOwnerRecord {
    UiSelectionOwnerRecord {
        incarnation: registration.incarnation(),
        policy: registration.policy(),
        catalog: std::sync::Arc::from([]),
        catalog_positions: std::sync::Arc::new(BTreeMap::new()),
        catalog_posture: registration.catalog_posture(),
        catalog_revision: registration.catalog_revision(),
        catalog_available: true,
        selected: BTreeSet::new(),
        anchor: None,
        cursor: None,
    }
}

pub(in crate::runtime::selection) fn validate_catalog(
    owner: super::super::UiSelectionOwnerIdentity,
    catalog: &[super::super::UiSelectionStableKey],
) -> Result<
    BTreeMap<super::super::UiSelectionStableKey, usize>,
    super::super::UiSelectionRequestDenial,
> {
    if catalog.len() > super::super::model::UI_SELECTION_CATALOG_LIMIT {
        return Err(super::super::UiSelectionRequestDenial::CatalogCapacityExceeded);
    }
    let mut positions = BTreeMap::new();
    for (index, key) in catalog.iter().copied().enumerate() {
        if key.family() != owner.item_key_family() {
            return Err(super::super::UiSelectionRequestDenial::ForeignItemKeyFamily);
        }
        if positions.insert(key, index).is_some() {
            return Err(super::super::UiSelectionRequestDenial::DuplicateCatalogKey);
        }
    }
    Ok(positions)
}

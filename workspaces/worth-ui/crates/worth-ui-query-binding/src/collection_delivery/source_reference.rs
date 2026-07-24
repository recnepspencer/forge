use std::sync::Arc;

/// Opaque UI authority for one retained live collection source.
///
/// The token is minted only after the live resource enters its binding owner.
/// It contains no Query identity and cannot be converted back into one.
#[derive(Clone)]
pub struct WorthUiCollectionChangeSourceReference {
    authority: Arc<WorthUiCollectionChangeSourceAuthority>,
}

struct WorthUiCollectionChangeSourceAuthority {
    _non_zero_sized: u8,
}

/// UI-local reference for one row effect in one exact collection change.
///
/// This is not a mounted identity. Milestone 3.12 may use it while deciding
/// preserve/remount consequences, and Milestone 3.10 alone may mint mounted
/// receipt authority.
#[derive(Clone, Eq, PartialEq)]
pub struct WorthUiCollectionRowReference {
    source: WorthUiCollectionChangeSourceReference,
    change_order: u64,
    effect_order: usize,
}

impl WorthUiCollectionChangeSourceReference {
    pub(crate) fn mint() -> Self {
        Self {
            authority: Arc::new(WorthUiCollectionChangeSourceAuthority { _non_zero_sized: 0 }),
        }
    }
}

impl WorthUiCollectionRowReference {
    pub(crate) fn mint(
        source: &WorthUiCollectionChangeSourceReference,
        change_order: u64,
        effect_order: usize,
    ) -> Self {
        Self {
            source: source.clone(),
            change_order,
            effect_order,
        }
    }

    pub fn source(&self) -> &WorthUiCollectionChangeSourceReference {
        &self.source
    }
}

impl PartialEq for WorthUiCollectionChangeSourceReference {
    fn eq(&self, candidate: &Self) -> bool {
        Arc::ptr_eq(&self.authority, &candidate.authority)
    }
}

impl Eq for WorthUiCollectionChangeSourceReference {}

impl std::fmt::Debug for WorthUiCollectionChangeSourceReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiCollectionChangeSourceReference")
            .field("authority", &"sealed")
            .finish()
    }
}

impl std::fmt::Debug for WorthUiCollectionRowReference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiCollectionRowReference")
            .field("source", &self.source)
            .field("change_order", &self.change_order)
            .field("effect_order", &self.effect_order)
            .finish()
    }
}

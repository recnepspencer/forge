use std::{
    cmp::Ordering as CmpOrdering,
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Stable UI binding-slot authority admitted from one exact installed Query
/// binding. It remains equal across settlement refreshes of that binding.
#[derive(Clone, Eq, PartialEq)]
pub struct WorthUiAdmittedQueryBindingReference {
    installed: super::WorthUiInstalledQueryBindingReference,
    key: WorthUiAdmittedQueryBindingKey,
}

/// Compact immutable identity for one owner-admitted UI binding slot.
///
/// Ordered indexes retain this key instead of the Query installation handle
/// held by the full reference.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthUiAdmittedQueryBindingKey {
    authority_order: usize,
    definition: crate::WorthUiQueryViewDefinition,
}

/// UI-owned revision authority for one successfully settled Query projection.
/// A refreshed settlement always receives a distinct reference.
#[derive(Clone)]
pub struct WorthUiAdmittedQuerySettlementReference {
    authority: Arc<SettlementAuthorityToken>,
}

struct SettlementAuthorityToken {
    _non_zero_sized: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedQuerySettlementTouchReference {
    binding: WorthUiAdmittedQueryBindingReference,
    settlement: WorthUiAdmittedQuerySettlementReference,
}

impl WorthUiAdmittedQueryBindingReference {
    pub(crate) fn admit(installed: &super::WorthUiInstalledQueryBindingReference) -> Self {
        Self {
            installed: installed.clone(),
            key: WorthUiAdmittedQueryBindingKey {
                authority_order: installed.authority_order(),
                definition: installed.definition().clone(),
            },
        }
    }

    pub fn definition(&self) -> &crate::WorthUiQueryViewDefinition {
        self.installed.definition()
    }

    pub fn installation_is_current(&self) -> bool {
        self.installed.installation_is_current()
    }

    pub fn key(&self) -> &WorthUiAdmittedQueryBindingKey {
        &self.key
    }

    pub(crate) fn installed_reference(&self) -> &super::WorthUiInstalledQueryBindingReference {
        &self.installed
    }
}

impl WorthUiAdmittedQuerySettlementReference {
    pub(crate) fn mint() -> Self {
        Self {
            authority: Arc::new(SettlementAuthorityToken { _non_zero_sized: 0 }),
        }
    }
}

impl WorthUiAdmittedQuerySettlementTouchReference {
    pub(crate) fn mint(fact: &super::WorthUiSettledSnapshotFact) -> Self {
        Self {
            binding: fact.binding_reference().clone(),
            settlement: fact.settlement_reference().clone(),
        }
    }

    pub fn binding_reference(&self) -> &WorthUiAdmittedQueryBindingReference {
        &self.binding
    }

    pub fn settlement_reference(&self) -> &WorthUiAdmittedQuerySettlementReference {
        &self.settlement
    }
}

impl Ord for WorthUiAdmittedQueryBindingReference {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        self.installed
            .authority_order()
            .cmp(&other.installed.authority_order())
            .then_with(|| {
                self.installed
                    .definition()
                    .cmp(other.installed.definition())
            })
    }
}

impl PartialOrd for WorthUiAdmittedQueryBindingReference {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Hash for WorthUiAdmittedQueryBindingReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.installed.authority_order().hash(state);
        self.installed.definition().hash(state);
    }
}

impl Eq for WorthUiAdmittedQuerySettlementReference {}

impl Ord for WorthUiAdmittedQuerySettlementReference {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        self.authority_address().cmp(&other.authority_address())
    }
}

impl PartialEq for WorthUiAdmittedQuerySettlementReference {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.authority, &other.authority)
    }
}

impl PartialOrd for WorthUiAdmittedQuerySettlementReference {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Hash for WorthUiAdmittedQuerySettlementReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.authority_address().hash(state);
    }
}

impl WorthUiAdmittedQuerySettlementReference {
    fn authority_address(&self) -> usize {
        Arc::as_ptr(&self.authority) as usize
    }
}

impl fmt::Debug for WorthUiAdmittedQueryBindingReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiAdmittedQueryBindingReference")
            .field("definition", self.definition())
            .field("query_authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for WorthUiAdmittedQueryBindingKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiAdmittedQueryBindingKey")
            .field("definition", &self.definition)
            .field("binding_authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for WorthUiAdmittedQuerySettlementReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiAdmittedQuerySettlementReference")
            .field("revision_authority", &"sealed")
            .finish()
    }
}

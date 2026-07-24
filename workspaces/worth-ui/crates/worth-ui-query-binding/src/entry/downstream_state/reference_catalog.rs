use std::collections::BTreeMap;

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryViewExecutionEvidenceDenial,
    WorthUiQueryViewIdentity,
};

#[derive(Debug)]
pub(super) struct WorthUiInstalledReferenceCatalog {
    references: BTreeMap<WorthUiQueryViewIdentity, WorthUiInstalledQueryBindingReference>,
}

impl WorthUiInstalledReferenceCatalog {
    pub(super) fn new(
        references: BTreeMap<WorthUiQueryViewIdentity, WorthUiInstalledQueryBindingReference>,
    ) -> Self {
        Self { references }
    }

    pub(super) fn validate(
        &self,
        reference: &WorthUiInstalledQueryBindingReference,
    ) -> Result<(), WorthUiQueryViewExecutionEvidenceDenial> {
        if self.references.get(reference.definition().identity()) != Some(reference) {
            return Err(WorthUiQueryViewExecutionEvidenceDenial::ForeignInstalledReference);
        }
        Ok(())
    }

    pub(super) fn installation_is_current(&self) -> bool {
        self.references
            .values()
            .next()
            .is_none_or(WorthUiInstalledQueryBindingReference::installation_is_current)
    }

    pub(super) fn len(&self) -> usize {
        self.references.len()
    }

    pub(super) fn stale_reference_count(&self) -> usize {
        self.references
            .values()
            .filter(|reference| !reference.installation_is_current())
            .count()
    }

    pub(super) fn has_live_reference(&self) -> bool {
        self.references.values().any(|reference| {
            reference.definition().lifecycle() == crate::WorthUiQueryViewLifecycle::Live
        })
    }

    pub(super) fn identities(
        &self,
    ) -> impl ExactSizeIterator<Item = WorthUiQueryViewIdentity> + '_ {
        self.references.keys().cloned()
    }

    pub(super) fn live_identities(&self) -> impl Iterator<Item = WorthUiQueryViewIdentity> + '_ {
        self.references
            .values()
            .filter(|reference| {
                reference.definition().lifecycle() == crate::WorthUiQueryViewLifecycle::Live
            })
            .map(|reference| reference.definition().identity().clone())
    }

    pub(super) fn references(
        &self,
    ) -> impl ExactSizeIterator<Item = WorthUiInstalledQueryBindingReference> + '_ {
        self.references.values().cloned()
    }
}

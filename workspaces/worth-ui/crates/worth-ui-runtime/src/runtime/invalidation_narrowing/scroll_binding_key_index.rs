use std::collections::HashMap;

use super::UiAdmittedScrollInvalidationBinding;

#[derive(Clone, Debug, Default)]
pub(super) struct BindingKeyIndex {
    exact:
        HashMap<crate::runtime::UiScrollReceiptActivationKey, UiAdmittedScrollInvalidationBinding>,
    diagnostic: HashMap<DiagnosticKey, crate::runtime::UiScrollReceiptActivationKey>,
    by_receipt: HashMap<ReceiptDiagnosticKey, Vec<crate::runtime::UiScrollReceiptActivationKey>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DiagnosticKey {
    receipt_identity: crate::runtime::UiAllocationReceiptIdentity,
    source: crate::runtime::UiAdmittedScrollExtentSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReceiptDiagnosticKey(crate::runtime::UiAllocationReceiptIdentity);

impl std::hash::Hash for ReceiptDiagnosticKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.identity_digest());
    }
}

impl DiagnosticKey {
    fn from_key(key: &crate::runtime::UiScrollReceiptActivationKey) -> Self {
        Self {
            receipt_identity: key.receipt_identity().clone(),
            source: key.source().clone(),
        }
    }
}

impl std::hash::Hash for DiagnosticKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.receipt_identity.identity_digest());
        state.write_u64(self.source.identity_digest());
    }
}

impl BindingKeyIndex {
    pub(super) fn get(
        &self,
        key: &crate::runtime::UiScrollReceiptActivationKey,
    ) -> Option<&UiAdmittedScrollInvalidationBinding> {
        self.exact.get(key)
    }
    pub(super) fn values(&self) -> impl Iterator<Item = &UiAdmittedScrollInvalidationBinding> {
        self.exact.values()
    }
    pub(super) fn contains_key(&self, key: &crate::runtime::UiScrollReceiptActivationKey) -> bool {
        self.get(key).is_some()
    }
    pub(super) fn insert(
        &mut self,
        _key_digest: u64,
        row: UiAdmittedScrollInvalidationBinding,
    ) -> Option<UiAdmittedScrollInvalidationBinding> {
        let receipt = row.receipt_key()?.clone();
        if let Some(prior) = self.exact.get(&receipt) {
            return Some(prior.clone());
        }
        let diagnostic_receipt = ReceiptDiagnosticKey(receipt.receipt_identity().clone());
        let sources = self.by_receipt.entry(diagnostic_receipt).or_default();
        if let Some(prior_key) = sources.iter().find(|key| key.source() == receipt.source()) {
            return self.exact.get(prior_key).cloned();
        }
        sources.push(receipt.clone());
        self.diagnostic
            .insert(DiagnosticKey::from_key(&receipt), receipt.clone());
        self.exact.insert(receipt, row)
    }
    pub(super) fn classify(
        &self,
        requested: &crate::runtime::UiScrollReceiptActivationKey,
    ) -> Option<crate::runtime::UiScrollOwnerAcquisitionDenial> {
        if let Some(active) = self.diagnostic.get(&DiagnosticKey::from_key(requested)) {
            return Some(active.mismatch_denial(requested));
        }
        let sources = self
            .by_receipt
            .get(&ReceiptDiagnosticKey(requested.receipt_identity().clone()))?;
        match sources.as_slice() {
            [] => None,
            [active] => Some(active.mismatch_denial(requested)),
            _ => Some(crate::runtime::UiScrollOwnerAcquisitionDenial::AmbiguousOwner),
        }
    }
}

use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionBundleFieldState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionBundleField {
    field_name: Arc<str>,
    field_state: BridgeSubscriptionBundleFieldState,
    field_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionBundleField {
    pub(crate) fn new(
        field_name: impl Into<Arc<str>>,
        field_state: BridgeSubscriptionBundleFieldState,
        field_digest: impl Into<Arc<str>>,
    ) -> Self {
        let field_name = field_name.into();
        let field_digest = field_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-bundle-field|name={field_name}|state={}|field-digest={field_digest}",
            field_state.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            field_name,
            field_state,
            field_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-bundle-field:sha256:{digest:x}"
            )),
        }
    }

    pub fn field_name(&self) -> &str {
        self.field_name.as_ref()
    }

    pub fn field_state(&self) -> BridgeSubscriptionBundleFieldState {
        self.field_state
    }

    pub fn field_digest(&self) -> &str {
        self.field_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

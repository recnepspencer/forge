use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeTemporalAsyncFailureClass, BridgeTemporalAsyncFailureSubcode,
    BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncFailureLocalizationRow {
    failure_class: BridgeTemporalAsyncFailureClass,
    subcode: BridgeTemporalAsyncFailureSubcode,
    localized_failure_digest: Arc<str>,
}

impl BridgeTemporalAsyncFailureLocalizationRow {
    fn new(
        failure_class: BridgeTemporalAsyncFailureClass,
        subcode: BridgeTemporalAsyncFailureSubcode,
        localized_failure_digest: Arc<str>,
    ) -> Self {
        Self {
            failure_class,
            subcode,
            localized_failure_digest,
        }
    }

    pub fn failure_class(&self) -> BridgeTemporalAsyncFailureClass {
        self.failure_class
    }

    pub fn subcode(&self) -> BridgeTemporalAsyncFailureSubcode {
        self.subcode
    }

    pub fn localized_failure_digest(&self) -> &str {
        self.localized_failure_digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncFailureLocalizationMatrix {
    rows: Arc<[BridgeTemporalAsyncFailureLocalizationRow]>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncFailureLocalizationMatrix {
    pub fn from_bundle(bundle: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed) -> Self {
        let rows = bundle
            .localized_failures()
            .iter()
            .map(|failure| {
                BridgeTemporalAsyncFailureLocalizationRow::new(
                    failure.failure_class(),
                    failure.subcode(),
                    Arc::from(failure.digest().to_owned()),
                )
            })
            .collect::<Vec<_>>();
        let canonical_basis = rows
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|{}",
                    row.failure_class.as_str(),
                    row.subcode.as_str(),
                    row.localized_failure_digest.as_ref()
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rows: rows.into(),
            digest: Arc::from(format!(
                "bridge-temporal-async-failure-localization-matrix:sha256:{digest:x}"
            )),
        }
    }

    pub fn rows(&self) -> &[BridgeTemporalAsyncFailureLocalizationRow] {
        self.rows.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

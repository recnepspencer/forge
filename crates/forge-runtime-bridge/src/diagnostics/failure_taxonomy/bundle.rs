use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeLocalizedTemporalAsyncFailure, BridgeTemporalAsyncFailureCounters};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeTemporalAsyncOfflineDiagnosisBundleRejectionKind {
    EmptyLocalizationSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncOfflineDiagnosisBundleRejection {
    kind: BridgeTemporalAsyncOfflineDiagnosisBundleRejectionKind,
    detail: Arc<str>,
}

impl BridgeTemporalAsyncOfflineDiagnosisBundleRejection {
    fn new(
        kind: BridgeTemporalAsyncOfflineDiagnosisBundleRejectionKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> BridgeTemporalAsyncOfflineDiagnosisBundleRejectionKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncOfflineDiagnosisBundleDraft {
    localized_failures: Arc<[BridgeLocalizedTemporalAsyncFailure]>,
}

impl BridgeTemporalAsyncOfflineDiagnosisBundleDraft {
    pub fn new(
        localized_failures: Vec<BridgeLocalizedTemporalAsyncFailure>,
    ) -> Result<Self, BridgeTemporalAsyncOfflineDiagnosisBundleRejection> {
        if localized_failures.is_empty() {
            return Err(BridgeTemporalAsyncOfflineDiagnosisBundleRejection::new(
                BridgeTemporalAsyncOfflineDiagnosisBundleRejectionKind::EmptyLocalizationSet,
                "offline diagnosis bundle requires at least one localized failure",
            ));
        }
        Ok(Self {
            localized_failures: localized_failures.into(),
        })
    }

    pub fn localized_failures(&self) -> &[BridgeLocalizedTemporalAsyncFailure] {
        self.localized_failures.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncOfflineDiagnosisBundleSealed {
    localized_failures: Arc<[BridgeLocalizedTemporalAsyncFailure]>,
    counters: BridgeTemporalAsyncFailureCounters,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncOfflineDiagnosisBundleSealed {
    pub fn seal(draft: BridgeTemporalAsyncOfflineDiagnosisBundleDraft) -> Self {
        let canonical_basis = draft
            .localized_failures()
            .iter()
            .map(BridgeLocalizedTemporalAsyncFailure::digest)
            .collect::<Vec<_>>()
            .join(",");
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let localized_failures = draft.localized_failures;
        Self {
            counters: BridgeTemporalAsyncFailureCounters::bundled(localized_failures.len()),
            localized_failures,
            digest: Arc::from(format!(
                "bridge-temporal-async-offline-diagnosis-bundle:sha256:{digest:x}"
            )),
        }
    }

    pub fn localized_failures(&self) -> &[BridgeLocalizedTemporalAsyncFailure] {
        self.localized_failures.as_ref()
    }

    pub fn counters(&self) -> &BridgeTemporalAsyncFailureCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncFailureBundleComparison {
    equivalent: bool,
    left_digest: Arc<str>,
    right_digest: Arc<str>,
}

impl BridgeTemporalAsyncFailureBundleComparison {
    pub fn compare(
        left: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
        right: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
    ) -> Self {
        Self {
            equivalent: left.digest() == right.digest(),
            left_digest: Arc::from(left.digest().to_owned()),
            right_digest: Arc::from(right.digest().to_owned()),
        }
    }

    pub fn equivalent(&self) -> bool {
        self.equivalent
    }

    pub fn left_digest(&self) -> &str {
        self.left_digest.as_ref()
    }

    pub fn right_digest(&self) -> &str {
        self.right_digest.as_ref()
    }
}

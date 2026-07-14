use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use serde::{Deserialize, Serialize};

macro_rules! epoch_type {
    ($name:ident, $label:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(u64);

        impl $name {
            pub fn new(value: u64) -> Result<Self, SupportTrustFailure> {
                if value == 0 {
                    return Err(SupportTrustFailure::new(
                        SupportTrustFailureKind::SupportTrustEpochExpired,
                        SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                        concat!($label, " epoch must be non-zero"),
                    ));
                }
                Ok(Self(value))
            }

            pub fn value(&self) -> u64 {
                self.0
            }
        }
    };
}

epoch_type!(SupportCatalogEpoch, "catalog");
epoch_type!(SupportOperationalLedgerEpoch, "operational ledger");
epoch_type!(SupportCompatibilityEpoch, "compatibility");
epoch_type!(SupportCertificationEpoch, "certification");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SupportCertificationCorpusVersion(String);

impl SupportCertificationCorpusVersion {
    pub fn new(value: impl Into<String>) -> Result<Self, SupportTrustFailure> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support certification corpus version must be non-empty",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportTrustEpoch {
    catalog: SupportCatalogEpoch,
    operational_ledger: SupportOperationalLedgerEpoch,
    compatibility: SupportCompatibilityEpoch,
    certification: Option<SupportCertificationEpoch>,
}

impl SupportTrustEpoch {
    pub fn new(
        catalog: SupportCatalogEpoch,
        operational_ledger: SupportOperationalLedgerEpoch,
        compatibility: SupportCompatibilityEpoch,
        certification: Option<SupportCertificationEpoch>,
    ) -> Self {
        Self {
            catalog,
            operational_ledger,
            compatibility,
            certification,
        }
    }

    pub fn catalog(&self) -> SupportCatalogEpoch {
        self.catalog
    }

    pub fn operational_ledger(&self) -> SupportOperationalLedgerEpoch {
        self.operational_ledger
    }

    pub fn compatibility(&self) -> SupportCompatibilityEpoch {
        self.compatibility
    }

    pub fn certification(&self) -> Option<SupportCertificationEpoch> {
        self.certification
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustFreshnessWitness {
    epoch: SupportTrustEpoch,
}

impl SupportTrustFreshnessWitness {
    #[allow(dead_code)]
    pub(crate) fn new(epoch: SupportTrustEpoch) -> Self {
        Self { epoch }
    }

    pub fn epoch(&self) -> SupportTrustEpoch {
        self.epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportTrustExpiredReport {
    expected: SupportTrustEpoch,
    actual: SupportTrustEpoch,
}

impl SupportTrustExpiredReport {
    #[allow(dead_code)]
    pub(crate) fn new(expected: SupportTrustEpoch, actual: SupportTrustEpoch) -> Self {
        Self { expected, actual }
    }

    pub fn expected(&self) -> SupportTrustEpoch {
        self.expected
    }

    pub fn actual(&self) -> SupportTrustEpoch {
        self.actual
    }
}

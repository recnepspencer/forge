use std::sync::Arc;

use crate::domain_computation::WorthQueryGraphProviderFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderCheckpointExport {
    format_identity: Arc<str>,
    format_version: u64,
    compatibility_identity: Arc<str>,
    payload: Arc<[u8]>,
}

pub(crate) enum WorthQueryProviderCheckpointExportInvocation {
    Returned(Result<WorthQueryProviderCheckpointExport, WorthQueryGraphProviderFailure>),
    Panicked,
}

impl WorthQueryProviderCheckpointExport {
    pub fn new(
        format_identity: impl Into<Arc<str>>,
        format_version: u64,
        compatibility_identity: impl Into<Arc<str>>,
        payload: impl Into<Arc<[u8]>>,
    ) -> Result<Self, WorthQueryGraphProviderFailure> {
        let format_identity = format_identity.into();
        let compatibility_identity = compatibility_identity.into();
        let payload = payload.into();
        if format_identity.is_empty() {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export format identity cannot be empty",
            ));
        }
        if format_version == 0 {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export format version must be nonzero",
            ));
        }
        if compatibility_identity.is_empty() {
            return Err(WorthQueryGraphProviderFailure::new(
                "checkpoint export compatibility identity cannot be empty",
            ));
        }
        Ok(Self {
            format_identity,
            format_version,
            compatibility_identity,
            payload,
        })
    }

    pub fn format_identity(&self) -> &str {
        &self.format_identity
    }

    pub const fn format_version(&self) -> u64 {
        self.format_version
    }

    pub fn compatibility_identity(&self) -> &str {
        &self.compatibility_identity
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

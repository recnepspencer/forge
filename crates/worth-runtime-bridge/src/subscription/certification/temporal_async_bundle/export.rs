use std::sync::Arc;

use super::bundle::BridgeTemporalAsyncCertificationBundleSealed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationBundleExport {
    export_name: Arc<str>,
    bundle_digest: Arc<str>,
    semantic_digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationBundleExport {
    pub(crate) fn export(bundle: &BridgeTemporalAsyncCertificationBundleSealed) -> Self {
        Self {
            export_name: Arc::from(format!(
                "{}-{}.json",
                bundle.schema_version(),
                bundle.digest().rsplit(':').next().unwrap_or("digest")
            )),
            bundle_digest: Arc::from(bundle.digest().to_owned()),
            semantic_digest: Arc::from(bundle.semantic_digest().to_owned()),
        }
    }

    pub fn export_name(&self) -> &str {
        self.export_name.as_ref()
    }

    pub fn bundle_digest(&self) -> &str {
        self.bundle_digest.as_ref()
    }
}

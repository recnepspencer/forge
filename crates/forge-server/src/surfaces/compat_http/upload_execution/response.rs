use crate::{
    ForgeServerBinaryCertificationBundle, ForgeServerCompatibilityFileEnvelope,
    ForgeServerIngressIntegrityDigest, ForgeServerIngressPerformanceReceipt,
    ForgeServerMultipartUpload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityUpload {
    upload: ForgeServerMultipartUpload,
    ingress_integrity: ForgeServerIngressIntegrityDigest,
    ingress_performance: ForgeServerIngressPerformanceReceipt,
    mutation: crate::ForgeServerCompatibilityMutation,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerBinaryCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerCompatibilityUpload {
    pub(crate) fn new(
        upload: ForgeServerMultipartUpload,
        ingress_integrity: ForgeServerIngressIntegrityDigest,
        ingress_performance: ForgeServerIngressPerformanceReceipt,
        mutation: crate::ForgeServerCompatibilityMutation,
        file_envelope: ForgeServerCompatibilityFileEnvelope,
        certification_bundle: ForgeServerBinaryCertificationBundle,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-upload-v4|upload={}|integrity={}|ingress_performance={}|mutation={}|file_envelope={}|certification={}",
            upload.canonical_digest(),
            ingress_integrity.canonical_digest(),
            ingress_performance_digest(&ingress_performance),
            mutation.canonical_digest(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            upload,
            ingress_integrity,
            ingress_performance,
            mutation,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn upload(&self) -> &ForgeServerMultipartUpload {
        &self.upload
    }

    pub fn ingress_integrity(&self) -> &ForgeServerIngressIntegrityDigest {
        &self.ingress_integrity
    }

    pub fn ingress_performance(&self) -> &ForgeServerIngressPerformanceReceipt {
        &self.ingress_performance
    }

    pub fn mutation(&self) -> &crate::ForgeServerCompatibilityMutation {
        &self.mutation
    }

    pub fn file_envelope(&self) -> &ForgeServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &ForgeServerBinaryCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn ingress_performance_digest(receipt: &ForgeServerIngressPerformanceReceipt) -> String {
    receipt
        .receipt()
        .counter_rows()
        .iter()
        .map(|row| format!("{}={}", row.name().as_str(), row.observed_count()))
        .collect::<Vec<_>>()
        .join("|")
}

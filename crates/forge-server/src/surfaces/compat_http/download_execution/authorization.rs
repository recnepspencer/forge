use crate::{ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryDownloadAuthorization {
    start: usize,
    end_exclusive: usize,
    canonical_digest: String,
}

impl ForgeServerBinaryDownloadAuthorization {
    pub fn entire_representation(total_bytes: usize) -> Self {
        Self::admitted_window(0, total_bytes)
    }

    pub fn admitted_window(start: usize, end_exclusive: usize) -> Self {
        let canonical_digest = format!(
            "compat-http-download-authorization-v1|start={start}|end_exclusive={end_exclusive}"
        );
        Self {
            start,
            end_exclusive,
            canonical_digest,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end_exclusive(&self) -> usize {
        self.end_exclusive
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn admit_selected_span(
        &self,
        diagnostics_profile: forge_foundational::DiagnosticRichnessProfile,
        start: usize,
        end_exclusive: usize,
    ) -> Result<(), ForgeServerQueryHandoffDenial> {
        if start < self.start || end_exclusive > self.end_exclusive {
            return Err(ForgeServerQueryHandoffDenial::new(
                ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid,
                diagnostics_profile,
                "requested binary egress span falls outside the admitted authorization window",
            ));
        }
        Ok(())
    }
}

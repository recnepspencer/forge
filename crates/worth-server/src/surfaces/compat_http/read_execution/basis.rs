use crate::{
    WorthServerBranchTarget, WorthServerCompatibilityPreparedRequest,
    WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerExternalBasisRequest {
    branch_target: WorthServerBranchTarget,
    requested_basis_digest: Option<String>,
    canonical_digest: String,
}

impl WorthServerExternalBasisRequest {
    pub(crate) fn from_prepared_request(
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<Self, WorthServerQueryHandoffDenial> {
        let request_context = prepared_request.admission().request_context();
        let basis_values = prepared_request
            .request_contract()
            .normalized_query_pairs()
            .iter()
            .filter(|(name, _)| name == "basis")
            .map(|(_, value)| value.trim())
            .collect::<Vec<&str>>();

        if basis_values.iter().any(|value| value.is_empty()) {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid,
                request_context.diagnostics_profile(),
                "compatibility basis query parameter may not be blank",
            ));
        }
        if basis_values.len() > 1 {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid,
                request_context.diagnostics_profile(),
                "compatibility read admits at most one canonical basis query parameter",
            ));
        }
        if matches!(
            request_context.branch_target(),
            WorthServerBranchTarget::Preview { .. }
        ) && !basis_values.is_empty()
        {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestUnsupported,
                request_context.diagnostics_profile(),
                "preview-targeted compatibility reads do not admit an additional explicit basis digest",
            ));
        }

        let branch_target = request_context.branch_target().clone();
        let requested_basis_digest = basis_values.first().map(|value| (*value).to_string());
        let canonical_digest = format!(
            "compat-http-basis-v1|branch:{}|basis:{}",
            branch_target.branch_digest(),
            requested_basis_digest.as_deref().unwrap_or("none"),
        );
        Ok(Self {
            branch_target,
            requested_basis_digest,
            canonical_digest,
        })
    }

    pub fn branch_target(&self) -> &WorthServerBranchTarget {
        &self.branch_target
    }

    pub fn requested_basis_digest(&self) -> Option<&str> {
        self.requested_basis_digest.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn validate_observed_basis(
        &self,
        diagnostics_profile: worth_foundational::facade::DiagnosticRichnessProfile,
        observed_basis_digest: Option<&str>,
    ) -> Result<(), WorthServerQueryHandoffDenial> {
        let Some(requested_basis_digest) = self.requested_basis_digest() else {
            return Ok(());
        };

        let Some(observed_basis_digest) = observed_basis_digest else {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestUnsupported,
                diagnostics_profile,
                "compatibility basis targeting requires an observed retained basis digest",
            ));
        };

        if requested_basis_digest != observed_basis_digest {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid,
                diagnostics_profile,
                format!(
                    "compatibility basis request `{requested_basis_digest}` drifted from the admitted retained basis `{observed_basis_digest}`",
                ),
            ));
        }

        Ok(())
    }
}

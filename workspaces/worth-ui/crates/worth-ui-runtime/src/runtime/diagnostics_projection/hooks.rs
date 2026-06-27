use crate::runtime::diagnostics_projection::WorthUiDiagnosticsSurfaceBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticsProjectionHook {
    binding: WorthUiDiagnosticsSurfaceBinding,
    effect: WorthUiDiagnosticsProjectionHookEffect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDiagnosticsProjectionHookEffect {
    PresentationOnly,
    IdentityRewriteAttempt { attempted_code: String },
}

impl WorthUiDiagnosticsProjectionHook {
    pub fn surface(surface_id: impl Into<String>) -> Self {
        Self {
            binding: WorthUiDiagnosticsSurfaceBinding::new(surface_id),
            effect: WorthUiDiagnosticsProjectionHookEffect::PresentationOnly,
        }
    }

    pub fn binding(&self) -> &WorthUiDiagnosticsSurfaceBinding {
        &self.binding
    }

    pub fn effect(&self) -> &WorthUiDiagnosticsProjectionHookEffect {
        &self.effect
    }

    pub fn projection_digest(&self) -> u64 {
        self.binding.surface_digest()
    }

    #[cfg(test)]
    pub(crate) fn identity_rewrite_attempt_for_test(
        surface_id: impl Into<String>,
        attempted_code: impl Into<String>,
    ) -> Self {
        Self {
            binding: WorthUiDiagnosticsSurfaceBinding::new(surface_id),
            effect: WorthUiDiagnosticsProjectionHookEffect::IdentityRewriteAttempt {
                attempted_code: attempted_code.into(),
            },
        }
    }
}

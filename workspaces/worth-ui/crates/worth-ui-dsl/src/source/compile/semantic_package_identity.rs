use std::fmt;
use std::sync::Arc;

use super::semantic_package_exact_basis::WorthUiSemanticPackageExactBasis;
use crate::source::{WorthUiSemanticModule, WorthUiSourceModuleId};

/// Collision-safe identity of canonical DSL meaning.
///
/// The fingerprint is only a narrowing hint. Equality always confirms the
/// complete canonical basis, so an equal fingerprint cannot authorize aliasing.
#[derive(Clone)]
pub struct WorthUiSemanticPackageIdentity {
    narrowing_fingerprint: u64,
    exact_basis: Arc<WorthUiSemanticPackageExactBasis>,
}

impl WorthUiSemanticPackageIdentity {
    pub(super) fn from_modules<'module>(
        modules: impl IntoIterator<
            Item = (
                &'module WorthUiSourceModuleId,
                &'module WorthUiSemanticModule,
            ),
        >,
    ) -> Self {
        let exact_basis = Arc::new(WorthUiSemanticPackageExactBasis::from_modules(modules));
        Self {
            narrowing_fingerprint: exact_basis.narrowing_fingerprint(),
            exact_basis,
        }
    }

    pub fn narrowing_fingerprint(&self) -> u64 {
        self.narrowing_fingerprint
    }

    #[cfg(test)]
    pub(crate) fn with_narrowing_fingerprint_for_test(
        mut self,
        narrowing_fingerprint: u64,
    ) -> Self {
        self.narrowing_fingerprint = narrowing_fingerprint;
        self
    }
}

impl PartialEq for WorthUiSemanticPackageIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.narrowing_fingerprint == other.narrowing_fingerprint
            && self.exact_basis == other.exact_basis
    }
}

impl Eq for WorthUiSemanticPackageIdentity {}

impl fmt::Debug for WorthUiSemanticPackageIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiSemanticPackageIdentity")
            .field("narrowing_fingerprint", &self.narrowing_fingerprint)
            .finish_non_exhaustive()
    }
}

use crate::ProtectedPhysicalByteView;
use worth_store::physical_runtime::VerificationPhysicalAllocation;

#[derive(Debug)]
pub struct IntegrityEntryRequest<'runtime, 'lease> {
    protected_view: ProtectedPhysicalByteView<'lease>,
    verification: VerificationPhysicalAllocation<'runtime>,
}

impl<'runtime, 'lease> IntegrityEntryRequest<'runtime, 'lease> {
    pub const fn new(
        protected_view: ProtectedPhysicalByteView<'lease>,
        verification: VerificationPhysicalAllocation<'runtime>,
    ) -> Self {
        Self {
            protected_view,
            verification,
        }
    }

    pub const fn protected_view(self) -> ProtectedPhysicalByteView<'lease> {
        self.protected_view
    }

    pub(crate) const fn protected_view_ref(&self) -> ProtectedPhysicalByteView<'lease> {
        self.protected_view
    }

    pub(crate) const fn verification(&self) -> &VerificationPhysicalAllocation<'runtime> {
        &self.verification
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ProtectedPhysicalByteView<'lease>,
        VerificationPhysicalAllocation<'runtime>,
    ) {
        (self.protected_view, self.verification)
    }
}

use crate::{IntegrityEntryWitness, ProtectedPhysicalByteView};
use worth_store::physical_runtime::VerificationPhysicalAllocation;

#[derive(Debug)]
pub struct IntegrityInspectionLease<'runtime, 'lease> {
    protected_view: ProtectedPhysicalByteView<'lease>,
    verification: VerificationPhysicalAllocation<'runtime>,
    entry_witness: IntegrityEntryWitness,
}

impl<'runtime, 'lease> IntegrityInspectionLease<'runtime, 'lease> {
    pub(crate) const fn new(
        protected_view: ProtectedPhysicalByteView<'lease>,
        verification: VerificationPhysicalAllocation<'runtime>,
        entry_witness: IntegrityEntryWitness,
    ) -> Self {
        Self {
            protected_view,
            verification,
            entry_witness,
        }
    }

    pub const fn protected_bytes(&self) -> ProtectedPhysicalByteView<'lease> {
        self.protected_view
    }

    pub const fn entry_witness(&self) -> IntegrityEntryWitness {
        self.entry_witness
    }

    pub const fn verification(&self) -> &VerificationPhysicalAllocation<'runtime> {
        &self.verification
    }
}

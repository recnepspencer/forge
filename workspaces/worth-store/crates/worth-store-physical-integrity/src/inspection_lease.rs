use crate::{IntegrityEntryWitness, ProtectedPhysicalByteView, ScrubEnvelopeLimits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityInspectionLease<'lease> {
    protected_view: ProtectedPhysicalByteView<'lease>,
    entry_witness: IntegrityEntryWitness,
}

impl<'lease> IntegrityInspectionLease<'lease> {
    pub(crate) const fn new(
        protected_view: ProtectedPhysicalByteView<'lease>,
        entry_witness: IntegrityEntryWitness,
    ) -> Self {
        Self {
            protected_view,
            entry_witness,
        }
    }

    pub const fn protected_bytes(self) -> ProtectedPhysicalByteView<'lease> {
        self.protected_view
    }

    pub const fn entry_witness(self) -> IntegrityEntryWitness {
        self.entry_witness
    }

    pub const fn scrub_envelope_limits(self) -> ScrubEnvelopeLimits {
        self.entry_witness.scrub_envelope_limits()
    }
}

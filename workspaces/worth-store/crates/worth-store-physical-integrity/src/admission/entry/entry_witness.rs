use crate::IntegrityEntryBasis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryWitness {
    basis: IntegrityEntryBasis,
}

impl IntegrityEntryWitness {
    pub(crate) const fn mint(basis: IntegrityEntryBasis) -> Self {
        Self { basis }
    }

    pub const fn entry_basis(self) -> IntegrityEntryBasis {
        self.basis
    }

    pub const fn proves_recovery_behavior(self) -> bool {
        false
    }

    pub const fn proves_blob_lifecycle(self) -> bool {
        false
    }

    pub const fn proves_repair_behavior(self) -> bool {
        false
    }

    pub const fn proves_authenticity(self) -> bool {
        false
    }

    pub const fn proves_certification_closeout(self) -> bool {
        false
    }
}

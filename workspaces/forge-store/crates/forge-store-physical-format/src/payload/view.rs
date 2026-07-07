use crate::PhysicalHeaderDecodeWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPayloadViewAdmission<'a> {
    view: PhysicalPayloadView<'a>,
}

impl<'a> PhysicalPayloadViewAdmission<'a> {
    pub(crate) const fn new(view: PhysicalPayloadView<'a>) -> Self {
        Self { view }
    }

    pub const fn view(self) -> PhysicalPayloadView<'a> {
        self.view
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPayloadView<'a> {
    bytes: &'a [u8],
    witness: PhysicalHeaderDecodeWitness,
}

impl<'a> PhysicalPayloadView<'a> {
    pub(crate) const fn new(bytes: &'a [u8], witness: PhysicalHeaderDecodeWitness) -> Self {
        Self { bytes, witness }
    }

    pub const fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }

    pub const fn witness(self) -> PhysicalHeaderDecodeWitness {
        self.witness
    }
}

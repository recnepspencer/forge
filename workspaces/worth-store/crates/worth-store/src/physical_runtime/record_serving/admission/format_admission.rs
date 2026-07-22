use worth_store_physical_format::PhysicalRecordFormatDeclaration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPhysicalRecordFormat(PhysicalRecordFormatDeclaration);

impl AdmittedPhysicalRecordFormat {
    pub const fn admit(declaration: PhysicalRecordFormatDeclaration) -> Self {
        Self(declaration)
    }

    pub const fn declaration(self) -> PhysicalRecordFormatDeclaration {
        self.0
    }
}

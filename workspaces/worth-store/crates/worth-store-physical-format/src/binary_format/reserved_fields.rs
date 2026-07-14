#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReservedFieldPolicy {
    ZeroedAndPreserved,
}

impl PhysicalReservedFieldPolicy {
    pub const fn zeroed_and_preserved() -> Self {
        Self::ZeroedAndPreserved
    }

    pub const fn code(self) -> u8 {
        match self {
            Self::ZeroedAndPreserved => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReservedFieldPolicyDeclaration {
    Known(PhysicalReservedFieldPolicy),
    Unknown,
}

impl From<PhysicalReservedFieldPolicy> for PhysicalReservedFieldPolicyDeclaration {
    fn from(value: PhysicalReservedFieldPolicy) -> Self {
        Self::Known(value)
    }
}

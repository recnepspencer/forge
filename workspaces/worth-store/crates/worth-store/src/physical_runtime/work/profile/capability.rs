#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalWorkSignalFamily {
    ReadFault,
    ExactWriteback,
    Publication,
    Lifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalWorkSignalFamilySet(u8);

impl PhysicalWorkSignalFamilySet {
    pub const fn none() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(0b1111)
    }

    pub const fn only(family: PhysicalWorkSignalFamily) -> Self {
        Self(family_bit(family))
    }

    pub const fn with(mut self, family: PhysicalWorkSignalFamily) -> Self {
        self.0 |= family_bit(family);
        self
    }

    pub const fn contains(self, family: PhysicalWorkSignalFamily) -> bool {
        self.0 & family_bit(family) != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub(super) const fn bits(self) -> u8 {
        self.0
    }
}

const fn family_bit(family: PhysicalWorkSignalFamily) -> u8 {
    match family {
        PhysicalWorkSignalFamily::ReadFault => 1 << 0,
        PhysicalWorkSignalFamily::ExactWriteback => 1 << 1,
        PhysicalWorkSignalFamily::Publication => 1 << 2,
        PhysicalWorkSignalFamily::Lifecycle => 1 << 3,
    }
}

#[derive(Clone, Copy)]
pub(in crate::physical_runtime) struct PhysicalAsyncCapabilitySpec {
    family: PhysicalWorkSignalFamily,
    contract_id: u64,
    max_payload_bytes: u64,
}

impl PhysicalAsyncCapabilitySpec {
    pub(in crate::physical_runtime) const fn family(self) -> PhysicalWorkSignalFamily {
        self.family
    }
    pub(in crate::physical_runtime) const fn contract_id(self) -> u64 {
        self.contract_id
    }
    pub(in crate::physical_runtime) const fn max_payload_bytes(self) -> u64 {
        self.max_payload_bytes
    }
}

pub(in crate::physical_runtime) const PHYSICAL_ASYNC_CAPABILITIES: [PhysicalAsyncCapabilitySpec;
    4] = [
    PhysicalAsyncCapabilitySpec {
        family: PhysicalWorkSignalFamily::ReadFault,
        contract_id: 1,
        max_payload_bytes: 64,
    },
    PhysicalAsyncCapabilitySpec {
        family: PhysicalWorkSignalFamily::ExactWriteback,
        contract_id: 2,
        max_payload_bytes: 64,
    },
    PhysicalAsyncCapabilitySpec {
        family: PhysicalWorkSignalFamily::Publication,
        contract_id: 3,
        max_payload_bytes: 64,
    },
    PhysicalAsyncCapabilitySpec {
        family: PhysicalWorkSignalFamily::Lifecycle,
        contract_id: 4,
        max_payload_bytes: 32,
    },
];

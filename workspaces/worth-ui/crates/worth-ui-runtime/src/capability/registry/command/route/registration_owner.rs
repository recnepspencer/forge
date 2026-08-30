/// Opaque future plugin-registration identity. Milestone 3.15 does not expose
/// authority to mint extension owners.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRegistrationOwnerIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRegistrationGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRegistrationOwner {
    identity: UiCommandRegistrationOwnerIdentity,
    generation: UiCommandRegistrationGeneration,
}

impl UiCommandRegistrationOwnerIdentity {
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiCommandRegistrationGeneration {
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl UiCommandRegistrationOwner {
    pub const fn identity(self) -> UiCommandRegistrationOwnerIdentity {
        self.identity
    }

    pub const fn generation(self) -> UiCommandRegistrationGeneration {
        self.generation
    }
}

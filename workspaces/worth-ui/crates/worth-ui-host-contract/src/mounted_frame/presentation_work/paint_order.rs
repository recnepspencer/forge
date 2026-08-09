use super::UiMountedPaintCommandIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPaintOrderIdentity {
    command: UiMountedPaintCommandIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPaintOrderIntegrity {
    digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPaintOrderEdit {
    Remove(UiMountedPaintOrderIdentity),
    PlaceAfter {
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    },
}

impl UiMountedPaintOrderIdentity {
    #[doc(hidden)]
    pub const fn for_command(command: UiMountedPaintCommandIdentity) -> Self {
        Self { command }
    }

    pub const fn command(self) -> UiMountedPaintCommandIdentity {
        self.command
    }
}

impl UiMountedPaintOrderIntegrity {
    pub fn for_order(order: &[UiMountedPaintOrderIdentity]) -> Self {
        let digest = order
            .iter()
            .fold(0xcbf2_9ce4_8422_2325_u64, |digest, identity| {
                digest
                    .wrapping_mul(0x0000_0100_0000_01b3)
                    .wrapping_add(identity.command.order_fingerprint())
            });
        Self { digest }
    }

    pub fn admits(self, order: &[UiMountedPaintOrderIdentity]) -> bool {
        self == Self::for_order(order)
    }
}

impl UiMountedPaintOrderEdit {
    #[doc(hidden)]
    pub const fn place_after(
        identity: UiMountedPaintOrderIdentity,
        predecessor: Option<UiMountedPaintOrderIdentity>,
    ) -> Self {
        Self::PlaceAfter {
            identity,
            predecessor,
        }
    }

    #[doc(hidden)]
    pub const fn remove(identity: UiMountedPaintOrderIdentity) -> Self {
        Self::Remove(identity)
    }

    pub const fn identity(self) -> UiMountedPaintOrderIdentity {
        match self {
            Self::Remove(identity) | Self::PlaceAfter { identity, .. } => identity,
        }
    }

    pub const fn predecessor(self) -> Option<UiMountedPaintOrderIdentity> {
        match self {
            Self::Remove(_) => None,
            Self::PlaceAfter { predecessor, .. } => predecessor,
        }
    }

    pub const fn is_removal(self) -> bool {
        matches!(self, Self::Remove(_))
    }
}

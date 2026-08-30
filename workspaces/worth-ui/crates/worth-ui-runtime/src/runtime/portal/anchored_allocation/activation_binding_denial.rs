#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalActivationBindingDenial {
    NeighborhoodMismatch { ordinal: u16 },
    AnchorIdentityMismatch { ordinal: u16 },
    HostWitnessMismatch { ordinal: u16 },
    CardinalityExceeded,
}

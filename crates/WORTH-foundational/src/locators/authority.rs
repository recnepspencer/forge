#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocatorAuthority {
    Authoritative,
    Derived,
    Projected,
    SupportOnly,
    Planned,
    ReceiptBearing,
}

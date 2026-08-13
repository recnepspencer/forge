use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

/// Application-owned typed association to one portable boundary protocol.
///
/// Foundational owns the generic identity and version vocabulary. Query owns
/// only the fact that this exact application payload produces that protocol.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApplicationExternalEffectProtocol {
    identity: BoundaryProtocolIdentity,
    version: BoundaryProtocolVersion,
}

impl ApplicationExternalEffectProtocol {
    pub const fn new(identity: BoundaryProtocolIdentity, version: BoundaryProtocolVersion) -> Self {
        Self { identity, version }
    }

    pub const fn identity(&self) -> &BoundaryProtocolIdentity {
        &self.identity
    }

    pub const fn version(&self) -> BoundaryProtocolVersion {
        self.version
    }
}

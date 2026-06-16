use forge_foundational::facade::{
    FoundationalAuthorityIdentity, FoundationalBoundaryBridgedIdentity,
    FoundationalDigestIdentityEvidence, FoundationalExternalIdentityToken,
    FoundationalProjectionIdentity,
};

use super::authority::BridgeTruthAuthority;

pub type BridgeTruthAuthorityIdentity<Value, Kind> =
    FoundationalAuthorityIdentity<Value, BridgeTruthAuthority, Kind>;

pub type BridgeTruthBoundaryBridgedIdentity<Value, Kind> =
    FoundationalBoundaryBridgedIdentity<Value, BridgeTruthAuthority, Kind>;

pub type BridgeTruthExternalIdentityToken<Value, Kind> =
    FoundationalExternalIdentityToken<Value, Kind>;

pub type BridgeTruthProjectionIdentity<Label, Kind> = FoundationalProjectionIdentity<Label, Kind>;

pub type BridgeTruthDigestIdentityEvidence<Basis, Kind> =
    FoundationalDigestIdentityEvidence<Basis, BridgeTruthAuthority, Kind>;

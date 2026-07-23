use std::sync::Arc;

use worth_foundational::facade::{
    admit_foundational_authority_identity, project_foundational_identity,
    FoundationalAuthorityIdentity, FoundationalIdentityKind, FoundationalProjectionIdentity,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct BridgeConditionalLoweringIdentityKind;

impl FoundationalIdentityKind for BridgeConditionalLoweringIdentityKind {}

struct BridgeConditionalLoweringAuthority {
    _owner_seal: (),
}

impl AuthorityMarker for BridgeConditionalLoweringAuthority {}

pub(super) struct BridgeInstalledConditionalLoweringAuthorityIdentity {
    _identity: FoundationalAuthorityIdentity<
        Arc<str>,
        BridgeConditionalLoweringAuthority,
        BridgeConditionalLoweringIdentityKind,
    >,
}

pub type BridgeConditionalLoweringProjectionIdentity =
    FoundationalProjectionIdentity<Arc<str>, BridgeConditionalLoweringIdentityKind>;

pub(super) fn mint_bridge_conditional_lowering_identity(
    projection_basis: String,
) -> (
    BridgeInstalledConditionalLoweringAuthorityIdentity,
    BridgeConditionalLoweringProjectionIdentity,
) {
    let projection_basis: Arc<str> = projection_basis.into();
    let identity =
        admit_foundational_authority_identity(Arc::clone(&projection_basis), lowering_authority());
    let projection =
        project_foundational_identity(&identity, projection_basis, lowering_authority());
    (
        BridgeInstalledConditionalLoweringAuthorityIdentity {
            _identity: identity,
        },
        projection,
    )
}

fn lowering_authority() -> AuthorityWitness<BridgeConditionalLoweringAuthority> {
    AuthorityWitness::from_authority_marker(BridgeConditionalLoweringAuthority { _owner_seal: () })
}

use worth_foundational::facade::{
    admit_foundational_authority_identity, FoundationalAuthorityIdentity, FoundationalIdentityKind,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_schema_graph::facade::{
    CarryingArtifactIdentity, DurableReferenceKind, GraphPromotionIdentityBasis, SubelementKey,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthQueryGraphPromotionIdentityAuthority(());

impl AuthorityMarker for WorthQueryGraphPromotionIdentityAuthority {}

struct WorthQueryPromotedGraphIdentityKind;

impl FoundationalIdentityKind for WorthQueryPromotedGraphIdentityKind {}

type AdmittedGraphPromotionIdentity = FoundationalAuthorityIdentity<
    GraphPromotionIdentityBasis,
    WorthQueryGraphPromotionIdentityAuthority,
    WorthQueryPromotedGraphIdentityKind,
>;

/// Query-admitted operational graph identity produced from Schema Graph's pure
/// promotion grammar. Its Foundational authority identity cannot be assembled
/// from the publicly constructible grammar basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPromotedGraphIdentity {
    admitted: AdmittedGraphPromotionIdentity,
}

impl WorthQueryPromotedGraphIdentity {
    pub const fn reference_kind(&self) -> DurableReferenceKind {
        self.admitted.value().reference_kind()
    }

    pub const fn carrying_artifact_identity(&self) -> &CarryingArtifactIdentity {
        self.admitted.value().carrying_artifact_identity()
    }

    pub const fn subelement_key(&self) -> &SubelementKey {
        self.admitted.value().subelement_key()
    }
}

pub(super) fn admit_graph_promotion_identity(
    basis: GraphPromotionIdentityBasis,
) -> WorthQueryPromotedGraphIdentity {
    WorthQueryPromotedGraphIdentity {
        admitted: admit_foundational_authority_identity(
            basis,
            AuthorityWitness::from_authority_marker(WorthQueryGraphPromotionIdentityAuthority(())),
        ),
    }
}

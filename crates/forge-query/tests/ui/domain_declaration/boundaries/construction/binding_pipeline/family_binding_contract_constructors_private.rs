use forge_query::facade::{
    ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity, ForgeQueryDeclarationAspectContract,
    ForgeQueryFamilyBindingContract, ForgeQueryFamilyContextExtractorContract,
    ForgeQueryFamilyTargetResolverContract,
};

fn main() {
    let contract = ForgeQueryDeclarationAspectContract::empty();
    let _ = ForgeQueryFamilyBindingContract {
        family_key: "family",
        required_aspect_contract: contract.clone(),
    };
    let _ = ForgeQueryFamilyContextExtractorContract {
        family_key: "family",
        allowed_sources: vec![ForgeQueryBindingSourceKind::ExplicitSelection],
        required_aspect_contract: contract.clone(),
    };
    let _ = ForgeQueryFamilyTargetResolverContract {
        family_key: "family",
        required_aspect_contract: contract,
        route_intent: None,
        specificity_rank: ForgeQueryBindingSpecificity::ExactExplicit,
    };
}

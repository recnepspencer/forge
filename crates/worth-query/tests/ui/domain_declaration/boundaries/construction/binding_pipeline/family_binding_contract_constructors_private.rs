use worth_query::facade::foundation::{WorthQueryBindingSourceKind, WorthQueryBindingSpecificity, WorthQueryDeclarationAspectContract, WorthQueryFamilyBindingContract, WorthQueryFamilyContextExtractorContract, WorthQueryFamilyTargetResolverContract};

fn main() {
    let contract = WorthQueryDeclarationAspectContract::empty();
    let _ = WorthQueryFamilyBindingContract {
        family_key: "family",
        required_aspect_contract: contract.clone(),
    };
    let _ = WorthQueryFamilyContextExtractorContract {
        family_key: "family",
        allowed_sources: vec![WorthQueryBindingSourceKind::ExplicitSelection],
        required_aspect_contract: contract.clone(),
    };
    let _ = WorthQueryFamilyTargetResolverContract {
        family_key: "family",
        required_aspect_contract: contract,
        route_intent: None,
        specificity_rank: WorthQueryBindingSpecificity::ExactExplicit,
    };
}

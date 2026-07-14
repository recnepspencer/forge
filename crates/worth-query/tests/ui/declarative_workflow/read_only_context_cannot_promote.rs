use worth_query::facade::preview::{
    WorthQueryPromotionEligiblePreviewDeclaration, WorthQueryReadOnlyPreviewContext,
};

fn cannot_promote(
    declaration: WorthQueryPromotionEligiblePreviewDeclaration,
    context: WorthQueryReadOnlyPreviewContext,
) {
    let _request = declaration.using(context);
}

fn main() {}

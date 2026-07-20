use super::{
    WorthQueryPromotionEligiblePreviewDeclaration, WorthQueryPromotionPreviewContext,
    WorthQueryReadOnlyPreviewContext, WorthQueryReadOnlyPreviewDeclaration,
};

pub struct WorthQueryReadOnlyPreviewRequest {
    pub(crate) declaration: WorthQueryReadOnlyPreviewDeclaration,
    pub(crate) context: WorthQueryReadOnlyPreviewContext,
}

pub struct WorthQueryPromotionEligiblePreviewRequest {
    pub(crate) declaration: WorthQueryPromotionEligiblePreviewDeclaration,
    pub(crate) context: WorthQueryPromotionPreviewContext,
}

impl WorthQueryReadOnlyPreviewDeclaration {
    pub fn using(
        self,
        context: WorthQueryReadOnlyPreviewContext,
    ) -> WorthQueryReadOnlyPreviewRequest {
        WorthQueryReadOnlyPreviewRequest {
            declaration: self,
            context,
        }
    }
}

impl WorthQueryPromotionEligiblePreviewDeclaration {
    pub fn using(
        self,
        context: WorthQueryPromotionPreviewContext,
    ) -> WorthQueryPromotionEligiblePreviewRequest {
        WorthQueryPromotionEligiblePreviewRequest {
            declaration: self,
            context,
        }
    }
}

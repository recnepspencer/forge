use super::{
    WorthQueryPreviewContext, WorthQueryPromotionEligiblePreviewDeclaration,
    WorthQueryReadOnlyPreviewDeclaration,
};

pub struct WorthQueryReadOnlyPreviewRequest {
    pub(crate) declaration: WorthQueryReadOnlyPreviewDeclaration,
    pub(crate) context: WorthQueryPreviewContext,
}

pub struct WorthQueryPromotionEligiblePreviewRequest {
    pub(crate) declaration: WorthQueryPromotionEligiblePreviewDeclaration,
    pub(crate) context: WorthQueryPreviewContext,
}

impl WorthQueryReadOnlyPreviewDeclaration {
    pub fn using(self, context: WorthQueryPreviewContext) -> WorthQueryReadOnlyPreviewRequest {
        WorthQueryReadOnlyPreviewRequest {
            declaration: self,
            context,
        }
    }
}

impl WorthQueryPromotionEligiblePreviewDeclaration {
    pub fn using(
        self,
        context: WorthQueryPreviewContext,
    ) -> WorthQueryPromotionEligiblePreviewRequest {
        WorthQueryPromotionEligiblePreviewRequest {
            declaration: self,
            context,
        }
    }
}

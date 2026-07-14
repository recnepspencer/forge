use crate::ordinary::read::WorthQueryReadCompletion;
use crate::ordinary::WorthQueryOrdinaryInspectionPolicy;
use crate::runtime::WorthQueryReadReceipt;

use super::{WorthQueryInspectionContext, WorthQueryInspectionRequest};

pub struct WorthQueryInspectionDeclaration {
    pub(super) source_receipt: WorthQueryReadReceipt,
    pub(super) inspection_policy: WorthQueryOrdinaryInspectionPolicy,
}

impl WorthQueryInspectionDeclaration {
    pub fn with_rich_inspection(mut self) -> Self {
        self.inspection_policy = WorthQueryOrdinaryInspectionPolicy::Rich;
        self
    }

    pub fn using(self, context: WorthQueryInspectionContext) -> WorthQueryInspectionRequest {
        WorthQueryInspectionRequest {
            declaration: self,
            context,
        }
    }
}

pub fn inspect(completion: &WorthQueryReadCompletion) -> WorthQueryInspectionDeclaration {
    WorthQueryInspectionDeclaration {
        source_receipt: completion.result().receipt().clone(),
        inspection_policy: WorthQueryOrdinaryInspectionPolicy::OperationalOnly,
    }
}

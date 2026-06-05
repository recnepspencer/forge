use super::*;

impl RuntimeBridge {
    /// Orders one admitted mixed-cause set into canonical ordered, suppressed,
    /// and denied bridge artifacts without relying on host callback order.
    pub fn order_mixed_causes(
        &self,
        request: &BridgeMixedCauseOrderingRequest,
    ) -> BridgeMixedCauseOrdering {
        let _ = self;
        BridgeMixedCauseOrdering::order(request)
    }

    /// Lowers one canonical mixed-cause ordering into a delivery-ready ordered
    /// window artifact for later fanout and execution phases.
    pub fn plan_mixed_cause_delivery_window(
        &self,
        ordering: &BridgeMixedCauseOrdering,
        delivery_family_kind: BridgeSubscriptionDeliveryFamilyKind,
    ) -> Result<BridgeMixedCauseDeliveryWindowPlan, BridgeMixedCauseDeliveryWindowRejection> {
        let _ = self;
        BridgeMixedCauseDeliveryWindowPlan::plan(ordering, delivery_family_kind)
    }
}

use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub(crate) struct WorthQueryRetainedOwnerDeliveryClassification {
    conditional: Arc<crate::domain_installation::WorthQueryConditionalProvenance>,
    impact: Arc<crate::domain_installation::WorthQueryImpactDecision>,
}

impl WorthQueryRetainedOwnerDeliveryClassification {
    pub(crate) fn new(
        conditional: Arc<crate::domain_installation::WorthQueryConditionalProvenance>,
        impact: Arc<crate::domain_installation::WorthQueryImpactDecision>,
    ) -> Self {
        Self {
            conditional,
            impact,
        }
    }

    pub(crate) fn conditional(
        &self,
    ) -> &crate::domain_installation::WorthQueryConditionalProvenance {
        &self.conditional
    }

    pub(crate) fn conditional_arc(
        &self,
    ) -> &Arc<crate::domain_installation::WorthQueryConditionalProvenance> {
        &self.conditional
    }

    pub(crate) fn impact(&self) -> &Arc<crate::domain_installation::WorthQueryImpactDecision> {
        &self.impact
    }
}

#[derive(Default)]
pub(super) struct WorthQueryOwnerDeliveryContinuation {
    seed: OnceLock<Arc<worth_runtime_bridge::facade::BridgeRetainedConditionalDecisionSeed>>,
}

impl WorthQueryOwnerDeliveryContinuation {
    pub(super) fn seed(
        &self,
    ) -> Option<Arc<worth_runtime_bridge::facade::BridgeRetainedConditionalDecisionSeed>> {
        self.seed.get().cloned()
    }

    pub(super) fn retain_seed(
        &self,
        seed: worth_runtime_bridge::facade::BridgeRetainedConditionalDecisionSeed,
    ) -> Arc<worth_runtime_bridge::facade::BridgeRetainedConditionalDecisionSeed> {
        if let Some(retained) = self.seed() {
            return retained;
        }
        let _ = self.seed.set(Arc::new(seed));
        Arc::clone(
            self.seed
                .get()
                .expect("owner delivery decision seed was retained"),
        )
    }
}

#[derive(Default)]
pub(super) struct WorthQueryOwnerDeliveryTargetContinuation {
    classification: OnceLock<WorthQueryRetainedOwnerDeliveryClassification>,
}

impl WorthQueryOwnerDeliveryTargetContinuation {
    pub(super) fn retained(&self) -> Option<WorthQueryRetainedOwnerDeliveryClassification> {
        self.classification.get().cloned()
    }

    pub(super) fn retain(
        &self,
        classification: WorthQueryRetainedOwnerDeliveryClassification,
    ) -> WorthQueryRetainedOwnerDeliveryClassification {
        if let Some(retained) = self.retained() {
            return retained;
        }
        let _ = self.classification.set(classification);
        self.classification
            .get()
            .expect("target owner delivery classification was retained")
            .clone()
    }
}

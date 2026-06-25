use crate::runtime::{
    WorthUiPrimitiveObservedPostureReceipt, WorthUiPrimitiveProofReceipt,
    WorthUiResolvedAppearanceStateReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveActiveAppearancePlan {
    surface_id: String,
    produced_fact: WorthUiRuntimeFactId,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    observed_posture: WorthUiPrimitiveObservedPostureReceipt,
    active_appearance: WorthUiResolvedAppearanceStateReceipt,
    receipt_digest: u64,
}

impl WorthUiPrimitiveActiveAppearancePlan {
    pub(crate) fn from_receipt(
        receipt: &WorthUiPrimitiveProofReceipt,
        observed_posture: WorthUiPrimitiveObservedPostureReceipt,
    ) -> Self {
        let surface_id = receipt.surface_id().to_owned();
        let active_appearance = receipt
            .appearance_state()
            .resolve_active(observed_posture.posture());
        let produced_fact = WorthUiRuntimeFactId::primitive_active_appearance(&surface_id);
        let consumed_facts = vec![
            WorthUiRuntimeFactId::primitive_appearance_state(&surface_id),
            WorthUiRuntimeFactId::primitive_interaction(&surface_id),
            observed_posture.active_appearance_fact().clone(),
        ];
        let receipt_digest = active_appearance_plan_digest(
            &surface_id,
            &produced_fact,
            &consumed_facts,
            observed_posture.receipt_digest(),
            active_appearance.receipt_digest(),
        );
        Self {
            surface_id,
            produced_fact,
            consumed_facts,
            observed_posture,
            active_appearance,
            receipt_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn produced_fact(&self) -> &WorthUiRuntimeFactId {
        &self.produced_fact
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn observed_posture(&self) -> &WorthUiPrimitiveObservedPostureReceipt {
        &self.observed_posture
    }

    pub fn active_appearance(&self) -> &WorthUiResolvedAppearanceStateReceipt {
        &self.active_appearance
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn active_appearance_plan_digest(
    surface_id: &str,
    produced_fact: &WorthUiRuntimeFactId,
    consumed_facts: &[WorthUiRuntimeFactId],
    observed_digest: u64,
    active_digest: u64,
) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325;
    for value in [
        "primitive-active-appearance",
        surface_id,
        produced_fact.family().token(),
        produced_fact.identity(),
    ] {
        digest = fold(digest, value.as_bytes());
    }
    for fact in consumed_facts {
        digest = fold(digest, fact.family().token().as_bytes());
        digest = fold(digest, fact.identity().as_bytes());
    }
    for value in [observed_digest, active_digest] {
        digest = fold(digest, &value.to_le_bytes());
    }
    digest
}

fn fold(mut digest: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}

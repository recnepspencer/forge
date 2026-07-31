use worth_foundational::facade::AspectValue;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationDisclosureReceiptPosture {
    Public,
    Governed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationDisclosureReceipt {
    posture: WorthQueryApplicationDisclosureReceiptPosture,
    classification: Option<String>,
    disclosed: Vec<AspectValue>,
    omitted: Vec<AspectValue>,
    capability_authority_identity: Option<String>,
    decision_identity: Option<[u8; 32]>,
    decision_fact_count: usize,
}

impl WorthQueryApplicationDisclosureReceipt {
    pub(super) const fn public() -> Self {
        Self {
            posture: WorthQueryApplicationDisclosureReceiptPosture::Public,
            classification: None,
            disclosed: Vec::new(),
            omitted: Vec::new(),
            capability_authority_identity: None,
            decision_identity: None,
            decision_fact_count: 0,
        }
    }

    pub(super) fn governed(
        classification: impl Into<String>,
        mut disclosed: Vec<AspectValue>,
        mut omitted: Vec<AspectValue>,
        capability_authority_identity: impl Into<String>,
        decision_identity: [u8; 32],
        decision_fact_count: usize,
    ) -> Self {
        disclosed.sort();
        disclosed.dedup();
        omitted.sort();
        omitted.dedup();
        Self {
            posture: WorthQueryApplicationDisclosureReceiptPosture::Governed,
            classification: Some(classification.into()),
            disclosed,
            omitted,
            capability_authority_identity: Some(capability_authority_identity.into()),
            decision_identity: Some(decision_identity),
            decision_fact_count,
        }
    }

    pub const fn posture(&self) -> WorthQueryApplicationDisclosureReceiptPosture {
        self.posture
    }

    pub fn classification(&self) -> Option<&str> {
        self.classification.as_deref()
    }

    pub fn disclosed(&self) -> &[AspectValue] {
        &self.disclosed
    }

    pub fn omitted(&self) -> &[AspectValue] {
        &self.omitted
    }

    pub fn capability_authority_identity(&self) -> Option<&str> {
        self.capability_authority_identity.as_deref()
    }

    pub const fn decision_identity(&self) -> Option<&[u8; 32]> {
        self.decision_identity.as_ref()
    }

    pub const fn decision_fact_count(&self) -> usize {
        self.decision_fact_count
    }
}

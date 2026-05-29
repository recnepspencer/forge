#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRoutePlanExplanation {
    route_contract_reason: &'static str,
    retained_facts: Vec<String>,
    route_segment_reasons: Vec<String>,
    intent_reason: Option<String>,
}

impl ForgeQueryDeclarationRoutePlanExplanation {
    pub(crate) fn new(
        route_contract_reason: &'static str,
        retained_facts: Vec<String>,
        route_segment_reasons: Vec<String>,
        intent_reason: Option<String>,
    ) -> Self {
        Self {
            route_contract_reason,
            retained_facts,
            route_segment_reasons,
            intent_reason,
        }
    }

    pub fn route_contract_reason(&self) -> &'static str {
        self.route_contract_reason
    }

    pub fn retained_facts(&self) -> &[String] {
        &self.retained_facts
    }

    pub fn route_segment_reasons(&self) -> &[String] {
        &self.route_segment_reasons
    }

    pub fn intent_reason(&self) -> Option<&str> {
        self.intent_reason.as_deref()
    }
}

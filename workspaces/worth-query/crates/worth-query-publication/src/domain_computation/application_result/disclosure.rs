use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationDisclosureReceipt, WorthQueryApplicationDisclosureReceiptPosture,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorthQueryPublishedApplicationDisclosureIdentity {
    posture: WorthQueryPublishedApplicationDisclosurePosture,
    disclosure_decision_count: usize,
    disclosed_value_count: usize,
    omitted_value_count: usize,
    authorization_decision_fact_count: usize,
}

impl WorthQueryPublishedApplicationDisclosureIdentity {
    pub(crate) fn boundary_axis(self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            posture_axis(self.posture),
            self.disclosure_decision_count,
            self.disclosed_value_count,
            self.omitted_value_count,
            self.authorization_decision_fact_count
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WorthQueryPublishedApplicationDisclosurePosture {
    Public,
    Governed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublishedApplicationDisclosure {
    identity: WorthQueryPublishedApplicationDisclosureIdentity,
    posture: WorthQueryPublishedApplicationDisclosurePosture,
    disclosure_decision_count: usize,
    disclosed_value_count: usize,
    omitted_value_count: usize,
    authorization_decision_fact_count: usize,
}

impl WorthQueryPublishedApplicationDisclosure {
    pub(super) fn capture(disclosure: &WorthQueryApplicationDisclosureReceipt) -> Self {
        let posture = match disclosure.posture() {
            WorthQueryApplicationDisclosureReceiptPosture::Public => {
                WorthQueryPublishedApplicationDisclosurePosture::Public
            }
            WorthQueryApplicationDisclosureReceiptPosture::Governed => {
                WorthQueryPublishedApplicationDisclosurePosture::Governed
            }
        };
        let disclosure_decision_count = disclosure.disclosure_decision_count();
        let disclosed_value_count = disclosure.disclosed().len();
        let omitted_value_count = disclosure.omitted().len();
        let authorization_decision_fact_count = disclosure.authorization_decision_fact_count();
        let identity = WorthQueryPublishedApplicationDisclosureIdentity {
            posture,
            disclosure_decision_count,
            disclosed_value_count,
            omitted_value_count,
            authorization_decision_fact_count,
        };
        Self {
            identity,
            posture,
            disclosure_decision_count,
            disclosed_value_count,
            omitted_value_count,
            authorization_decision_fact_count,
        }
    }

    pub const fn identity(&self) -> WorthQueryPublishedApplicationDisclosureIdentity {
        self.identity
    }

    pub const fn posture(&self) -> WorthQueryPublishedApplicationDisclosurePosture {
        self.posture
    }

    pub const fn disclosure_decision_count(&self) -> usize {
        self.disclosure_decision_count
    }

    pub const fn disclosed_value_count(&self) -> usize {
        self.disclosed_value_count
    }

    pub const fn omitted_value_count(&self) -> usize {
        self.omitted_value_count
    }

    pub const fn authorization_decision_fact_count(&self) -> usize {
        self.authorization_decision_fact_count
    }

    pub const fn has_omissions(&self) -> bool {
        self.omitted_value_count > 0
    }
}

const fn posture_axis(posture: WorthQueryPublishedApplicationDisclosurePosture) -> &'static str {
    match posture {
        WorthQueryPublishedApplicationDisclosurePosture::Public => "public",
        WorthQueryPublishedApplicationDisclosurePosture::Governed => "governed",
    }
}

#[cfg(test)]
mod identity_tests;

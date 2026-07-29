use super::{UiConsumedFactSelector, UiSubsystemConsumedFactRule};
use crate::fact_contract::{UiProducedFact, UiProducedFactFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConsumedFactContract {
    fact_family: UiProducedFactFamily,
    selector: UiConsumedFactSelector,
}

impl UiConsumedFactContract {
    pub(crate) fn authored(identity: impl Into<Box<str>>) -> UiConsumedFactContract {
        Self {
            fact_family: UiProducedFactFamily::AuthoredSource,
            selector: UiConsumedFactSelector::authored_declaration(identity),
        }
    }

    pub(crate) fn declared_aspect(
        fact_family: UiProducedFactFamily,
        aspect: crate::declaration::UiAspectName,
    ) -> Option<Self> {
        UiSubsystemConsumedFactRule::for_fact_family(fact_family)
            .iter()
            .any(|rule| {
                rule.fact_family() == fact_family
                    && rule.affected_aspect_family() == aspect.family()
            })
            .then(|| Self {
                fact_family,
                selector: UiConsumedFactSelector::aspect(aspect),
            })
    }

    pub(crate) fn matches(&self, fact: &UiProducedFact) -> bool {
        if self.fact_family != fact.family() {
            return false;
        }
        match (&self.selector, fact) {
            (
                UiConsumedFactSelector::AuthoredDeclarationIdentity(expected),
                UiProducedFact::AuthoredSource(authored),
            ) => match authored.selector() {
                crate::fact_contract::UiAuthoredFactSelector::Node(observed) => {
                    expected == observed
                }
                crate::fact_contract::UiAuthoredFactSelector::Module(_) => false,
            },
            (UiConsumedFactSelector::Aspect(_), _) => true,
            _ => false,
        }
    }

    pub const fn fact_family(&self) -> UiProducedFactFamily {
        self.fact_family
    }

    pub const fn selector(&self) -> &UiConsumedFactSelector {
        &self.selector
    }
}

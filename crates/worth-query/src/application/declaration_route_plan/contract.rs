use super::class::{
    WorthQueryDeclarationRouteIntentRequirement, WorthQueryDeclarationRouteMultiplicity,
    WorthQueryLowerAuthorityRouteFamily,
};

const RELATIONAL_ONLY: &[WorthQueryLowerAuthorityRouteFamily] =
    &[WorthQueryLowerAuthorityRouteFamily::Relational];
const BRIDGE_ONLY: &[WorthQueryLowerAuthorityRouteFamily] =
    &[WorthQueryLowerAuthorityRouteFamily::Bridge];
const SIGNAL_ONLY: &[WorthQueryLowerAuthorityRouteFamily] =
    &[WorthQueryLowerAuthorityRouteFamily::Signal];
const RELATIONAL_AND_BRIDGE: &[WorthQueryLowerAuthorityRouteFamily] = &[
    WorthQueryLowerAuthorityRouteFamily::Relational,
    WorthQueryLowerAuthorityRouteFamily::Bridge,
];
const MIXED_ONLY: &[WorthQueryLowerAuthorityRouteFamily] =
    &[WorthQueryLowerAuthorityRouteFamily::Mixed];
const NO_ROUTES: &[WorthQueryLowerAuthorityRouteFamily] = &[];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthQueryDeclarationRouteAutomationAdmission {
    AdmittedByDefault,
    ExpensiveByDefault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRouteContract {
    allowed_route_families: &'static [WorthQueryLowerAuthorityRouteFamily],
    multiplicity: WorthQueryDeclarationRouteMultiplicity,
    intent_requirement: WorthQueryDeclarationRouteIntentRequirement,
    can_defer: bool,
    automation_admission: WorthQueryDeclarationRouteAutomationAdmission,
    signal_routed: bool,
    reason: &'static str,
}

impl WorthQueryDeclarationRouteContract {
    pub fn relational_only() -> Self {
        Self {
            allowed_route_families: RELATIONAL_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration lowers through one relational route",
        }
    }

    pub fn bridge_only() -> Self {
        Self {
            allowed_route_families: BRIDGE_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration lowers through one bridge route",
        }
    }

    pub fn signal_only() -> Self {
        Self {
            allowed_route_families: SIGNAL_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: true,
            reason: "the declaration lowers through one signal route",
        }
    }

    pub fn relational_and_bridge() -> Self {
        Self {
            allowed_route_families: RELATIONAL_AND_BRIDGE,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::PluralCapable,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration may lower through both relational and bridge routes",
        }
    }

    pub fn deferred_auto() -> Self {
        Self {
            allowed_route_families: NO_ROUTES,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: true,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration route remains explicitly deferred",
        }
    }

    pub fn required_relational_intent() -> Self {
        Self {
            allowed_route_families: RELATIONAL_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Required,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration needs caller route intent before relational lowering",
        }
    }

    pub fn relational_intent_forbidden() -> Self {
        Self {
            allowed_route_families: RELATIONAL_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Forbidden,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration does not allow caller-owned route narrowing",
        }
    }

    pub fn unresolved_mixed() -> Self {
        Self {
            allowed_route_families: MIXED_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::PluralCapable,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::AdmittedByDefault,
            signal_routed: false,
            reason: "the declaration route contract still names a classification-only mixed route",
        }
    }

    #[cfg(test)]
    pub(crate) fn expensive_by_default_for_tests() -> Self {
        Self {
            allowed_route_families: RELATIONAL_ONLY,
            multiplicity: WorthQueryDeclarationRouteMultiplicity::Singular,
            intent_requirement: WorthQueryDeclarationRouteIntentRequirement::Optional,
            can_defer: false,
            automation_admission: WorthQueryDeclarationRouteAutomationAdmission::ExpensiveByDefault,
            signal_routed: false,
            reason: "the declaration has one legal route, but ordinary orchestration must stop before automating it by default",
        }
    }

    pub fn allowed_route_families(&self) -> &'static [WorthQueryLowerAuthorityRouteFamily] {
        self.allowed_route_families
    }

    pub fn multiplicity(&self) -> WorthQueryDeclarationRouteMultiplicity {
        self.multiplicity
    }

    pub fn intent_requirement(&self) -> WorthQueryDeclarationRouteIntentRequirement {
        self.intent_requirement
    }

    pub fn can_defer(&self) -> bool {
        self.can_defer
    }

    pub(crate) fn automation_requires_explicit_handoff(&self) -> bool {
        self.automation_admission
            == WorthQueryDeclarationRouteAutomationAdmission::ExpensiveByDefault
    }

    pub fn signal_routed(&self) -> bool {
        self.signal_routed
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

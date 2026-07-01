use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationIdentity, UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
    UiDeclaredHostCapabilityPosture, UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureContract,
    UiDeclaredPostureLane, UiDeclaredPosturePayload, UiDeclaredQueryBindingPosture,
    UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture, UiStructuralDeclarationPayload,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationGraphHandoff {
    identity: UiDeclarationIdentity,
    structural: UiStructuralDeclarationPayload,
    declared_posture: UiDeclaredPosturePayload,
}

impl UiDeclarationGraphHandoff {
    pub(crate) fn new(
        identity: UiDeclarationIdentity,
        structural: UiStructuralDeclarationPayload,
        declared_posture: UiDeclaredPosturePayload,
    ) -> Self {
        Self {
            identity,
            structural,
            declared_posture,
        }
    }

    pub fn identity(&self) -> &UiDeclarationIdentity {
        &self.identity
    }

    pub fn family(&self) -> &UiDeclarationFamily {
        self.structural.family()
    }

    pub const fn family_kind(&self) -> UiDeclarationFamilyKind {
        self.structural.family_kind()
    }

    pub const fn role(&self) -> UiDeclarationStructuralRole {
        self.structural.role()
    }

    pub fn containment_intent(&self) -> &UiDeclarationContainmentIntent {
        self.structural.containment_intent()
    }

    pub fn slot_participation_intent(&self) -> &UiDeclarationSlotParticipationIntent {
        self.structural.slot_participation_intent()
    }

    pub const fn ordering_guarantee(&self) -> UiDeclarationOrderingGuarantee {
        self.structural.ordering_guarantee()
    }

    pub const fn repetition_posture(&self) -> UiDeclarationRepetitionPosture {
        self.structural.repetition_posture()
    }

    pub const fn declared_posture(&self) -> &UiDeclaredPostureContract {
        self.declared_posture.contract()
    }

    pub const fn query_binding(&self) -> &UiDeclaredPostureLane<UiDeclaredQueryBindingPosture> {
        self.declared_posture.query_binding()
    }

    pub const fn service_usage(&self) -> &UiDeclaredPostureLane<UiDeclaredServiceUsagePosture> {
        self.declared_posture.service_usage()
    }

    pub const fn touch_meaning(&self) -> &UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture> {
        self.declared_posture.touch_meaning()
    }

    pub const fn measurement_policy(
        &self,
    ) -> &UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture> {
        self.declared_posture.measurement_policy()
    }

    pub const fn host_capability(&self) -> &UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture> {
        self.declared_posture.host_capability()
    }
}

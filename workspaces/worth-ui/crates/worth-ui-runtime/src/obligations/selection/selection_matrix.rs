use crate::graph::UiGraphNodeRecord;
use crate::obligations::catalog::{UiObligationCheckKind, UiObligationFamily};
use crate::obligations::touch::{
    UiGraphTouchAspectFact, UiGraphTouchDescriptor, UiGraphTouchOriginClass,
    UiGraphTouchRuntimeLane, UiGraphTouchTargetClass,
};

use super::{
    UiObligationSelectionReason, UiObligationSupportBasis, UiObligationSupportSelectionPosture,
    UiObligationWorldProfileClass,
};

const TOUCH_MEANING_SCOPE: [UiGraphTouchRuntimeLane; 1] = [UiGraphTouchRuntimeLane::Structural];
const PARTICIPATION_SCOPE: [UiGraphTouchRuntimeLane; 1] = [UiGraphTouchRuntimeLane::Participation];
const MEASUREMENT_SCOPE: [UiGraphTouchRuntimeLane; 1] = [UiGraphTouchRuntimeLane::Measurement];
const QUERY_BINDING_SCOPE: [UiGraphTouchRuntimeLane; 1] = [UiGraphTouchRuntimeLane::QueryBinding];
const HOST_CAPABILITY_SCOPE: [UiGraphTouchRuntimeLane; 1] =
    [UiGraphTouchRuntimeLane::HostCapability];
const DIAGNOSTIC_SCOPE: [UiGraphTouchRuntimeLane; 1] = [UiGraphTouchRuntimeLane::Diagnostic];

const STARTER_ROWS: [UiObligationSelectionMatrixRow; 8] = [
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::Lane(UiGraphTouchRuntimeLane::Structural),
        UiObligationFamily::StructuralLegality,
        UiObligationCheckKind::BlockingInvariant,
        &TOUCH_MEANING_SCOPE,
        UiObligationSupportBasis::TouchMeaning,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::TargetClass(UiGraphTouchTargetClass::SlotOccupancy),
        UiObligationFamily::SlotContract,
        UiObligationCheckKind::BlockingInvariant,
        &TOUCH_MEANING_SCOPE,
        UiObligationSupportBasis::TouchMeaning,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::Lane(UiGraphTouchRuntimeLane::Participation),
        UiObligationFamily::ParticipationLegality,
        UiObligationCheckKind::BlockingInvariant,
        &PARTICIPATION_SCOPE,
        UiObligationSupportBasis::TouchMeaning,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::OriginAndLane {
            origin: UiGraphTouchOriginClass::HostObservation,
            lane: UiGraphTouchRuntimeLane::Measurement,
        },
        UiObligationFamily::MeasurementRequirement,
        UiObligationCheckKind::PrerequisiteRequirement,
        &MEASUREMENT_SCOPE,
        UiObligationSupportBasis::MeasurementPolicy,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::OriginAndLane {
            origin: UiGraphTouchOriginClass::HostObservation,
            lane: UiGraphTouchRuntimeLane::HostCapability,
        },
        UiObligationFamily::HostCapabilityRequirement,
        UiObligationCheckKind::CapabilityGapScreen,
        &HOST_CAPABILITY_SCOPE,
        UiObligationSupportBasis::HostCapability,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::QueryBindingRequirement,
        UiObligationFamily::QueryBindingRequirement,
        UiObligationCheckKind::PrerequisiteRequirement,
        &QUERY_BINDING_SCOPE,
        UiObligationSupportBasis::QueryBinding,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::ServiceUsageRequirement,
        UiObligationFamily::PortalHostRequirement,
        UiObligationCheckKind::PrerequisiteRequirement,
        &TOUCH_MEANING_SCOPE,
        UiObligationSupportBasis::ServiceUsage,
    ),
    UiObligationSelectionMatrixRow::new(
        UiObligationSelectionRule::DiagnosticSurface,
        UiObligationFamily::DiagnosticSurfaceRequirement,
        UiObligationCheckKind::DiagnosticOnlyCheck,
        &DIAGNOSTIC_SCOPE,
        UiObligationSupportBasis::ServiceUsage,
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiObligationSelectionMatrix;

impl UiObligationSelectionMatrix {
    pub(crate) const fn starter() -> Self {
        Self
    }

    pub(crate) const fn rows(self) -> &'static [UiObligationSelectionMatrixRow] {
        &STARTER_ROWS
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiObligationSelectionMatrixRow {
    rule: UiObligationSelectionRule,
    family: UiObligationFamily,
    check_kind: UiObligationCheckKind,
    aspect_scope: &'static [UiGraphTouchRuntimeLane],
    support_basis: UiObligationSupportBasis,
}

impl UiObligationSelectionMatrixRow {
    const fn new(
        rule: UiObligationSelectionRule,
        family: UiObligationFamily,
        check_kind: UiObligationCheckKind,
        aspect_scope: &'static [UiGraphTouchRuntimeLane],
        support_basis: UiObligationSupportBasis,
    ) -> Self {
        Self {
            rule,
            family,
            check_kind,
            aspect_scope,
            support_basis,
        }
    }

    pub(crate) fn matches(
        self,
        touch: &UiGraphTouchDescriptor,
        node_record: Option<&UiGraphNodeRecord>,
        support_posture: UiObligationSupportSelectionPosture,
    ) -> bool {
        self.rule.matches(touch, node_record, support_posture)
    }

    pub(crate) fn family(self) -> UiObligationFamily {
        self.family
    }

    pub(crate) fn check_kind(self) -> UiObligationCheckKind {
        self.check_kind
    }

    pub(crate) fn aspect_scope(self) -> &'static [UiGraphTouchRuntimeLane] {
        self.aspect_scope
    }

    pub(crate) fn support_basis(self) -> UiObligationSupportBasis {
        self.support_basis
    }

    pub(crate) fn selection_reasons(
        self,
        touch: &UiGraphTouchDescriptor,
        support_posture: UiObligationSupportSelectionPosture,
    ) -> Vec<UiObligationSelectionReason> {
        let mut reasons = vec![
            UiObligationSelectionReason::TouchTargetClass(touch.target().class()),
            UiObligationSelectionReason::TouchOriginClass(touch.origin().class()),
            UiObligationSelectionReason::WorldProfile(UiObligationWorldProfileClass::from_profile(
                touch.world().world_profile(),
            )),
            UiObligationSelectionReason::SupportPosture(support_posture),
        ];
        reasons.push(match self.support_basis {
            UiObligationSupportBasis::TouchMeaning => UiObligationSelectionReason::SupportRow(
                crate::declaration::UiDeclarationSupportRowSchemaKind::TouchMeaning,
            ),
            UiObligationSupportBasis::QueryBinding => UiObligationSelectionReason::SupportRow(
                crate::declaration::UiDeclarationSupportRowSchemaKind::QueryBinding,
            ),
            UiObligationSupportBasis::ServiceUsage => UiObligationSelectionReason::SupportRow(
                crate::declaration::UiDeclarationSupportRowSchemaKind::ServiceUsage,
            ),
            UiObligationSupportBasis::MeasurementPolicy => UiObligationSelectionReason::SupportRow(
                crate::declaration::UiDeclarationSupportRowSchemaKind::MeasurementPolicy,
            ),
            UiObligationSupportBasis::HostCapability => UiObligationSelectionReason::SupportRow(
                crate::declaration::UiDeclarationSupportRowSchemaKind::HostCapability,
            ),
        });
        reasons.extend(self.rule.extra_reasons(touch));
        reasons
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UiObligationSelectionRule {
    Lane(UiGraphTouchRuntimeLane),
    TargetClass(UiGraphTouchTargetClass),
    OriginAndLane {
        origin: UiGraphTouchOriginClass,
        lane: UiGraphTouchRuntimeLane,
    },
    QueryBindingRequirement,
    ServiceUsageRequirement,
    DiagnosticSurface,
}

impl UiObligationSelectionRule {
    fn matches(
        self,
        touch: &UiGraphTouchDescriptor,
        node_record: Option<&UiGraphNodeRecord>,
        support_posture: UiObligationSupportSelectionPosture,
    ) -> bool {
        match self {
            Self::Lane(lane) => has_lane(touch, lane),
            Self::TargetClass(target_class) => touch.target().class() == target_class,
            Self::OriginAndLane { origin, lane } => {
                touch.origin().class() == origin && has_lane(touch, lane)
            }
            Self::QueryBindingRequirement => {
                matches!(
                    touch.origin().class(),
                    UiGraphTouchOriginClass::QueryBindingChange
                        | UiGraphTouchOriginClass::QueryFactChange
                ) && has_lane(touch, UiGraphTouchRuntimeLane::QueryBinding)
                    && node_record
                        .map(|record| record.attachment_posture().query_binding_attached())
                        .unwrap_or(false)
            }
            Self::ServiceUsageRequirement => {
                has_lane(touch, UiGraphTouchRuntimeLane::Structural)
                    && node_record
                        .map(|record| record.attachment_posture().service_usage_attached())
                        .unwrap_or(false)
            }
            Self::DiagnosticSurface => {
                touch.origin().class() == UiGraphTouchOriginClass::DiagnosticOnly
                    || has_lane(touch, UiGraphTouchRuntimeLane::Diagnostic)
                    || matches!(
                        support_posture,
                        UiObligationSupportSelectionPosture::DiagnosticOnly
                    )
            }
        }
    }

    fn extra_reasons(self, touch: &UiGraphTouchDescriptor) -> Vec<UiObligationSelectionReason> {
        match self {
            Self::Lane(lane) | Self::OriginAndLane { lane, .. } => reasons_for_lane(touch, lane),
            Self::QueryBindingRequirement => {
                let mut reasons = reasons_for_lane(touch, UiGraphTouchRuntimeLane::QueryBinding);
                reasons.push(UiObligationSelectionReason::GraphQueryBindingAttachment);
                reasons
            }
            Self::ServiceUsageRequirement => {
                reasons_for_lane(touch, UiGraphTouchRuntimeLane::Structural)
            }
            Self::DiagnosticSurface => reasons_for_lane(touch, UiGraphTouchRuntimeLane::Diagnostic),
            Self::TargetClass(_) => Vec::new(),
        }
    }
}

fn has_lane(touch: &UiGraphTouchDescriptor, lane: UiGraphTouchRuntimeLane) -> bool {
    touch.aspects().iter().any(|fact| fact.lane() == lane)
}

fn reasons_for_lane(
    touch: &UiGraphTouchDescriptor,
    lane: UiGraphTouchRuntimeLane,
) -> Vec<UiObligationSelectionReason> {
    touch
        .aspects()
        .iter()
        .filter(|fact| fact.lane() == lane)
        .flat_map(|fact| reasons_for_fact(*fact))
        .collect()
}

fn reasons_for_fact(fact: UiGraphTouchAspectFact) -> [UiObligationSelectionReason; 2] {
    [
        UiObligationSelectionReason::TouchRuntimeLane(fact.lane()),
        UiObligationSelectionReason::TouchAspectPosture(fact.posture()),
    ]
}

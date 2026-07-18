use crate::capability::MosaicSizingContractId;
use crate::declaration::{
    stable_text_digest, UiDeclarationOrderingGuarantee, UiDeclarationPlanningOperatorKind,
    UiDeclarationRepetitionPosture, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
};
use crate::evidence::{
    UiAllocationNeighborhoodClass, UiAllocationNeighborhoodMembershipRule,
    UiLayoutOperatorContractIdentity, UiLayoutOperatorFamily, UiLayoutOperatorPlanningSemantics,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorContainmentKind {
    RootPage,
    PageSet,
    Region,
    Mosaic,
    LocalComposition,
    Control,
    DiagnosticSurface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLayoutOperatorSlotParticipationKind {
    None,
    DeclaredParticipant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiLayoutOperatorPlanningContract {
    identity: UiLayoutOperatorContractIdentity,
    operator_kind: UiDeclarationPlanningOperatorKind,
    operator_family: UiLayoutOperatorFamily,
    containment_kind: UiLayoutOperatorContainmentKind,
    mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    slot_participation_kind: UiLayoutOperatorSlotParticipationKind,
    ordering_guarantee: UiDeclarationOrderingGuarantee,
    repetition_posture: UiDeclarationRepetitionPosture,
    neighborhood_class: UiAllocationNeighborhoodClass,
    membership_rule: UiAllocationNeighborhoodMembershipRule,
    measurement_mode: Option<UiDeclaredMeasurementMode>,
    constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    evidence_requirements: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
    semantics: UiLayoutOperatorPlanningSemantics,
}

pub(crate) struct UiLayoutOperatorPlanningContractInput {
    pub(crate) operator_kind: UiDeclarationPlanningOperatorKind,
    pub(crate) operator_family: UiLayoutOperatorFamily,
    pub(crate) containment_kind: UiLayoutOperatorContainmentKind,
    pub(crate) mosaic_sizing_contract_id: Option<MosaicSizingContractId>,
    pub(crate) slot_participation_kind: UiLayoutOperatorSlotParticipationKind,
    pub(crate) ordering_guarantee: UiDeclarationOrderingGuarantee,
    pub(crate) repetition_posture: UiDeclarationRepetitionPosture,
    pub(crate) neighborhood_class: UiAllocationNeighborhoodClass,
    pub(crate) membership_rule: UiAllocationNeighborhoodMembershipRule,
    pub(crate) measurement_mode: Option<UiDeclaredMeasurementMode>,
    pub(crate) constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    pub(crate) basis_source: Option<UiDeclaredMeasurementBasisSource>,
    pub(crate) ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    pub(crate) evidence_requirements: Vec<UiDeclaredMeasurementEvidenceRequirement>,
}

impl UiLayoutOperatorPlanningContract {
    pub(crate) fn new(input: UiLayoutOperatorPlanningContractInput) -> Self {
        let UiLayoutOperatorPlanningContractInput {
            operator_kind,
            operator_family,
            containment_kind,
            mosaic_sizing_contract_id,
            slot_participation_kind,
            ordering_guarantee,
            repetition_posture,
            neighborhood_class,
            membership_rule,
            measurement_mode,
            constraint_modifier,
            basis_source,
            ownership_posture,
            mut evidence_requirements,
        } = input;
        evidence_requirements.sort_unstable_by_key(evidence_requirement_rank);
        let semantics = UiLayoutOperatorPlanningSemantics::for_operator_kind(
            operator_kind,
            measurement_mode,
            basis_source,
            ownership_posture,
        );
        let identity = UiLayoutOperatorContractIdentity::new(
            evidence_requirements.iter().fold(
                stable_text_digest("worth-ui.layout-operator-planning-contract")
                    ^ operator_kind_digest(operator_kind).rotate_left(7)
                    ^ operator_family_digest(operator_family).rotate_left(11)
                    ^ containment_kind_digest(containment_kind).rotate_left(13)
                    ^ mosaic_sizing_contract_id_digest(mosaic_sizing_contract_id.as_ref())
                        .rotate_left(15)
                    ^ slot_participation_kind_digest(slot_participation_kind).rotate_left(17)
                    ^ ordering_guarantee_digest(ordering_guarantee).rotate_left(19)
                    ^ repetition_posture_digest(repetition_posture).rotate_left(21)
                    ^ (neighborhood_class as u64).rotate_left(23)
                    ^ (membership_rule as u64).rotate_left(27)
                    ^ measurement_mode_digest(measurement_mode).rotate_left(29)
                    ^ constraint_modifier_digest(constraint_modifier).rotate_left(31)
                    ^ basis_source_digest(basis_source).rotate_left(37)
                    ^ ownership_posture_digest(ownership_posture).rotate_left(41)
                    ^ semantics.identity_digest().rotate_left(43),
                |digest, requirement| {
                    digest.rotate_left(11)
                        ^ evidence_requirement_digest(*requirement).rotate_left(47)
                },
            ),
        );

        Self {
            identity,
            operator_kind,
            operator_family,
            containment_kind,
            mosaic_sizing_contract_id,
            slot_participation_kind,
            ordering_guarantee,
            repetition_posture,
            neighborhood_class,
            membership_rule,
            measurement_mode,
            constraint_modifier,
            basis_source,
            ownership_posture,
            evidence_requirements: evidence_requirements.into_boxed_slice(),
            semantics,
        }
    }

    pub fn identity(&self) -> UiLayoutOperatorContractIdentity {
        self.identity
    }

    pub fn operator_kind(&self) -> UiDeclarationPlanningOperatorKind {
        self.operator_kind
    }

    pub fn operator_family(&self) -> UiLayoutOperatorFamily {
        self.operator_family
    }

    pub fn containment_kind(&self) -> UiLayoutOperatorContainmentKind {
        self.containment_kind
    }

    pub fn mosaic_sizing_contract_id(&self) -> Option<&MosaicSizingContractId> {
        self.mosaic_sizing_contract_id.as_ref()
    }

    pub fn slot_participation_kind(&self) -> UiLayoutOperatorSlotParticipationKind {
        self.slot_participation_kind
    }

    pub fn ordering_guarantee(&self) -> UiDeclarationOrderingGuarantee {
        self.ordering_guarantee
    }

    pub fn repetition_posture(&self) -> UiDeclarationRepetitionPosture {
        self.repetition_posture
    }

    pub fn neighborhood_class(&self) -> UiAllocationNeighborhoodClass {
        self.neighborhood_class
    }

    pub fn membership_rule(&self) -> UiAllocationNeighborhoodMembershipRule {
        self.membership_rule
    }

    pub fn measurement_mode(&self) -> Option<UiDeclaredMeasurementMode> {
        self.measurement_mode
    }

    pub fn constraint_modifier(&self) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.constraint_modifier
    }

    pub fn basis_source(&self) -> Option<UiDeclaredMeasurementBasisSource> {
        self.basis_source
    }

    pub fn ownership_posture(&self) -> Option<UiDeclaredMeasurementOwnershipPosture> {
        self.ownership_posture
    }

    pub fn evidence_requirements(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.evidence_requirements
    }

    pub fn semantics(&self) -> &UiLayoutOperatorPlanningSemantics {
        &self.semantics
    }
}

fn operator_kind_digest(operator_kind: UiDeclarationPlanningOperatorKind) -> u64 {
    match operator_kind {
        UiDeclarationPlanningOperatorKind::PageRoot => {
            stable_text_digest("worth-ui.operator-kind.page-root")
        }
        UiDeclarationPlanningOperatorKind::PageSet => {
            stable_text_digest("worth-ui.operator-kind.page-set")
        }
        UiDeclarationPlanningOperatorKind::Region => {
            stable_text_digest("worth-ui.operator-kind.region")
        }
        UiDeclarationPlanningOperatorKind::Mosaic => {
            stable_text_digest("worth-ui.operator-kind.mosaic")
        }
        UiDeclarationPlanningOperatorKind::LocalComposition => {
            stable_text_digest("worth-ui.operator-kind.local-composition")
        }
        UiDeclarationPlanningOperatorKind::Control => {
            stable_text_digest("worth-ui.operator-kind.control")
        }
        UiDeclarationPlanningOperatorKind::DiagnosticSurface => {
            stable_text_digest("worth-ui.operator-kind.diagnostic-surface")
        }
        UiDeclarationPlanningOperatorKind::Stack => {
            stable_text_digest("worth-ui.operator-kind.stack")
        }
        UiDeclarationPlanningOperatorKind::Row => stable_text_digest("worth-ui.operator-kind.row"),
        UiDeclarationPlanningOperatorKind::Grid => {
            stable_text_digest("worth-ui.operator-kind.grid")
        }
        UiDeclarationPlanningOperatorKind::Split => {
            stable_text_digest("worth-ui.operator-kind.split")
        }
        UiDeclarationPlanningOperatorKind::Overlay => {
            stable_text_digest("worth-ui.operator-kind.overlay")
        }
        UiDeclarationPlanningOperatorKind::Scroll => {
            stable_text_digest("worth-ui.operator-kind.scroll")
        }
        UiDeclarationPlanningOperatorKind::PortalAnchor => {
            stable_text_digest("worth-ui.operator-kind.portal-anchor")
        }
    }
}

fn operator_family_digest(operator_family: UiLayoutOperatorFamily) -> u64 {
    match operator_family {
        UiLayoutOperatorFamily::Page => stable_text_digest("worth-ui.operator-family.page"),
        UiLayoutOperatorFamily::PageSet => stable_text_digest("worth-ui.operator-family.page-set"),
        UiLayoutOperatorFamily::Region => stable_text_digest("worth-ui.operator-family.region"),
        UiLayoutOperatorFamily::Mosaic => stable_text_digest("worth-ui.operator-family.mosaic"),
        UiLayoutOperatorFamily::LocalComposition => {
            stable_text_digest("worth-ui.operator-family.local-composition")
        }
        UiLayoutOperatorFamily::Control => stable_text_digest("worth-ui.operator-family.control"),
        UiLayoutOperatorFamily::DiagnosticSurface => {
            stable_text_digest("worth-ui.operator-family.diagnostic-surface")
        }
    }
}

fn containment_kind_digest(containment_kind: UiLayoutOperatorContainmentKind) -> u64 {
    match containment_kind {
        UiLayoutOperatorContainmentKind::RootPage => {
            stable_text_digest("worth-ui.operator-contract.containment.root-page")
        }
        UiLayoutOperatorContainmentKind::PageSet => {
            stable_text_digest("worth-ui.operator-contract.containment.page-set")
        }
        UiLayoutOperatorContainmentKind::Region => {
            stable_text_digest("worth-ui.operator-contract.containment.region")
        }
        UiLayoutOperatorContainmentKind::Mosaic => {
            stable_text_digest("worth-ui.operator-contract.containment.mosaic")
        }
        UiLayoutOperatorContainmentKind::LocalComposition => {
            stable_text_digest("worth-ui.operator-contract.containment.local-composition")
        }
        UiLayoutOperatorContainmentKind::Control => {
            stable_text_digest("worth-ui.operator-contract.containment.control")
        }
        UiLayoutOperatorContainmentKind::DiagnosticSurface => {
            stable_text_digest("worth-ui.operator-contract.containment.diagnostic-surface")
        }
    }
}

fn mosaic_sizing_contract_id_digest(
    mosaic_sizing_contract_id: Option<&MosaicSizingContractId>,
) -> u64 {
    mosaic_sizing_contract_id
        .map(|id| stable_text_digest(id.as_str()))
        .unwrap_or_else(|| stable_text_digest("worth-ui.operator-contract.mosaic-sizing.none"))
}

fn slot_participation_kind_digest(
    slot_participation_kind: UiLayoutOperatorSlotParticipationKind,
) -> u64 {
    match slot_participation_kind {
        UiLayoutOperatorSlotParticipationKind::None => {
            stable_text_digest("worth-ui.operator-contract.slot.none")
        }
        UiLayoutOperatorSlotParticipationKind::DeclaredParticipant => {
            stable_text_digest("worth-ui.operator-contract.slot.declared-participant")
        }
    }
}

fn ordering_guarantee_digest(ordering_guarantee: UiDeclarationOrderingGuarantee) -> u64 {
    match ordering_guarantee {
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed => {
            stable_text_digest("worth-ui.operator-contract.ordering.not-semantically-claimed")
        }
    }
}

fn repetition_posture_digest(repetition_posture: UiDeclarationRepetitionPosture) -> u64 {
    match repetition_posture {
        UiDeclarationRepetitionPosture::NotAdmitted => {
            stable_text_digest("worth-ui.operator-contract.repetition.not-admitted")
        }
    }
}

fn evidence_requirement_rank(requirement: &UiDeclaredMeasurementEvidenceRequirement) -> u8 {
    match requirement {
        UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => 0,
        UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => 1,
        UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => 2,
    }
}

fn measurement_mode_digest(mode: Option<UiDeclaredMeasurementMode>) -> u64 {
    match mode {
        Some(UiDeclaredMeasurementMode::HugHeight) => {
            stable_text_digest("worth-ui.operator-contract.mode.hug-height")
        }
        None => stable_text_digest("worth-ui.operator-contract.mode.none"),
    }
}

fn constraint_modifier_digest(modifier: Option<UiDeclaredMeasurementConstraintModifier>) -> u64 {
    match modifier {
        Some(UiDeclaredMeasurementConstraintModifier::Bounded) => {
            stable_text_digest("worth-ui.operator-contract.constraint.bounded")
        }
        None => stable_text_digest("worth-ui.operator-contract.constraint.none"),
    }
}

fn basis_source_digest(source: Option<UiDeclaredMeasurementBasisSource>) -> u64 {
    match source {
        Some(UiDeclaredMeasurementBasisSource::ViewportExtent) => {
            stable_text_digest("worth-ui.operator-contract.basis.viewport-extent")
        }
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport) => {
            stable_text_digest("worth-ui.operator-contract.basis.scroll-viewport")
        }
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor) => {
            stable_text_digest("worth-ui.operator-contract.basis.portal-anchor")
        }
        None => stable_text_digest("worth-ui.operator-contract.basis.none"),
    }
}

fn ownership_posture_digest(posture: Option<UiDeclaredMeasurementOwnershipPosture>) -> u64 {
    match posture {
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis) => {
            stable_text_digest("worth-ui.operator-contract.ownership.scroll-container-basis")
        }
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired) => {
            stable_text_digest("worth-ui.operator-contract.ownership.portal-anchor-basis-required")
        }
        None => stable_text_digest("worth-ui.operator-contract.ownership.none"),
    }
}

fn evidence_requirement_digest(requirement: UiDeclaredMeasurementEvidenceRequirement) -> u64 {
    match requirement {
        UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => {
            stable_text_digest("worth-ui.operator-contract.evidence.host-font-metrics")
        }
        UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
            stable_text_digest("worth-ui.operator-contract.evidence.scroll-content-extent")
        }
        UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => {
            stable_text_digest("worth-ui.operator-contract.evidence.portal-anchor-metrics")
        }
    }
}

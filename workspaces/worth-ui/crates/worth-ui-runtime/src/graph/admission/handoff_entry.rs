use crate::declaration::UiDeclarationGraphHandoff;
use crate::graph::{
    UiGraphAttachmentPosture, UiGraphContainmentClaim, UiGraphCoreIndexContributionSeed,
    UiGraphMountedReceiptAuthoritySeed, UiGraphNodeInstantiationEntry,
    UiGraphParentResolutionClaim, UiGraphParticipationSeed, UiGraphTopologySeed,
    UiRepeatedInstanceBasis,
};

pub(super) fn construct_instantiation_entry(
    handoff: &UiDeclarationGraphHandoff,
    repeated_instance_basis: UiRepeatedInstanceBasis,
) -> UiGraphNodeInstantiationEntry {
    UiGraphNodeInstantiationEntry::new(
        handoff.identity().clone(),
        handoff.authored_provenance_digest(),
        handoff
            .measurement_policy()
            .admitted()
            .and_then(|policy| policy.constraint_modifier()),
        handoff.aspect_contract().clone(),
        repeated_instance_basis,
        UiGraphTopologySeed::new(
            handoff.structural_digest(),
            handoff.role(),
            handoff.operator_kind(),
            UiGraphContainmentClaim::from_declaration_intent(
                handoff.containment_intent(),
                handoff.mosaic_sizing_contract_id().cloned(),
            ),
            parent_resolution_claim_for_handoff(handoff),
            handoff.slot_participation_intent().clone(),
            handoff.ordering_guarantee(),
            handoff.repetition_posture(),
        ),
        UiGraphParticipationSeed::from_attachment_and_role(
            handoff.query_binding().admitted().is_some(),
            handoff.service_usage().admitted().is_some(),
            matches!(
                handoff.role(),
                crate::declaration::UiDeclarationStructuralRole::DiagnosticSurface
            ),
        ),
        UiGraphAttachmentPosture::new(
            handoff.query_binding().admitted().is_some(),
            handoff.service_usage().admitted().is_some(),
        ),
        UiGraphMountedReceiptAuthoritySeed::reserved(),
        UiGraphCoreIndexContributionSeed::authoritative(),
    )
}

fn parent_resolution_claim_for_handoff(
    handoff: &UiDeclarationGraphHandoff,
) -> UiGraphParentResolutionClaim {
    if handoff.containment_intent().is_root() {
        UiGraphParentResolutionClaim::RootPage
    } else {
        UiGraphParentResolutionClaim::ContainedByRootPage
    }
}

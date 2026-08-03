use crate::declaration::{stable_text_digest, UiDeclarationPlanningOperatorKind};
use crate::evidence::UiLayoutOperatorFamily;

use super::super::UiLayoutOperatorContainmentKind;

pub(super) fn operator_kind_digest(operator_kind: UiDeclarationPlanningOperatorKind) -> u64 {
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

pub(super) fn operator_family_digest(operator_family: UiLayoutOperatorFamily) -> u64 {
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

pub(super) fn containment_kind_digest(containment_kind: UiLayoutOperatorContainmentKind) -> u64 {
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

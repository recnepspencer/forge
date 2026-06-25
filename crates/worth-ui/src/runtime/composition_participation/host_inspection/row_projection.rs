use super::receipt::{
    WorthUiAccessibilityHostInspectionRow, WorthUiAccessibilityHostInspectionRowFeature,
};
use crate::runtime::composition_participation::projection::{
    WorthUiAccessibilityAssociationKind, WorthUiCompositionParticipationReceipt,
    WorthUiFocusParticipationPosture,
};

pub(super) fn inspection_rows_for_participation(
    participation: &WorthUiCompositionParticipationReceipt,
) -> Vec<WorthUiAccessibilityHostInspectionRow> {
    let mut rows = node_rows(participation);
    rows.extend(relationship_rows(participation));
    rows
}

fn node_rows(
    participation: &WorthUiCompositionParticipationReceipt,
) -> Vec<WorthUiAccessibilityHostInspectionRow> {
    let mut rows = Vec::new();
    for node in participation.accessibility_nodes() {
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            node.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::Role,
            Some(node.role().to_owned()),
        ));
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            node.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::Name,
            node.name().map(str::to_owned),
        ));
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            node.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::Description,
            Some(node.description_node_ids().join(",")),
        ));
    }
    for focus in participation.focus_nodes() {
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            focus.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::Enabled,
            Some((focus.posture() != WorthUiFocusParticipationPosture::Disabled).to_string()),
        ));
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            focus.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::Focusable,
            Some((focus.posture() == WorthUiFocusParticipationPosture::Focusable).to_string()),
        ));
        rows.push(WorthUiAccessibilityHostInspectionRow::projected(
            focus.node_id(),
            WorthUiAccessibilityHostInspectionRowFeature::TabOrder,
            Some(focus.graph_order().to_string()),
        ));
    }
    rows
}

fn relationship_rows(
    participation: &WorthUiCompositionParticipationReceipt,
) -> Vec<WorthUiAccessibilityHostInspectionRow> {
    participation
        .relationships()
        .iter()
        .map(|relationship| {
            let feature = match relationship.kind() {
                WorthUiAccessibilityAssociationKind::Label => {
                    WorthUiAccessibilityHostInspectionRowFeature::LabelFor
                }
                WorthUiAccessibilityAssociationKind::Description => {
                    WorthUiAccessibilityHostInspectionRowFeature::DescribedBy
                }
                WorthUiAccessibilityAssociationKind::Error => {
                    WorthUiAccessibilityHostInspectionRowFeature::ErrorMessage
                }
            };
            WorthUiAccessibilityHostInspectionRow::projected(
                relationship.target_node_id(),
                feature,
                relationship.source_resolved_text().map(str::to_owned),
            )
        })
        .collect()
}

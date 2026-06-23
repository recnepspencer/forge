use forge_query::facade::consumer_kit::{
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryGraphObligationLocalCeremonyAudit,
};

const SELECTION_SUBSTRATE_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/mod.rs";
const QUERY_CONSUMER_KIT_LANE_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/query_consumer_kit_lane/mod.rs";
const TOPOLOGY_TOUCHED_BASIS_LANE_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/query_consumer_kit_lane/topology_touched_basis_lane.rs";
const PRIMITIVE_CONSTRUCTION_LANE_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/query_consumer_kit_lane/primitive_construction_lane.rs";
const SPATIAL_DESCRIPTOR_LANE_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/query_consumer_kit_lane/spatial_descriptor_lane.rs";
const LOCAL_SELECTOR_DENIAL_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/local_selector_denial.rs";
const SELECTION_REQUEST_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/selection_request.rs";
const SELECTED_OBLIGATIONS_SOURCE_LABEL: &str =
    "crates/worth-kernel/src/query_obligation_selection/selection_substrate/selected_obligations.rs";

const SELECTION_SUBSTRATE_RS: &str = include_str!("../mod.rs");
const QUERY_CONSUMER_KIT_LANE_RS: &str = include_str!("mod.rs");
const TOPOLOGY_TOUCHED_BASIS_LANE_RS: &str = include_str!("topology_touched_basis_lane.rs");
const PRIMITIVE_CONSTRUCTION_LANE_RS: &str = include_str!("primitive_construction_lane.rs");
const SPATIAL_DESCRIPTOR_LANE_RS: &str = include_str!("spatial_descriptor_lane.rs");
const LOCAL_SELECTOR_DENIAL_RS: &str = include_str!("../local_selector_denial.rs");
const SELECTION_REQUEST_RS: &str = include_str!("../selection_request.rs");
const SELECTED_OBLIGATIONS_RS: &str = include_str!("../selected_obligations.rs");

pub fn selection_substrate_local_ceremony_audit() -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &ForgeQueryBoundaryAuditSourceSet::new("worth-kernel.query-obligation-selection")
            .source_file(
                SELECTION_SUBSTRATE_SOURCE_LABEL,
                SELECTION_SUBSTRATE_SOURCE_LABEL,
                SELECTION_SUBSTRATE_RS,
            )
            .source_file(
                QUERY_CONSUMER_KIT_LANE_SOURCE_LABEL,
                QUERY_CONSUMER_KIT_LANE_SOURCE_LABEL,
                QUERY_CONSUMER_KIT_LANE_RS,
            )
            .source_file(
                TOPOLOGY_TOUCHED_BASIS_LANE_SOURCE_LABEL,
                TOPOLOGY_TOUCHED_BASIS_LANE_SOURCE_LABEL,
                TOPOLOGY_TOUCHED_BASIS_LANE_RS,
            )
            .source_file(
                PRIMITIVE_CONSTRUCTION_LANE_SOURCE_LABEL,
                PRIMITIVE_CONSTRUCTION_LANE_SOURCE_LABEL,
                PRIMITIVE_CONSTRUCTION_LANE_RS,
            )
            .source_file(
                SPATIAL_DESCRIPTOR_LANE_SOURCE_LABEL,
                SPATIAL_DESCRIPTOR_LANE_SOURCE_LABEL,
                SPATIAL_DESCRIPTOR_LANE_RS,
            )
            .source_file(
                LOCAL_SELECTOR_DENIAL_SOURCE_LABEL,
                LOCAL_SELECTOR_DENIAL_SOURCE_LABEL,
                LOCAL_SELECTOR_DENIAL_RS,
            )
            .source_file(
                SELECTION_REQUEST_SOURCE_LABEL,
                SELECTION_REQUEST_SOURCE_LABEL,
                SELECTION_REQUEST_RS,
            )
            .source_file(
                SELECTED_OBLIGATIONS_SOURCE_LABEL,
                SELECTED_OBLIGATIONS_SOURCE_LABEL,
                SELECTED_OBLIGATIONS_RS,
            ),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        selection_substrate_local_ceremony_audit, LOCAL_SELECTOR_DENIAL_SOURCE_LABEL,
        PRIMITIVE_CONSTRUCTION_LANE_SOURCE_LABEL, QUERY_CONSUMER_KIT_LANE_SOURCE_LABEL,
        SELECTED_OBLIGATIONS_SOURCE_LABEL, SELECTION_REQUEST_SOURCE_LABEL,
        SELECTION_SUBSTRATE_SOURCE_LABEL, SPATIAL_DESCRIPTOR_LANE_SOURCE_LABEL,
        TOPOLOGY_TOUCHED_BASIS_LANE_SOURCE_LABEL,
    };

    #[test]
    fn selection_substrate_audit_covers_spatial_descriptor_lane() {
        let audit = selection_substrate_local_ceremony_audit();
        let expected_labels = [
            SELECTION_SUBSTRATE_SOURCE_LABEL,
            QUERY_CONSUMER_KIT_LANE_SOURCE_LABEL,
            TOPOLOGY_TOUCHED_BASIS_LANE_SOURCE_LABEL,
            PRIMITIVE_CONSTRUCTION_LANE_SOURCE_LABEL,
            SPATIAL_DESCRIPTOR_LANE_SOURCE_LABEL,
            LOCAL_SELECTOR_DENIAL_SOURCE_LABEL,
            SELECTION_REQUEST_SOURCE_LABEL,
            SELECTED_OBLIGATIONS_SOURCE_LABEL,
        ];

        assert_eq!(audit.evaluated_source_count(), expected_labels.len());
        assert_eq!(audit.audited_source_labels(), expected_labels);
        assert!(audit.is_clean());
    }
}

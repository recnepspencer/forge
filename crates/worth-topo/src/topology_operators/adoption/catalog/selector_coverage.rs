use forge_query::facade::consumer_kit::ForgeQueryGraphObligationSelectorCoverageDeclaration;

use super::topology_operator_graph_obligation_catalog;

pub fn topology_operator_graph_obligation_selector_coverage(
) -> ForgeQueryGraphObligationSelectorCoverageDeclaration {
    ForgeQueryGraphObligationSelectorCoverageDeclaration::required(
        topology_operator_graph_obligation_catalog()
            .covered_rows()
            .filter_map(|row| {
                row.touch_selector()
                    .cloned()
                    .map(|selector| (row.operator_family(), selector))
            }),
    )
}

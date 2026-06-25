use crate::derived_topology::materialized_graph::{
    MaterializationFallbackClass, MaterializationReport,
};

pub(super) fn materialization_report_digest(report: &MaterializationReport) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-materialization-report:v1".to_string(),
        format!("entity-count:{}", report.breadth.entity_count),
        format!("relation-count:{}", report.breadth.relation_count),
        format!(
            "topology-entity-count:{}",
            report.breadth.topology_entity_count
        ),
        format!(
            "topology-relation-count:{}",
            report.breadth.topology_relation_count
        ),
        format!("whole-view:{}", report.whole_view_materialization),
        format!(
            "fallback:{}",
            report
                .fallback_class
                .map(materialization_fallback_class_label)
                .unwrap_or("not-applicable")
        ),
    ])
}

pub(super) fn materialization_report_is_whole_view_fallback(
    report: &MaterializationReport,
) -> bool {
    report.whole_view_materialization
        || report.fallback_class == Some(MaterializationFallbackClass::WholeViewRebuild)
}

fn materialization_fallback_class_label(fallback: MaterializationFallbackClass) -> &'static str {
    match fallback {
        MaterializationFallbackClass::WholeViewRebuild => "whole_view_rebuild",
        MaterializationFallbackClass::CompleteTopologyBootstrap => "complete_topology_bootstrap",
    }
}

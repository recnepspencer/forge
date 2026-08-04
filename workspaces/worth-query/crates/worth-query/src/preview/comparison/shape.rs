#[cfg(test)]
use crate::basis::ExecutionPreflightBundle;
#[cfg(test)]
use crate::collection::MaterializationBreadthClass;
#[cfg(test)]
use crate::identity::CollectionPlanDigest;
#[cfg(test)]
use crate::preview::workflow_context_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg(test)]
pub(in crate::preview) struct PreviewComparisonShapeContract {
    pub(super) collection_digest: Option<CollectionPlanDigest>,
    pub(super) result_family: String,
    pub(super) ordering_digest: String,
    pub(super) materialization_boundary_digest: String,
    pub(in crate::preview) shape_check_width: usize,
}

#[cfg(test)]
impl PreviewComparisonShapeContract {
    #[cfg(test)]
    pub(in crate::preview) fn from_preflight(preflight: &ExecutionPreflightBundle) -> Self {
        let collection = preflight.plan().collection();
        let ordering_digest = workflow_context_identity::compose_preview_comparison_ordering_digest(
            &collection
                .map(|collection| collection.ordering_basis().digest_parts())
                .unwrap_or_else(|| vec!["detail_ordering:root_entity_identity".to_string()]),
        );
        let materialization_boundary_digest =
            workflow_context_identity::compose_preview_comparison_materialization_boundary_digest(
                &collection
                    .map(|collection| {
                        let mut parts = vec![
                            collection.window_policy().digest_part(),
                            collection.cursor_contract().digest_part(),
                        ];
                        parts.extend(collection.traversal_bound().digest_parts());
                        parts.extend(collection.post_read_shaping().digest_parts());
                        parts
                    })
                    .unwrap_or_else(|| {
                        vec![
                            "window_policy:detail_single_read".to_string(),
                            "cursor_contract:not_applicable".to_string(),
                            "materialization_breadth:scalar_only".to_string(),
                            "detail_result_family:detail".to_string(),
                        ]
                    }),
            );
        let shape_check_width = collection
            .map(|collection| {
                collection.ordering_basis().entries().len()
                    + collection.traversal_bound().edge_classes().len()
                    + usize::from(matches!(
                        collection.traversal_bound().materialization_breadth(),
                        MaterializationBreadthClass::RootPlusTraversal
                    ))
                    + preflight.plan().result_shape().binding_count()
            })
            .unwrap_or_else(|| preflight.plan().result_shape().binding_count().max(1));
        let result_family = collection
            .map(|collection| collection.planning_context().result_family().digest_label())
            .unwrap_or("detail")
            .to_string();

        Self {
            collection_digest: collection.map(|collection| collection.digest().clone()),
            result_family,
            ordering_digest,
            materialization_boundary_digest,
            shape_check_width,
        }
    }
}

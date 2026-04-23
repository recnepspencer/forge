use forge_query::facade::QuerySubscriptionAdmissionDimensions;

fn main() {
    let _fabricated = QuerySubscriptionAdmissionDimensions {
        authorized_projection_width: 1,
        ordering_width: 1,
        grouping_width: 0,
        relation_scope_width: 0,
        view_shape_metadata_width: 0,
    };
}

use std::num::NonZeroUsize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmissionDimensions {
    pub(super) authorized_projection_width: usize,
    pub(super) ordering_width: usize,
    pub(super) grouping_width: usize,
    pub(super) relation_scope_width: usize,
    pub(super) view_shape_metadata_width: usize,
}

impl QuerySubscriptionAdmissionDimensions {
    pub fn detail_exact(authorized_projection_width: NonZeroUsize) -> Self {
        Self::new(authorized_projection_width.get(), 0, 0, 0, 0)
    }

    pub fn collection_membership(
        authorized_projection_width: NonZeroUsize,
        ordering_width: NonZeroUsize,
    ) -> Self {
        Self::new(
            authorized_projection_width.get(),
            ordering_width.get(),
            0,
            0,
            0,
        )
    }

    pub fn grouped_collection_membership(
        authorized_projection_width: NonZeroUsize,
        ordering_width: NonZeroUsize,
        grouping_width: NonZeroUsize,
        view_shape_metadata_width: NonZeroUsize,
    ) -> Self {
        Self::new(
            authorized_projection_width.get(),
            ordering_width.get(),
            grouping_width.get(),
            0,
            view_shape_metadata_width.get(),
        )
    }

    pub fn inspector_detail_exact(
        authorized_projection_width: NonZeroUsize,
        view_shape_metadata_width: NonZeroUsize,
    ) -> Self {
        Self::new(
            authorized_projection_width.get(),
            0,
            0,
            0,
            view_shape_metadata_width.get(),
        )
    }

    pub fn bounded_materialization(
        authorized_projection_width: NonZeroUsize,
        ordering_width: NonZeroUsize,
        relation_scope_width: NonZeroUsize,
    ) -> Self {
        Self::new(
            authorized_projection_width.get(),
            ordering_width.get(),
            0,
            relation_scope_width.get(),
            0,
        )
    }

    fn new(
        authorized_projection_width: usize,
        ordering_width: usize,
        grouping_width: usize,
        relation_scope_width: usize,
        view_shape_metadata_width: usize,
    ) -> Self {
        Self {
            authorized_projection_width,
            ordering_width,
            grouping_width,
            relation_scope_width,
            view_shape_metadata_width,
        }
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn ordering_width(&self) -> usize {
        self.ordering_width
    }

    pub fn grouping_width(&self) -> usize {
        self.grouping_width
    }

    pub fn relation_scope_width(&self) -> usize {
        self.relation_scope_width
    }

    pub fn view_shape_metadata_width(&self) -> usize {
        self.view_shape_metadata_width
    }
}

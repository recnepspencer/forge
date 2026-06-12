use crate::authoring::{AuthoredQuery, AuthoredResultShape, DetailFamily, DetailResultShapeFamily};
use crate::composition::TemplateFamily;

use super::descriptor::QueryTemplateDescriptor;

impl QueryTemplateDescriptor<DetailFamily, DetailResultShapeFamily> {
    pub fn observed_inspector_deferred_for_test(
        query: AuthoredQuery<DetailFamily>,
        result_shape: AuthoredResultShape<DetailResultShapeFamily>,
    ) -> Self {
        deferred_detail_template_descriptor(
            TemplateFamily::ObservedInspectorDetailTemplate,
            query,
            result_shape,
        )
    }

    pub fn focused_inspector_deferred_for_test(
        query: AuthoredQuery<DetailFamily>,
        result_shape: AuthoredResultShape<DetailResultShapeFamily>,
    ) -> Self {
        deferred_detail_template_descriptor(
            TemplateFamily::FocusedInspectorDetailTemplate,
            query,
            result_shape,
        )
    }
}

fn deferred_detail_template_descriptor(
    family: TemplateFamily,
    query: AuthoredQuery<DetailFamily>,
    result_shape: AuthoredResultShape<DetailResultShapeFamily>,
) -> QueryTemplateDescriptor<DetailFamily, DetailResultShapeFamily> {
    QueryTemplateDescriptor::with_family_for_test(family, query, result_shape)
}

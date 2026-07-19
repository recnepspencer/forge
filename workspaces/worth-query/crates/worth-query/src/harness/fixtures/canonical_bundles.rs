use crate::authoring::GuidedAuthoringPath;
use crate::facade::foundation::CanonicalQueryBundle;

pub fn runtime_detail_bundle() -> CanonicalQueryBundle {
    let request = GuidedAuthoringPath::pair_detail(
        super::authored_requests::runtime_detail_query(),
        super::authored_requests::runtime_detail_result_shape(),
    )
    .unwrap();
    crate::facade::foundation::canonicalize_request(request).unwrap()
}

pub fn runtime_bound_detail_bundle() -> CanonicalQueryBundle {
    let bindings = crate::facade::foundation::QueryBindingDescriptor::new().with_identity(
        crate::facade::foundation::IdentityBindingDescriptor::new(
            crate::facade::foundation::QueryBindingSlot::new("root").unwrap(),
            crate::facade::foundation::QueryBindingSubject::RootEntity,
        ),
    );
    let request = GuidedAuthoringPath::pair_detail_with_bindings(
        super::authored_requests::runtime_detail_query(),
        super::authored_requests::runtime_detail_result_shape(),
        bindings,
    )
    .unwrap();
    crate::facade::foundation::canonicalize_request(request).unwrap()
}

pub fn legal_detail_bundle() -> CanonicalQueryBundle {
    super::schema_view::legal_detail_bundle()
}

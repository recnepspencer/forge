use crate::{
    WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily, WorthUiProjectionRequirement,
    WorthUiProjectionShape,
};

#[derive(Debug, Eq, PartialEq)]
pub(super) struct ProjectionExpectation {
    declaration: String,
    view: String,
    shape: ProjectionShapeExpectation,
    selected_fields: Vec<String>,
    row_identity: Option<String>,
    native_family: ProjectionNativeExpectation,
    lifecycle: ProjectionLifecycleExpectation,
    requires_complete_result: Option<bool>,
    permits_continuation: Option<bool>,
}

pub(super) struct CollectionExpectation<'field> {
    selected_fields: &'field [&'field str],
    lifecycle: WorthUiProjectionLifecycle,
    policy: (bool, bool),
}

#[derive(Debug, Eq, PartialEq)]
enum ProjectionShapeExpectation {
    Scalar,
    Collection,
}

#[derive(Debug, Eq, PartialEq)]
enum ProjectionNativeExpectation {
    Text,
    Boolean,
}

#[derive(Debug, Eq, PartialEq)]
enum ProjectionLifecycleExpectation {
    Snapshot,
    Live,
}

impl ProjectionExpectation {
    pub(super) fn scalar(
        declaration: &str,
        view: &str,
        field: &str,
        lifecycle: WorthUiProjectionLifecycle,
    ) -> Self {
        Self {
            declaration: declaration.to_owned(),
            view: view.to_owned(),
            shape: ProjectionShapeExpectation::Scalar,
            selected_fields: vec![field.to_owned()],
            row_identity: None,
            native_family: ProjectionNativeExpectation::Text,
            lifecycle: lifecycle_expectation(lifecycle),
            requires_complete_result: None,
            permits_continuation: None,
        }
    }

    pub(super) fn collection(
        declaration: &str,
        view: &str,
        row_identity: &str,
        selection: CollectionExpectation<'_>,
    ) -> Self {
        let mut selected_fields = selection
            .selected_fields
            .iter()
            .map(|field| (*field).to_owned())
            .collect::<Vec<_>>();
        selected_fields.sort();
        Self {
            declaration: declaration.to_owned(),
            view: view.to_owned(),
            shape: ProjectionShapeExpectation::Collection,
            selected_fields,
            row_identity: Some(row_identity.to_owned()),
            native_family: ProjectionNativeExpectation::Text,
            lifecycle: lifecycle_expectation(selection.lifecycle),
            requires_complete_result: Some(selection.policy.0),
            permits_continuation: Some(selection.policy.1),
        }
    }

    pub(super) fn capture(requirement: &WorthUiProjectionRequirement) -> Self {
        let policy = requirement.collection_policy();
        Self {
            declaration: requirement.declaration_identity().to_owned(),
            view: requirement.view_identity().to_owned(),
            shape: match requirement.shape() {
                WorthUiProjectionShape::Scalar => ProjectionShapeExpectation::Scalar,
                WorthUiProjectionShape::Collection => ProjectionShapeExpectation::Collection,
            },
            selected_fields: requirement.selected_fields().map(str::to_owned).collect(),
            row_identity: requirement.row_identity_field().map(str::to_owned),
            native_family: match requirement.native_family() {
                WorthUiProjectionNativeFamily::Text => ProjectionNativeExpectation::Text,
                WorthUiProjectionNativeFamily::Boolean => ProjectionNativeExpectation::Boolean,
            },
            lifecycle: lifecycle_expectation(requirement.lifecycle()),
            requires_complete_result: policy.map(|value| value.requires_complete_result()),
            permits_continuation: policy.map(|value| value.permits_continuation()),
        }
    }
}

impl<'field> CollectionExpectation<'field> {
    pub(super) fn new(
        selected_fields: &'field [&'field str],
        lifecycle: WorthUiProjectionLifecycle,
        policy: (bool, bool),
    ) -> Self {
        Self {
            selected_fields,
            lifecycle,
            policy,
        }
    }
}

fn lifecycle_expectation(lifecycle: WorthUiProjectionLifecycle) -> ProjectionLifecycleExpectation {
    match lifecycle {
        WorthUiProjectionLifecycle::Snapshot => ProjectionLifecycleExpectation::Snapshot,
        WorthUiProjectionLifecycle::Live => ProjectionLifecycleExpectation::Live,
    }
}

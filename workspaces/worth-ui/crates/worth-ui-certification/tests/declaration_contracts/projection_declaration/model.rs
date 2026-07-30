use worth_ui_dsl::{
    WorthUiProjectionLifecycle, WorthUiProjectionNativeFamily, WorthUiProjectionRequirement,
    WorthUiProjectionShape,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct RequirementModel {
    declaration: String,
    view: String,
    shape: Shape,
    selected_fields: Vec<String>,
    row_identity: Option<String>,
    native_family: NativeFamily,
    lifecycle: Lifecycle,
    collection_policy: Option<CollectionPolicy>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum NativeFamily {
    Text,
    Boolean,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Lifecycle {
    Snapshot,
    Live,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Shape {
    Scalar,
    Collection,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CollectionPolicy {
    requires_complete: bool,
    permits_continuation: bool,
}

impl RequirementModel {
    pub(super) fn scalar(
        declaration: &str,
        view: &str,
        field: &str,
        native_family: NativeFamily,
        lifecycle: Lifecycle,
    ) -> Self {
        Self {
            declaration: declaration.to_owned(),
            view: view.to_owned(),
            shape: Shape::Scalar,
            selected_fields: vec![field.to_owned()],
            row_identity: None,
            native_family,
            lifecycle,
            collection_policy: None,
        }
    }

    pub(super) fn collection(
        declaration: &str,
        view: &str,
        row_identity: &str,
        selected_fields: &[&str],
        native_family: NativeFamily,
        lifecycle: Lifecycle,
        policy: (bool, bool),
    ) -> Self {
        let mut selected_fields = selected_fields
            .iter()
            .map(|field| (*field).to_owned())
            .collect::<Vec<_>>();
        selected_fields.sort();
        Self {
            declaration: declaration.to_owned(),
            view: view.to_owned(),
            shape: Shape::Collection,
            selected_fields,
            row_identity: Some(row_identity.to_owned()),
            native_family,
            lifecycle,
            collection_policy: Some(CollectionPolicy {
                requires_complete: policy.0,
                permits_continuation: policy.1,
            }),
        }
    }

    pub(super) fn capture(requirement: &WorthUiProjectionRequirement) -> Self {
        let policy = requirement.collection_policy();
        Self {
            declaration: requirement.declaration_identity().to_owned(),
            view: requirement.view_identity().to_owned(),
            shape: match requirement.shape() {
                WorthUiProjectionShape::Scalar => Shape::Scalar,
                WorthUiProjectionShape::Collection => Shape::Collection,
            },
            selected_fields: requirement.selected_fields().map(str::to_owned).collect(),
            row_identity: requirement.row_identity_field().map(str::to_owned),
            native_family: match requirement.native_family() {
                WorthUiProjectionNativeFamily::Text => NativeFamily::Text,
                WorthUiProjectionNativeFamily::Boolean => NativeFamily::Boolean,
            },
            lifecycle: match requirement.lifecycle() {
                WorthUiProjectionLifecycle::Snapshot => Lifecycle::Snapshot,
                WorthUiProjectionLifecycle::Live => Lifecycle::Live,
            },
            collection_policy: policy.map(|policy| CollectionPolicy {
                requires_complete: policy.requires_complete_result(),
                permits_continuation: policy.permits_continuation(),
            }),
        }
    }
}

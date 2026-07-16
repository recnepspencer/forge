use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AspectValue,
    CanonicalAspectValueIdentityBasis, StructAspectValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ConsumedNativeValue {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

impl ConsumedNativeValue {
    pub(crate) fn scalar(value: AspectValue) -> Self {
        Self::Scalar(value)
    }

    pub(crate) fn struct_value(value: StructAspectValue) -> Self {
        Self::Struct(value)
    }

    pub(crate) fn from_snapshot_read_value(
        value: &worth_runtime_bridge::facade::SnapshotReadValue,
    ) -> Self {
        match value {
            worth_runtime_bridge::facade::SnapshotReadValue::Scalar(value) => {
                Self::scalar(value.clone())
            }
            worth_runtime_bridge::facade::SnapshotReadValue::Struct(value) => {
                Self::struct_value(value.clone())
            }
        }
    }

    pub(crate) fn view(&self) -> ConsumedNativeValueView<'_> {
        match self {
            Self::Scalar(value) => ConsumedNativeValueView::Scalar(value),
            Self::Struct(value) => ConsumedNativeValueView::Struct(value),
        }
    }

    pub(crate) fn canonical_identity_basis(&self) -> CanonicalAspectValueIdentityBasis {
        match self {
            Self::Scalar(value) => prepare_aspect_value_identity_basis(value),
            Self::Struct(value) => prepare_struct_aspect_value_identity_basis(value),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumedNativeValueView<'a> {
    Scalar(&'a AspectValue),
    Struct(&'a StructAspectValue),
}

impl<'a> ConsumedNativeValueView<'a> {
    pub fn scalar(self) -> Option<&'a AspectValue> {
        match self {
            Self::Scalar(value) => Some(value),
            Self::Struct(_) => None,
        }
    }

    pub fn struct_value(self) -> Option<&'a StructAspectValue> {
        match self {
            Self::Scalar(_) => None,
            Self::Struct(value) => Some(value),
        }
    }
}

use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, prepare_struct_aspect_value_identity_basis, AbsenceLaw,
    AspectValue, CanonicalAspectValueIdentityBasis, StructAspectValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ConsumedNativeValue {
    Scalar(AspectValue),
    Struct(StructAspectValue),
    Absent(AbsenceLaw),
}

impl ConsumedNativeValue {
    pub(crate) fn scalar(value: AspectValue) -> Self {
        Self::Scalar(value)
    }

    pub(crate) fn struct_value(value: StructAspectValue) -> Self {
        Self::Struct(value)
    }

    pub(crate) fn absent(posture: AbsenceLaw) -> Self {
        Self::Absent(posture)
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
            Self::Absent(posture) => ConsumedNativeValueView::Absent(*posture),
        }
    }

    pub(crate) fn canonical_identity_basis(&self) -> ConsumedNativeValueIdentityBasis {
        match self {
            Self::Scalar(value) => {
                ConsumedNativeValueIdentityBasis::Value(prepare_aspect_value_identity_basis(value))
            }
            Self::Struct(value) => ConsumedNativeValueIdentityBasis::Value(
                prepare_struct_aspect_value_identity_basis(value),
            ),
            Self::Absent(posture) => ConsumedNativeValueIdentityBasis::Absent(*posture),
        }
    }
}

pub(crate) enum ConsumedNativeValueIdentityBasis {
    Value(CanonicalAspectValueIdentityBasis),
    Absent(AbsenceLaw),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumedNativeValueView<'a> {
    Scalar(&'a AspectValue),
    Struct(&'a StructAspectValue),
    Absent(AbsenceLaw),
}

impl<'a> ConsumedNativeValueView<'a> {
    pub fn scalar(self) -> Option<&'a AspectValue> {
        match self {
            Self::Scalar(value) => Some(value),
            Self::Struct(_) | Self::Absent(_) => None,
        }
    }

    pub fn struct_value(self) -> Option<&'a StructAspectValue> {
        match self {
            Self::Scalar(_) | Self::Absent(_) => None,
            Self::Struct(value) => Some(value),
        }
    }

    pub fn absence(self) -> Option<AbsenceLaw> {
        match self {
            Self::Absent(posture) => Some(posture),
            Self::Scalar(_) | Self::Struct(_) => None,
        }
    }
}

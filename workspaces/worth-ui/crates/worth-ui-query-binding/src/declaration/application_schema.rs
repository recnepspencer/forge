use worth_foundational::facade::{AspectValue, CanonicalF32, ScalarAspectType};
use worth_query_decl::facade::application_schema::{
    TypedApplicationReadableValue, TypedApplicationValue,
};
use worth_query_decl::facade::{
    worth_query_application_schema, worth_query_aspect, worth_query_entity, worth_query_field,
};

worth_query_application_schema! {
    pub schema WorthUiApplicationSchema {
        owner: worth_ui,
        version: (1, 0),
        members: |schema| {
            schema
                .entity(WorthUiRecord::reference())
                .aspect(WorthUiRecord::reference(), IdentityAspect::reference())
                .aspect(WorthUiRecord::reference(), QueryTextAspect::reference())
                .aspect(WorthUiRecord::reference(), QueryRevisionAspect::reference())
                .aspect(WorthUiRecord::reference(), MeasurementAspect::reference())
                .aspect(WorthUiRecord::reference(), SizeAspect::reference())
                .field(WorthUiRecord::reference(), IdentityIdField::reference())
                .field(WorthUiRecord::reference(), QueryTextStatusField::reference())
                .field(WorthUiRecord::reference(), QueryRevisionValueField::reference())
                .field(WorthUiRecord::reference(), MeasurementValueField::reference())
                .field(WorthUiRecord::reference(), SizeValueField::reference())
        }
    }
}

worth_query_entity!(pub WorthUiRecord in WorthUiApplicationSchema);
worth_query_aspect!(pub IdentityAspect in WorthUiApplicationSchema, WorthUiRecord; identity = AspectIdentity(0x91611056), revision = AspectContractRevision(1),);
worth_query_aspect!(pub QueryTextAspect in WorthUiApplicationSchema, WorthUiRecord; identity = AspectIdentity(0x91611057), revision = AspectContractRevision(1),);
worth_query_aspect!(pub QueryRevisionAspect in WorthUiApplicationSchema, WorthUiRecord; identity = AspectIdentity(0x91611058), revision = AspectContractRevision(1),);
worth_query_aspect!(pub MeasurementAspect in WorthUiApplicationSchema, WorthUiRecord; identity = AspectIdentity(0x91611059), revision = AspectContractRevision(1),);
worth_query_aspect!(pub SizeAspect in WorthUiApplicationSchema, WorthUiRecord; identity = AspectIdentity(0x9161105a), revision = AspectContractRevision(1),);
worth_query_field!(
    pub IdentityIdField in WorthUiApplicationSchema, WorthUiRecord, IdentityAspect:
    String, read_only, equality
);
worth_query_field!(
    pub QueryTextStatusField in WorthUiApplicationSchema, WorthUiRecord, QueryTextAspect:
    String, read_only, equality
);
worth_query_field!(
    pub QueryRevisionValueField in WorthUiApplicationSchema, WorthUiRecord, QueryRevisionAspect:
    u64, read_only, equality
);
worth_query_field!(
    pub MeasurementValueField in WorthUiApplicationSchema, WorthUiRecord, MeasurementAspect:
    UiMeasurementValue, read_only, equality
);
worth_query_field!(
    pub SizeValueField in WorthUiApplicationSchema, WorthUiRecord, SizeAspect:
    UiSizeValue, read_only, equality
);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UiMeasurementValue(CanonicalF32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct UiSizeValue(CanonicalF32);

macro_rules! float_application_value_api {
    ($type:ty) => {
        impl $type {
            pub fn from_f32(value: f32) -> Self {
                Self(CanonicalF32::from_f32(value))
            }

            pub fn as_f32(self) -> f32 {
                self.0.as_f32()
            }
        }
    };
}

float_application_value_api!(UiMeasurementValue);
float_application_value_api!(UiSizeValue);

macro_rules! float_application_value {
    ($type:ty) => {
        impl TypedApplicationValue for $type {
            const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Float32;

            fn into_foundational_value(self) -> AspectValue {
                AspectValue::Float32(self.0)
            }
        }

        impl TypedApplicationReadableValue for $type {
            fn from_foundational_value(value: &AspectValue) -> Option<Self> {
                match value {
                    AspectValue::Float32(value) => Some(Self(*value)),
                    _ => None,
                }
            }
        }
    };
}

float_application_value!(UiMeasurementValue);
float_application_value!(UiSizeValue);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WorthUiProjectionField {
    IdentityId,
    QueryTextStatus,
    QueryRevisionValue,
    MeasurementValue,
    SizeValue,
}

impl WorthUiProjectionField {
    pub(crate) const fn native_key(self) -> &'static str {
        match self {
            Self::IdentityId => "id",
            Self::QueryTextStatus => "status",
            Self::QueryRevisionValue | Self::MeasurementValue | Self::SizeValue => "value",
        }
    }

    pub(crate) const fn collection_contract_key(self) -> &'static str {
        match self {
            Self::IdentityId => "identity.id",
            Self::QueryTextStatus => "query_text.status",
            Self::QueryRevisionValue => "query_revision.value",
            Self::MeasurementValue => "measurement.value",
            Self::SizeValue => "size.value",
        }
    }
}

pub trait WorthUiNativeField: sealed::Sealed {
    const FIELD: WorthUiProjectionField;
}

impl WorthUiNativeField for IdentityIdField {
    const FIELD: WorthUiProjectionField = WorthUiProjectionField::IdentityId;
}

impl WorthUiNativeField for QueryTextStatusField {
    const FIELD: WorthUiProjectionField = WorthUiProjectionField::QueryTextStatus;
}

impl WorthUiNativeField for QueryRevisionValueField {
    const FIELD: WorthUiProjectionField = WorthUiProjectionField::QueryRevisionValue;
}

impl WorthUiNativeField for MeasurementValueField {
    const FIELD: WorthUiProjectionField = WorthUiProjectionField::MeasurementValue;
}

impl WorthUiNativeField for SizeValueField {
    const FIELD: WorthUiProjectionField = WorthUiProjectionField::SizeValue;
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::IdentityIdField {}
    impl Sealed for super::QueryTextStatusField {}
    impl Sealed for super::QueryRevisionValueField {}
    impl Sealed for super::MeasurementValueField {}
    impl Sealed for super::SizeValueField {}
}

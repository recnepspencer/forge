use worth_foundational::facade::{
    CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz, ContentRefId,
    EntityId, InternedString, ScalarAspectType, StructAspectValue,
};

use super::WorthQueryArtifactNativeValueView;
use super::{
    WorthQueryArtifactNativeAccessAdmission, WorthQueryArtifactNativeAccessCounters,
    WorthQueryArtifactNativeAccessDenial, WorthQueryArtifactProviderAccessDenial,
};

#[derive(Clone, Copy, Debug)]
pub enum WorthQueryArtifactNativeFieldSlice<'a> {
    Null(usize),
    Bool(&'a [bool]),
    Int8(&'a [i8]),
    Int16(&'a [i16]),
    Int32(&'a [i32]),
    Int64(&'a [i64]),
    UInt8(&'a [u8]),
    UInt16(&'a [u16]),
    UInt32(&'a [u32]),
    UInt64(&'a [u64]),
    Float32(&'a [CanonicalF32]),
    Float64(&'a [CanonicalF64]),
    Decimal(&'a [CanonicalDecimal]),
    BigInt(&'a [CanonicalBigInt]),
    Rational(&'a [CanonicalRational]),
    String(&'a [InternedString]),
    Bytes(&'a [ContentRefId]),
    Uuid(&'a [[u8; 16]]),
    Date(&'a [CanonicalDate]),
    Time(&'a [CanonicalTime]),
    Timestamp(&'a [CanonicalTimestamp]),
    TimestampTz(&'a [CanonicalTimestampTz]),
    EntityRef(&'a [EntityId]),
    ContentRef(&'a [ContentRefId]),
    Struct(&'a [StructAspectValue]),
}

pub(crate) fn with_borrowed_field<T>(
    admission: &mut WorthQueryArtifactNativeAccessAdmission<'_>,
    start_row: usize,
    max_rows: usize,
    field: &worth_foundational::facade::AspectKey,
    consume: impl for<'view> FnOnce(WorthQueryArtifactNativeFieldSlice<'view>) -> T,
) -> Result<T, WorthQueryArtifactNativeAccessDenial> {
    let layout = admission.native_contract().layout().clone();
    let field = field.clone();
    let (value, increment) = admission.with_provider(|provider, session| {
        let values = provider.borrow_field(session, start_row, max_rows, &field)?;
        if values.len() > max_rows {
            return Err(WorthQueryArtifactProviderAccessDenial::BoundsExceeded);
        }
        let Some(contract) = layout
            .fields()
            .iter()
            .find(|contract| contract.aspect().key() == &field)
        else {
            return Err(WorthQueryArtifactProviderAccessDenial::ShapeMismatch);
        };
        let matches = match contract.aspect().shape() {
            worth_foundational::facade::AspectShape::Scalar(family) => {
                values.matches_scalar_family(*family)
            }
            worth_foundational::facade::AspectShape::Struct(_) => values.is_struct(),
            worth_foundational::facade::AspectShape::Opaque(_)
            | worth_foundational::facade::AspectShape::Reference(_)
            | worth_foundational::facade::AspectShape::Content => false,
        };
        if !matches {
            return Err(WorthQueryArtifactProviderAccessDenial::ShapeMismatch);
        }
        let increment = WorthQueryArtifactNativeAccessCounters {
            field_slice_contacts: 1,
            rows_exposed: values.len(),
            values_exposed: values.len(),
            ..WorthQueryArtifactNativeAccessCounters::default()
        };
        Ok((consume(values), increment))
    })?;
    admission.counters_mut().accumulate(increment);
    Ok(value)
}

impl<'a> WorthQueryArtifactNativeFieldSlice<'a> {
    pub fn len(self) -> usize {
        match self {
            Self::Null(len) => len,
            Self::Bool(values) => values.len(),
            Self::Int8(values) => values.len(),
            Self::Int16(values) => values.len(),
            Self::Int32(values) => values.len(),
            Self::Int64(values) => values.len(),
            Self::UInt8(values) => values.len(),
            Self::UInt16(values) => values.len(),
            Self::UInt32(values) => values.len(),
            Self::UInt64(values) => values.len(),
            Self::Float32(values) => values.len(),
            Self::Float64(values) => values.len(),
            Self::Decimal(values) => values.len(),
            Self::BigInt(values) => values.len(),
            Self::Rational(values) => values.len(),
            Self::String(values) => values.len(),
            Self::Bytes(values) => values.len(),
            Self::Uuid(values) => values.len(),
            Self::Date(values) => values.len(),
            Self::Time(values) => values.len(),
            Self::Timestamp(values) => values.len(),
            Self::TimestampTz(values) => values.len(),
            Self::EntityRef(values) => values.len(),
            Self::ContentRef(values) => values.len(),
            Self::Struct(values) => values.len(),
        }
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn value(self, row: usize) -> Option<WorthQueryArtifactNativeValueView<'a>> {
        Some(match self {
            Self::Null(len) if row < len => WorthQueryArtifactNativeValueView::Null,
            Self::Bool(values) => WorthQueryArtifactNativeValueView::Bool(values.get(row)?),
            Self::Int8(values) => WorthQueryArtifactNativeValueView::Int8(values.get(row)?),
            Self::Int16(values) => WorthQueryArtifactNativeValueView::Int16(values.get(row)?),
            Self::Int32(values) => WorthQueryArtifactNativeValueView::Int32(values.get(row)?),
            Self::Int64(values) => WorthQueryArtifactNativeValueView::Int64(values.get(row)?),
            Self::UInt8(values) => WorthQueryArtifactNativeValueView::UInt8(values.get(row)?),
            Self::UInt16(values) => WorthQueryArtifactNativeValueView::UInt16(values.get(row)?),
            Self::UInt32(values) => WorthQueryArtifactNativeValueView::UInt32(values.get(row)?),
            Self::UInt64(values) => WorthQueryArtifactNativeValueView::UInt64(values.get(row)?),
            Self::Float32(values) => WorthQueryArtifactNativeValueView::Float32(values.get(row)?),
            Self::Float64(values) => WorthQueryArtifactNativeValueView::Float64(values.get(row)?),
            Self::Decimal(values) => WorthQueryArtifactNativeValueView::Decimal(values.get(row)?),
            Self::BigInt(values) => WorthQueryArtifactNativeValueView::BigInt(values.get(row)?),
            Self::Rational(values) => WorthQueryArtifactNativeValueView::Rational(values.get(row)?),
            Self::String(values) => WorthQueryArtifactNativeValueView::String(values.get(row)?),
            Self::Bytes(values) => WorthQueryArtifactNativeValueView::Bytes(values.get(row)?),
            Self::Uuid(values) => WorthQueryArtifactNativeValueView::Uuid(values.get(row)?),
            Self::Date(values) => WorthQueryArtifactNativeValueView::Date(values.get(row)?),
            Self::Time(values) => WorthQueryArtifactNativeValueView::Time(values.get(row)?),
            Self::Timestamp(values) => {
                WorthQueryArtifactNativeValueView::Timestamp(values.get(row)?)
            }
            Self::TimestampTz(values) => {
                WorthQueryArtifactNativeValueView::TimestampTz(values.get(row)?)
            }
            Self::EntityRef(values) => {
                WorthQueryArtifactNativeValueView::EntityRef(values.get(row)?)
            }
            Self::ContentRef(values) => {
                WorthQueryArtifactNativeValueView::ContentRef(values.get(row)?)
            }
            Self::Struct(values) => WorthQueryArtifactNativeValueView::Struct(values.get(row)?),
            Self::Null(_) => return None,
        })
    }

    pub(crate) fn matches_scalar_family(self, family: ScalarAspectType) -> bool {
        matches!(
            (self, family),
            (Self::Null(_), ScalarAspectType::Null)
                | (Self::Bool(_), ScalarAspectType::Bool)
                | (Self::Int8(_), ScalarAspectType::Int8)
                | (Self::Int16(_), ScalarAspectType::Int16)
                | (Self::Int32(_), ScalarAspectType::Int32)
                | (Self::Int64(_), ScalarAspectType::Int64)
                | (Self::UInt8(_), ScalarAspectType::UInt8)
                | (Self::UInt16(_), ScalarAspectType::UInt16)
                | (Self::UInt32(_), ScalarAspectType::UInt32)
                | (Self::UInt64(_), ScalarAspectType::UInt64)
                | (Self::Float32(_), ScalarAspectType::Float32)
                | (Self::Float64(_), ScalarAspectType::Float64)
                | (Self::Decimal(_), ScalarAspectType::Decimal)
                | (Self::BigInt(_), ScalarAspectType::BigInt)
                | (Self::Rational(_), ScalarAspectType::Rational)
                | (Self::String(_), ScalarAspectType::String)
                | (Self::Bytes(_), ScalarAspectType::Bytes)
                | (Self::Uuid(_), ScalarAspectType::Uuid)
                | (Self::Date(_), ScalarAspectType::Date)
                | (Self::Time(_), ScalarAspectType::Time)
                | (Self::Timestamp(_), ScalarAspectType::Timestamp)
                | (Self::TimestampTz(_), ScalarAspectType::TimestampTz)
                | (Self::EntityRef(_), ScalarAspectType::EntityRef)
                | (Self::ContentRef(_), ScalarAspectType::ContentRef)
        )
    }

    pub(crate) fn is_struct(self) -> bool {
        matches!(self, Self::Struct(_))
    }
}

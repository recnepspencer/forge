use worth_query_installation::facade::{
    WorthQueryImmutableSourceOccurrenceContract as SourceOccurrence,
    WorthQuerySourceOutputCorrespondence as Correspondence,
    WorthQueryTransformationDisposition as Disposition,
    WorthQueryTransformationErrorPosture as ErrorPosture,
    WorthQueryTransformationEvidenceContract as Transformation,
    WorthQueryTransformationIdentity as TransformationIdentity,
    WorthQueryTransformationLossPosture as LossPosture,
    WorthQueryTransformationOutcomeContract as Outcome,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(super) fn write_transformation(
    output: &mut dyn BinaryEncodingSink,
    contract: &Transformation,
) -> Result<(), Denial> {
    match contract {
        Transformation::NotTransformation => output.u16(1),
        Transformation::Declared {
            source_occurrence,
            transformation,
            outcome,
        } => {
            output.u16(2)?;
            output.text(source_occurrence.identity_family())?;
            output.text(transformation.family())?;
            output.u32(transformation.version())?;
            output.u16(correspondence_tag(outcome.correspondence()))?;
            output.u16(disposition_tag(outcome.disposition()))?;
            output.u16(error_tag(outcome.error()))?;
            output.u16(loss_tag(outcome.loss()))
        }
    }
}

pub(super) fn decode_transformation(input: &mut BinaryInput<'_>) -> Result<Transformation, Denial> {
    match input.u16()? {
        1 => Ok(Transformation::NotTransformation),
        2 => Ok(Transformation::declared(
            SourceOccurrence::new(input.text()?.to_owned()),
            TransformationIdentity::new(input.text()?.to_owned(), input.u32()?),
            Outcome::new(
                correspondence_from_tag(input.u16()?)?,
                disposition_from_tag(input.u16()?)?,
                error_from_tag(input.u16()?)?,
                loss_from_tag(input.u16()?)?,
            ),
        )),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn correspondence_tag(value: Correspondence) -> u16 {
    match value {
        Correspondence::OneToOne => 1,
        Correspondence::OneToMany => 2,
        Correspondence::ManyToOne => 3,
        Correspondence::ManyToMany => 4,
        Correspondence::Partial => 5,
    }
}

fn correspondence_from_tag(tag: u16) -> Result<Correspondence, Denial> {
    match tag {
        1 => Ok(Correspondence::OneToOne),
        2 => Ok(Correspondence::OneToMany),
        3 => Ok(Correspondence::ManyToOne),
        4 => Ok(Correspondence::ManyToMany),
        5 => Ok(Correspondence::Partial),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn disposition_tag(value: Disposition) -> u16 {
    match value {
        Disposition::Preserved => 1,
        Disposition::Normalized => 2,
        Disposition::Approximated => 3,
        Disposition::Repaired => 4,
        Disposition::Omitted => 5,
        Disposition::Unsupported => 6,
        Disposition::Quarantined => 7,
    }
}

fn disposition_from_tag(tag: u16) -> Result<Disposition, Denial> {
    match tag {
        1 => Ok(Disposition::Preserved),
        2 => Ok(Disposition::Normalized),
        3 => Ok(Disposition::Approximated),
        4 => Ok(Disposition::Repaired),
        5 => Ok(Disposition::Omitted),
        6 => Ok(Disposition::Unsupported),
        7 => Ok(Disposition::Quarantined),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn error_tag(value: ErrorPosture) -> u16 {
    match value {
        ErrorPosture::Exact => 1,
        ErrorPosture::Bounded => 2,
        ErrorPosture::Estimated => 3,
        ErrorPosture::Unknown => 4,
    }
}

fn error_from_tag(tag: u16) -> Result<ErrorPosture, Denial> {
    match tag {
        1 => Ok(ErrorPosture::Exact),
        2 => Ok(ErrorPosture::Bounded),
        3 => Ok(ErrorPosture::Estimated),
        4 => Ok(ErrorPosture::Unknown),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn loss_tag(value: LossPosture) -> u16 {
    match value {
        LossPosture::Lossless => 1,
        LossPosture::DeclaredLossy => 2,
        LossPosture::LossClassifiedByDomain => 3,
    }
}

fn loss_from_tag(tag: u16) -> Result<LossPosture, Denial> {
    match tag {
        1 => Ok(LossPosture::Lossless),
        2 => Ok(LossPosture::DeclaredLossy),
        3 => Ok(LossPosture::LossClassifiedByDomain),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

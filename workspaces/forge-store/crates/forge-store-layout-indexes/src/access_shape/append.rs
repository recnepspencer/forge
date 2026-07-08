use super::contract::S8ExpectedCounterClass;
use super::contract::{S8AccessShapeContract, S8AccessStaleDisposition};
use super::denial::S8AccessShapeUnsupportedDenial;
use super::detail::{S8AccessShapeDetail, S8MutationAccessBasis};
use super::lane::S8AccessLaneClassification;
use super::shape::S8AccessShape;
use crate::maintenance::S8PhysicalMutationShape;

pub(crate) fn append_path(
    mutation_shape: S8PhysicalMutationShape,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    if mutation_shape != S8PhysicalMutationShape::LogStructuredAppend {
        return Err(
            S8AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: S8AccessShape::Append,
                mutation_shape,
            },
        );
    }

    Ok(S8AccessShapeContract::mutation_path(
        S8AccessShapeDetail::Append(S8MutationAccessBasis::WalBeforeDataAppend),
        S8AccessLaneClassification::Maintenance,
        S8AccessStaleDisposition::RebindBeforeExecution,
        S8ExpectedCounterClass::AppendTraversal,
        mutation_shape,
    ))
}

pub(crate) fn compaction_read(
    mutation_shape: S8PhysicalMutationShape,
) -> Result<S8AccessShapeContract, S8AccessShapeUnsupportedDenial> {
    if mutation_shape != S8PhysicalMutationShape::CompactionRewrite {
        return Err(
            S8AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: S8AccessShape::CompactionRead,
                mutation_shape,
            },
        );
    }

    Ok(S8AccessShapeContract::mutation_path(
        S8AccessShapeDetail::CompactionRead(S8MutationAccessBasis::CompactionRewriteTraversal),
        S8AccessLaneClassification::Maintenance,
        S8AccessStaleDisposition::RebindBeforeExecution,
        S8ExpectedCounterClass::CompactionTraversal,
        mutation_shape,
    ))
}

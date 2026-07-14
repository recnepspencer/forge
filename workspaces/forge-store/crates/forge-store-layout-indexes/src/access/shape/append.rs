use super::contract::ExpectedCounterClass;
use super::contract::{AccessShapeContract, AccessStaleDisposition};
use super::denial::AccessShapeUnsupportedDenial;
use super::detail::{AccessShapeDetail, MutationAccessBasis};
use super::kind::AccessShape;
use super::lane::AccessLaneClassification;
use crate::maintenance::PhysicalMutationShape;

pub(crate) fn append_path(
    mutation_shape: PhysicalMutationShape,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if mutation_shape != PhysicalMutationShape::LogStructuredAppend {
        return Err(
            AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: AccessShape::Append,
                mutation_shape,
            },
        );
    }

    Ok(AccessShapeContract::mutation_path(
        AccessShapeDetail::Append(MutationAccessBasis::WalBeforeDataAppend),
        AccessLaneClassification::Maintenance,
        AccessStaleDisposition::RebindBeforeExecution,
        ExpectedCounterClass::AppendTraversal,
        mutation_shape,
    ))
}

pub(crate) fn compaction_read(
    mutation_shape: PhysicalMutationShape,
) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
    if mutation_shape != PhysicalMutationShape::CompactionRewrite {
        return Err(
            AccessShapeUnsupportedDenial::MutationShapeDoesNotSupportAccessShape {
                requested_shape: AccessShape::CompactionRead,
                mutation_shape,
            },
        );
    }

    Ok(AccessShapeContract::mutation_path(
        AccessShapeDetail::CompactionRead(MutationAccessBasis::CompactionRewriteTraversal),
        AccessLaneClassification::Maintenance,
        AccessStaleDisposition::RebindBeforeExecution,
        ExpectedCounterClass::CompactionTraversal,
        mutation_shape,
    ))
}

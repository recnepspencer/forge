mod bindings;
mod digest;
mod evaluation;
mod model;

use crate::workload_platform::evidence_lookup_query_surface_matrix::current_evidence_lookup_query_surface_matrix;

#[cfg(test)]
pub(crate) use evaluation::evaluate_consumer_kit_closeout_from_parts;
pub use model::EvidenceLookupQueryConsumerKitCloseout;

use super::error::{EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind};

pub fn current_evidence_lookup_query_consumer_kit(
) -> Result<EvidenceLookupQueryConsumerKitCloseout, EvidenceLookupQueryConsumerKitError> {
    let matrix = current_evidence_lookup_query_surface_matrix().map_err(|error| {
        EvidenceLookupQueryConsumerKitError::new(
            EvidenceLookupQueryConsumerKitErrorKind::QuerySurfaceMatrix,
            format!("{:?}", error.kind()),
        )
    })?;
    evaluation::project_current_closeout(matrix)
}

use std::sync::Arc;

/// One-way text projection for diagnostics, inspection, and external evidence.
///
/// Operational consumers must use `UiQueryEvidenceReference` instead. This
/// projection carries no Query identity or admission authority back inward.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiProjectionFactReportingProjection {
    query_world: Arc<str>,
    binding: Arc<str>,
    source_generation: Arc<str>,
    result_generation: Arc<str>,
}

impl UiProjectionFactReportingProjection {
    pub fn query_world(&self) -> &str {
        &self.query_world
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }

    pub fn source_generation(&self) -> &str {
        &self.source_generation
    }

    pub fn result_generation(&self) -> &str {
        &self.result_generation
    }
}

pub(super) fn project(
    fact: &super::UiProjectionFactReceipt,
) -> UiProjectionFactReportingProjection {
    UiProjectionFactReportingProjection {
        query_world: Arc::from(
            fact.query_world_authority()
                .terminal_projection_for_reporting(),
        ),
        binding: Arc::from(fact.binding_authority().terminal_projection_for_reporting()),
        source_generation: Arc::from(
            fact.source_generation_authority()
                .terminal_projection_for_reporting(),
        ),
        result_generation: Arc::from(
            fact.result_generation_authority()
                .terminal_projection_for_reporting(),
        ),
    }
}

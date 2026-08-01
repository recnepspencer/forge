use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionFactReceipt {
    projection_identity: crate::WorthUiQueryViewIdentity,
    observation_order: u64,
    query_world_identity: WorthQueryEvidenceIdentity,
    binding_identity: WorthQueryEvidenceIdentity,
    source_generation_identity: WorthQueryEvidenceIdentity,
    result_generation_identity: WorthQueryEvidenceIdentity,
}

pub(crate) struct UiProjectionFactReceiptInput {
    pub(crate) projection_identity: crate::WorthUiQueryViewIdentity,
    pub(crate) observation_order: u64,
    pub(crate) query_world_identity: WorthQueryEvidenceIdentity,
    pub(crate) binding_identity: WorthQueryEvidenceIdentity,
    pub(crate) source_generation_identity: WorthQueryEvidenceIdentity,
    pub(crate) result_generation_identity: WorthQueryEvidenceIdentity,
}

impl UiProjectionFactReceipt {
    pub(crate) fn admitted(input: UiProjectionFactReceiptInput) -> Self {
        Self {
            projection_identity: input.projection_identity,
            observation_order: input.observation_order,
            query_world_identity: input.query_world_identity,
            binding_identity: input.binding_identity,
            source_generation_identity: input.source_generation_identity,
            result_generation_identity: input.result_generation_identity,
        }
    }

    pub fn projection_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        &self.projection_identity
    }

    pub(crate) fn observation_order(&self) -> u64 {
        self.observation_order
    }

    pub(crate) fn query_world_authority(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_world_identity
    }

    pub(crate) fn binding_authority(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub(crate) fn source_generation_authority(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_generation_identity
    }

    pub(crate) fn result_generation_authority(&self) -> &WorthQueryEvidenceIdentity {
        &self.result_generation_identity
    }

    pub fn query_world_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.query_world_identity)
    }

    pub fn binding_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.binding_identity)
    }

    pub fn source_generation_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.source_generation_identity)
    }

    pub fn result_generation_identity(&self) -> crate::UiQueryEvidenceReference {
        crate::UiQueryEvidenceReference::query_issued(&self.result_generation_identity)
    }

    pub fn reporting_projection(&self) -> super::UiProjectionFactReportingProjection {
        super::reporting::project(self)
    }
}

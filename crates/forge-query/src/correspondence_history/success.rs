use crate::correspondence::{
    CorrespondenceAmbiguityEnvelope, CorrespondenceDisagreementEnvelope,
    CorrespondenceEvidenceResolved,
};
use crate::execution::ExecutionResultEnvelope;
use crate::historical::{HistoricalMaterializationPathMetadata, HistoricalPathResolved};

use super::view::{build_result_view, MetadataPreservingHistoricalResultView};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalSuccessEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalSuccessEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        build_result_view(
            &self.execution,
            &self.correspondence,
            &self.materialization_metadata,
            &self.historical,
        )
    }

    pub(crate) fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            historical,
            materialization_metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalAmbiguityEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    ambiguity: CorrespondenceAmbiguityEnvelope,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalAmbiguityEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn ambiguity(&self) -> &CorrespondenceAmbiguityEnvelope {
        &self.ambiguity
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        build_result_view(
            &self.execution,
            &self.correspondence,
            &self.materialization_metadata,
            &self.historical,
        )
    }

    pub(crate) fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        ambiguity: CorrespondenceAmbiguityEnvelope,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            ambiguity,
            historical,
            materialization_metadata,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoricalDisagreementEnvelope {
    execution: ExecutionResultEnvelope,
    correspondence: CorrespondenceEvidenceResolved,
    disagreement: CorrespondenceDisagreementEnvelope,
    historical: HistoricalPathResolved,
    materialization_metadata: HistoricalMaterializationPathMetadata,
}

impl CorrespondenceHistoricalDisagreementEnvelope {
    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }

    pub fn disagreement(&self) -> &CorrespondenceDisagreementEnvelope {
        &self.disagreement
    }

    pub fn historical(&self) -> &HistoricalPathResolved {
        &self.historical
    }

    pub fn materialization_metadata(&self) -> &HistoricalMaterializationPathMetadata {
        &self.materialization_metadata
    }

    pub fn result_view(&self) -> MetadataPreservingHistoricalResultView<'_> {
        build_result_view(
            &self.execution,
            &self.correspondence,
            &self.materialization_metadata,
            &self.historical,
        )
    }

    pub(crate) fn new(
        execution: ExecutionResultEnvelope,
        correspondence: CorrespondenceEvidenceResolved,
        disagreement: CorrespondenceDisagreementEnvelope,
        historical: HistoricalPathResolved,
    ) -> Self {
        let materialization_metadata =
            HistoricalMaterializationPathMetadata::from_resolved(historical.clone());
        Self {
            execution,
            correspondence,
            disagreement,
            historical,
            materialization_metadata,
        }
    }
}

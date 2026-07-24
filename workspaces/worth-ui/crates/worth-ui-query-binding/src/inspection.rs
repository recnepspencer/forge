use crate::{WorthUiQueryInspectionRelevance, WorthUiSettledSnapshotProjection};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryInspectionEvidencePolicy {
    Minimal,
    Rich,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiQueryInspectionMaterializationCounters {
    exact_artifact_link_count: usize,
    compact_field_count: usize,
    rich_evidence_section_count: usize,
}

#[derive(Debug)]
pub struct WorthUiExactQueryArtifactInspection<'artifact, Artifact> {
    exact_artifact: &'artifact Artifact,
    relevance: WorthUiQueryInspectionRelevance,
    counters: WorthUiQueryInspectionMaterializationCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSettledSnapshotRichEvidence {
    execution_warning_count: usize,
    projection_warning_count: usize,
    conditional_provenance_count: usize,
}

pub struct WorthUiSettledSnapshotInspection<'projection> {
    exact_projection: &'projection WorthUiSettledSnapshotProjection,
    relevance: WorthUiQueryInspectionRelevance,
    rich_evidence: Option<WorthUiSettledSnapshotRichEvidence>,
    counters: WorthUiQueryInspectionMaterializationCounters,
}

pub struct WorthUiQueryInspection;

impl WorthUiQueryInspection {
    /// Links a diagnostic row to the exact upstream artifact without copying
    /// or formatting Query-owned meaning.
    ///
    /// Artifact-specific rich diagnostics are read from that typed artifact.
    /// Only projections with UI-owned derived evidence accept an evidence
    /// policy below.
    pub fn exact_artifact<'artifact, Artifact>(
        artifact: &'artifact Artifact,
        relevance: WorthUiQueryInspectionRelevance,
    ) -> WorthUiExactQueryArtifactInspection<'artifact, Artifact> {
        WorthUiExactQueryArtifactInspection {
            exact_artifact: artifact,
            relevance,
            counters: WorthUiQueryInspectionMaterializationCounters {
                exact_artifact_link_count: 1,
                compact_field_count: 1,
                rich_evidence_section_count: 0,
            },
        }
    }

    pub fn settled_projection(
        projection: &WorthUiSettledSnapshotProjection,
        relevance: WorthUiQueryInspectionRelevance,
        policy: WorthUiQueryInspectionEvidencePolicy,
    ) -> WorthUiSettledSnapshotInspection<'_> {
        let rich_evidence =
            matches!(policy, WorthUiQueryInspectionEvidencePolicy::Rich).then(|| {
                WorthUiSettledSnapshotRichEvidence {
                    execution_warning_count: projection.execution_warnings().len(),
                    projection_warning_count: projection
                        .projection_warnings()
                        .map_or(0, |warnings| warnings.warning_kinds().len()),
                    conditional_provenance_count: projection.conditional_provenance().len(),
                }
            });
        let rich_evidence_section_count = usize::from(rich_evidence.is_some());
        WorthUiSettledSnapshotInspection {
            exact_projection: projection,
            relevance,
            rich_evidence,
            counters: WorthUiQueryInspectionMaterializationCounters {
                exact_artifact_link_count: 1,
                compact_field_count: 3,
                rich_evidence_section_count,
            },
        }
    }
}

impl WorthUiQueryInspectionMaterializationCounters {
    pub fn exact_artifact_link_count(self) -> usize {
        self.exact_artifact_link_count
    }

    pub fn compact_field_count(self) -> usize {
        self.compact_field_count
    }

    pub fn rich_evidence_section_count(self) -> usize {
        self.rich_evidence_section_count
    }
}

impl<Artifact> WorthUiExactQueryArtifactInspection<'_, Artifact> {
    pub fn exact_artifact(&self) -> &Artifact {
        self.exact_artifact
    }

    pub fn relevance(&self) -> WorthUiQueryInspectionRelevance {
        self.relevance
    }

    pub fn counters(&self) -> WorthUiQueryInspectionMaterializationCounters {
        self.counters
    }
}

impl WorthUiSettledSnapshotInspection<'_> {
    pub fn exact_projection(&self) -> &WorthUiSettledSnapshotProjection {
        self.exact_projection
    }

    pub fn relevance(&self) -> WorthUiQueryInspectionRelevance {
        self.relevance
    }

    pub fn settlement_reference(&self) -> &crate::WorthUiAdmittedQuerySettlementReference {
        self.exact_projection.fact().settlement_reference()
    }

    pub fn binding_reference(&self) -> &crate::WorthUiAdmittedQueryBindingReference {
        self.exact_projection.fact().binding_reference()
    }

    pub fn result_state(
        &self,
    ) -> worth_query::facade::installed::operation::WorthQueryOperationResultState {
        self.exact_projection.result_state()
    }

    pub fn rich_evidence(&self) -> Option<WorthUiSettledSnapshotRichEvidence> {
        self.rich_evidence
    }

    pub fn counters(&self) -> WorthUiQueryInspectionMaterializationCounters {
        self.counters
    }
}

impl WorthUiSettledSnapshotRichEvidence {
    pub fn execution_warning_count(self) -> usize {
        self.execution_warning_count
    }

    pub fn projection_warning_count(self) -> usize {
        self.projection_warning_count
    }

    pub fn conditional_provenance_count(self) -> usize {
        self.conditional_provenance_count
    }
}

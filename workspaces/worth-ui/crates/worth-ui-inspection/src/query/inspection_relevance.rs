use crate::UiInspectionEvidenceSource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionRelevance {
    Only(UiInspectionEvidenceSource),
    QueryBackedOnly,
    AllSources,
}

impl UiInspectionRelevance {
    pub fn worth_local_only() -> Self {
        Self::Only(UiInspectionEvidenceSource::WorthLocal)
    }

    pub fn query_projection_consumption_only() -> Self {
        Self::Only(UiInspectionEvidenceSource::QueryProjectionConsumption)
    }

    pub fn query_inspection_only() -> Self {
        Self::Only(UiInspectionEvidenceSource::QueryInspection)
    }

    pub fn query_backed_only() -> Self {
        Self::QueryBackedOnly
    }

    pub fn all_sources() -> Self {
        Self::AllSources
    }

    pub fn includes(self, source: UiInspectionEvidenceSource) -> bool {
        match self {
            Self::Only(selected) => selected == source,
            Self::QueryBackedOnly => {
                matches!(
                    source,
                    UiInspectionEvidenceSource::QueryInspection
                        | UiInspectionEvidenceSource::QueryProjectionConsumption
                        | UiInspectionEvidenceSource::QueryCausalExplanation
                )
            }
            Self::AllSources => true,
        }
    }

    pub fn includes_worth_local_evidence(self) -> bool {
        self.includes(UiInspectionEvidenceSource::WorthLocal)
    }

    pub fn includes_query_inspection(self) -> bool {
        self.includes(UiInspectionEvidenceSource::QueryInspection)
    }

    pub fn includes_query_projection_consumption(self) -> bool {
        self.includes(UiInspectionEvidenceSource::QueryProjectionConsumption)
    }

    pub fn includes_query_causal_explanation(self) -> bool {
        self.includes(UiInspectionEvidenceSource::QueryCausalExplanation)
    }
}

impl Default for UiInspectionRelevance {
    fn default() -> Self {
        Self::worth_local_only()
    }
}

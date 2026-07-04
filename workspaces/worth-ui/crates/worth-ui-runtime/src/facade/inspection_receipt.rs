use crate::evidence::{
    UiEvidenceAuthorityGeneration, UiEvidenceSlice, UiEvidenceSliceAssembly,
    UiEvidenceSliceAssemblyInput, UiEvidenceSliceRef, UiInspectionCostMetrics,
    UiInspectionCostReceipt, UiInspectionObligationEvidenceReceipt,
};
use worth_ui_inspection::{
    UiInspectionPosture, UiInspectionQuery, UiInspectionRelevance, UiInspectionRelevanceAdmission,
    UiInspectionRelevanceOutcome, UiInspectionSupportReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionReceipt {
    query: UiInspectionQuery,
    relevance_admission: UiInspectionRelevanceAdmission,
    selected_relevance: UiInspectionRelevance,
    authority_generation: Option<UiEvidenceAuthorityGeneration>,
    posture: Option<UiInspectionPosture>,
    support_report: Option<UiInspectionSupportReport>,
    evidence_slice_ref: Option<UiEvidenceSliceRef>,
    evidence_slice: Option<UiEvidenceSlice>,
    cost: Option<UiInspectionCostReceipt>,
}

impl UiInspectionReceipt {
    pub(crate) fn from_relevance_admission(
        query: UiInspectionQuery,
        relevance_admission: UiInspectionRelevanceAdmission,
        authority_generation: Option<UiEvidenceAuthorityGeneration>,
    ) -> Self {
        Self {
            selected_relevance: query.relevance().clone(),
            query,
            relevance_admission,
            authority_generation,
            posture: None,
            support_report: None,
            evidence_slice_ref: None,
            evidence_slice: None,
            cost: None,
        }
    }

    pub(crate) fn from_support(
        query: UiInspectionQuery,
        relevance_admission: UiInspectionRelevanceAdmission,
        support_report: UiInspectionSupportReport,
        authority_generation: Option<UiEvidenceAuthorityGeneration>,
    ) -> Self {
        let posture = UiInspectionPosture::from_support_report(support_report);
        Self {
            selected_relevance: query.relevance().clone(),
            query,
            relevance_admission,
            authority_generation,
            posture: Some(posture),
            support_report: Some(support_report),
            evidence_slice_ref: None,
            evidence_slice: None,
            cost: None,
        }
    }

    pub(crate) fn from_obligation(
        query: UiInspectionQuery,
        relevance_admission: UiInspectionRelevanceAdmission,
        authority_generation: UiEvidenceAuthorityGeneration,
        obligation_evidence: UiInspectionObligationEvidenceReceipt,
    ) -> Self {
        let assembly = UiEvidenceSliceAssembly::assemble(
            &query,
            UiEvidenceSliceAssemblyInput::new(
                authority_generation,
                obligation_evidence.refs().to_vec().into_boxed_slice(),
            )
            .with_materialized_detail(
                matches!(
                    query.richness(),
                    worth_ui_inspection::UiEvidenceRichness::MaterializedDetail
                )
                .then_some(crate::evidence::UiEvidenceMaterializedDetail::Obligation(
                    obligation_evidence.clone(),
                )),
            )
            .with_detail_available(true)
            .with_cost_metrics(UiInspectionCostMetrics::new(
                1,
                obligation_evidence.refs().len(),
                0,
                false,
            )),
        );
        let cost = assembly.cost();
        let evidence_slice_ref = assembly.slice().slice_ref();
        Self {
            selected_relevance: query.relevance().clone(),
            query,
            relevance_admission,
            authority_generation: Some(authority_generation),
            posture: None,
            support_report: None,
            evidence_slice_ref: Some(evidence_slice_ref),
            evidence_slice: Some(assembly.into_slice()),
            cost: Some(cost),
        }
    }

    pub(crate) fn from_assembled_slice(
        query: UiInspectionQuery,
        relevance_admission: UiInspectionRelevanceAdmission,
        authority_generation: UiEvidenceAuthorityGeneration,
        assembly: UiEvidenceSliceAssembly,
    ) -> Self {
        let cost = assembly.cost();
        let evidence_slice_ref = assembly.slice().slice_ref();
        Self {
            selected_relevance: query.relevance().clone(),
            query,
            relevance_admission,
            authority_generation: Some(authority_generation),
            posture: None,
            support_report: None,
            evidence_slice_ref: Some(evidence_slice_ref),
            evidence_slice: Some(assembly.into_slice()),
            cost: Some(cost),
        }
    }

    pub fn query(&self) -> &UiInspectionQuery {
        &self.query
    }

    pub fn relevance_admission(&self) -> &UiInspectionRelevanceAdmission {
        &self.relevance_admission
    }

    pub fn selected_relevance(&self) -> &UiInspectionRelevance {
        &self.selected_relevance
    }

    pub fn relevance_outcome(&self) -> UiInspectionRelevanceOutcome {
        self.relevance_admission.outcome()
    }

    pub fn authority_generation(&self) -> Option<UiEvidenceAuthorityGeneration> {
        self.authority_generation
    }

    pub fn posture(&self) -> Option<UiInspectionPosture> {
        self.posture
    }

    pub fn support_report(&self) -> Option<UiInspectionSupportReport> {
        self.support_report
    }

    pub fn evidence_slice_ref(&self) -> Option<UiEvidenceSliceRef> {
        self.evidence_slice_ref
    }

    pub fn evidence_slice(&self) -> Option<&UiEvidenceSlice> {
        self.evidence_slice.as_ref()
    }

    pub fn cost(&self) -> Option<UiInspectionCostReceipt> {
        self.cost
    }
}

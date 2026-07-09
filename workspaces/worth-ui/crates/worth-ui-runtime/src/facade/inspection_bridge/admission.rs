use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiInspectionQuery, UiInspectionRelevanceAdmission,
    UiInspectionRelevanceOutcome, UiInspectionSupportPosture, UiInspectionSupportReport,
};

use crate::facade::inspection_receipt::UiInspectionReceipt;
use crate::facade::lifecycle::WorthUiFacadeLifecycleBootstrap;

pub(crate) struct InspectionAuthority {
    pub generation: Option<UiEvidenceAuthorityGeneration>,
}

pub(crate) fn collect_inspection_authority(graph_generation: u64) -> InspectionAuthority {
    InspectionAuthority {
        generation: Some(UiEvidenceAuthorityGeneration::new(graph_generation)),
    }
}

pub(crate) enum RelevanceAdmissionDecision {
    Matched(UiInspectionRelevanceAdmission),
    Denied(UiInspectionReceipt),
}

pub(crate) fn decide_relevance_admission(
    query: UiInspectionQuery,
    authority: &InspectionAuthority,
) -> RelevanceAdmissionDecision {
    let admission = query.admit_relevance();
    if matches!(admission.outcome(), UiInspectionRelevanceOutcome::Matched) {
        RelevanceAdmissionDecision::Matched(admission)
    } else {
        RelevanceAdmissionDecision::Denied(UiInspectionReceipt::from_relevance_admission(
            query,
            admission,
            authority.generation,
        ))
    }
}

pub(crate) enum SupportAdmissionDecision {
    Matched {
        admission: UiInspectionRelevanceAdmission,
        support_report: UiInspectionSupportReport,
    },
    Denied(UiInspectionReceipt),
}

pub(crate) fn decide_support_admission(
    query: UiInspectionQuery,
    support_report: UiInspectionSupportReport,
    lifecycle: &WorthUiFacadeLifecycleBootstrap,
    authority: &InspectionAuthority,
) -> SupportAdmissionDecision {
    let admission = query
        .admit_relevance()
        .refined_for_support_report(support_report);
    if !matches!(
        support_report.posture(),
        UiInspectionSupportPosture::Supported
    ) {
        lifecycle.record_unsupported_inspection_query();
    }
    if matches!(admission.outcome(), UiInspectionRelevanceOutcome::Matched) {
        SupportAdmissionDecision::Matched {
            admission,
            support_report,
        }
    } else {
        SupportAdmissionDecision::Denied(UiInspectionReceipt::from_support(
            query,
            admission,
            support_report,
            authority.generation,
        ))
    }
}

pub(crate) fn assemble_relevance_receipt(
    query: UiInspectionQuery,
    admission: UiInspectionRelevanceAdmission,
    authority: &InspectionAuthority,
) -> UiInspectionReceipt {
    UiInspectionReceipt::from_relevance_admission(query, admission, authority.generation)
}

pub(crate) fn assemble_support_receipt(
    query: UiInspectionQuery,
    admission: UiInspectionRelevanceAdmission,
    support_report: UiInspectionSupportReport,
    authority: &InspectionAuthority,
) -> UiInspectionReceipt {
    UiInspectionReceipt::from_support(query, admission, support_report, authority.generation)
}

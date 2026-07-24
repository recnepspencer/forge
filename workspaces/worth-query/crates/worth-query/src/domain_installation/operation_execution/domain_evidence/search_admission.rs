use std::collections::BTreeSet;

use worth_query_installation::facade::{
    WorthQueryCandidateSearchContract, WorthQueryCandidateSearchPosture,
};

use super::{
    WorthQueryCandidateIncumbentDisposition, WorthQueryCandidateRecord,
    WorthQueryCandidateRecordDisposition, WorthQueryCandidateSearchSummary,
    WorthQueryCandidateTerminationClass, WorthQueryDomainEvidenceAdmissionDenial,
    WorthQueryDomainEvidenceAdmissionDenialKind,
};

pub(super) fn admit_candidate_search(
    contract: &WorthQueryCandidateSearchContract,
    summary: Option<WorthQueryCandidateSearchSummary>,
    records: Option<&[WorthQueryCandidateRecord]>,
) -> Result<Option<WorthQueryCandidateSearchSummary>, WorthQueryDomainEvidenceAdmissionDenial> {
    let Some(universe_family) = contract.universe_family() else {
        if summary.is_none() && records.is_none() {
            return Ok(None);
        }
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::UnexpectedCandidateSearchSummary,
            "candidate-search-not-declared",
        ));
    };
    let summary = summary.ok_or_else(|| {
        denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::MissingCandidateSearchSummary,
            universe_family,
        )
    })?;
    validate_summary(contract, &summary)?;
    if let Some(records) = records {
        validate_records(&summary, records)?;
    }
    Ok(Some(summary))
}

fn validate_summary(
    contract: &WorthQueryCandidateSearchContract,
    summary: &WorthQueryCandidateSearchSummary,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let parts = summary.parts();
    let families_match = contract.universe_family() == Some(parts.universe.family())
        && contract.termination_family() == Some(parts.termination_family.as_str())
        && contract.feasibility_family() == Some(parts.feasibility_family.as_str())
        && contract.comparison_family() == Some(parts.comparison_authority.family())
        && contract.incumbent_family() == Some(parts.incumbent_family.as_str());
    let postures_match = contract.search_posture() == &parts.completeness
        && contract.optimality_posture() == &parts.optimality;
    if !families_match
        || !postures_match
        || !portable(parts.universe.value())
        || !portable(parts.comparison_authority.value())
        || !termination_matches(&parts.completeness, parts.termination)
        || !counts_are_coherent(summary)
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSearchOverclaim,
            parts.universe.value(),
        ));
    }
    Ok(())
}

fn counts_are_coherent(summary: &WorthQueryCandidateSearchSummary) -> bool {
    let parts = summary.parts();
    if parts.rejected_count > parts.considered_count {
        return false;
    }
    match parts.incumbent {
        WorthQueryCandidateIncumbentDisposition::NotApplicable => false,
        WorthQueryCandidateIncumbentDisposition::None => {
            parts.considered_count == 0 || parts.rejected_count <= parts.considered_count
        }
        WorthQueryCandidateIncumbentDisposition::Selected
        | WorthQueryCandidateIncumbentDisposition::Reused => {
            parts.considered_count > 0 && parts.rejected_count < parts.considered_count
        }
        WorthQueryCandidateIncumbentDisposition::RejectedAll => {
            parts.considered_count > 0 && parts.rejected_count == parts.considered_count
        }
    }
}

fn termination_matches(
    search: &WorthQueryCandidateSearchPosture,
    termination: WorthQueryCandidateTerminationClass,
) -> bool {
    use WorthQueryCandidateSearchPosture as Search;
    use WorthQueryCandidateTerminationClass as Termination;
    matches!(
        (search, termination),
        (
            Search::Exhaustive,
            Termination::Completed | Termination::Exhausted
        ) | (
            Search::ProvenTopK { .. },
            Termination::Completed | Termination::BoundReached
        ) | (
            Search::Bounded { .. },
            Termination::Completed | Termination::BoundReached
        ) | (Search::Sampled { .. }, Termination::SampleCompleted)
            | (
                Search::Heuristic,
                Termination::Completed | Termination::HeuristicStop
            )
            | (
                Search::Incomplete,
                Termination::Interrupted | Termination::HeuristicStop | Termination::BoundReached
            )
    )
}

fn validate_records(
    summary: &WorthQueryCandidateSearchSummary,
    records: &[WorthQueryCandidateRecord],
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let parts = summary.parts();
    let identities = records
        .iter()
        .map(WorthQueryCandidateRecord::identity)
        .collect::<BTreeSet<_>>();
    let rejected = records
        .iter()
        .filter(|record| record.disposition() == WorthQueryCandidateRecordDisposition::Rejected)
        .count() as u64;
    let incumbents = records
        .iter()
        .filter(|record| record.disposition() == WorthQueryCandidateRecordDisposition::Incumbent)
        .count();
    let expected_incumbents = usize::from(matches!(
        parts.incumbent,
        WorthQueryCandidateIncumbentDisposition::Selected
            | WorthQueryCandidateIncumbentDisposition::Reused
    ));
    if records.len() as u64 != parts.considered_count
        || identities.len() != records.len()
        || records.iter().any(|record| !portable(record.identity()))
        || rejected != parts.rejected_count
        || incumbents != expected_incumbents
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::CandidateSidecarMismatch,
            parts.universe.value(),
        ));
    }
    Ok(())
}

fn denial(
    kind: WorthQueryDomainEvidenceAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryDomainEvidenceAdmissionDenial {
    WorthQueryDomainEvidenceAdmissionDenial::new(kind, subject)
}

fn portable(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}

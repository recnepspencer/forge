use std::collections::{BTreeMap, BTreeSet};

use worth_query_installation::facade::{
    WorthQueryDecisionCausalParentShape, WorthQueryDecisionRecordContract, WorthQueryDecisionSchema,
};

use super::{
    WorthQueryAdmittedDecisionSummary, WorthQueryDecisionCausalParent, WorthQueryDecisionRecord,
    WorthQueryDecisionSummary, WorthQueryDecisionSummaryCounts,
    WorthQueryDomainEvidenceAdmissionDenial, WorthQueryDomainEvidenceAdmissionDenialKind,
};

pub(super) fn admit_decisions(
    contract: &WorthQueryDecisionRecordContract,
    mut summaries: Vec<WorthQueryDecisionSummary>,
    records: Option<&[WorthQueryDecisionRecord]>,
) -> Result<Vec<WorthQueryAdmittedDecisionSummary>, WorthQueryDomainEvidenceAdmissionDenial> {
    let schemas = contract.schemas();
    if schemas.is_empty() {
        if summaries.is_empty() && records.is_none() {
            return Ok(Vec::new());
        }
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredDecisionSummary,
            "decision-records-not-declared",
        ));
    }
    summaries.sort_by(|left, right| left.kind().cmp(right.kind()));
    if summaries
        .windows(2)
        .any(|pair| pair[0].kind() == pair[1].kind())
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::DuplicateDecisionSummary,
            "duplicate-decision-summary",
        ));
    }
    let mut admitted = Vec::with_capacity(schemas.len());
    for schema in schemas {
        let summary = summaries
            .binary_search_by(|candidate| candidate.kind().cmp(schema.kind()))
            .ok()
            .map(|index| &summaries[index])
            .ok_or_else(|| {
                denial(
                    WorthQueryDomainEvidenceAdmissionDenialKind::MissingDecisionSummary,
                    schema.kind().as_str(),
                )
            })?;
        validate_counts(schema, summary.counts())?;
        admitted.push(WorthQueryAdmittedDecisionSummary::new(
            schema.clone(),
            summary.counts(),
        ));
    }
    if summaries.len() != schemas.len() {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredDecisionSummary,
            "unknown-decision-kind",
        ));
    }
    if let Some(records) = records {
        validate_records(schemas, &admitted, records)?;
    }
    Ok(admitted)
}

fn validate_counts(
    schema: &WorthQueryDecisionSchema,
    counts: WorthQueryDecisionSummaryCounts,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let occurrences = counts.occurrence_count();
    let parent_count_valid = match schema.causal_parent() {
        WorthQueryDecisionCausalParentShape::None => counts.causal_parent_count() == 0,
        WorthQueryDecisionCausalParentShape::OptionalSingle => {
            counts.causal_parent_count() <= occurrences
        }
        WorthQueryDecisionCausalParentShape::RequiredSingle => {
            counts.causal_parent_count() == occurrences
        }
        WorthQueryDecisionCausalParentShape::OrderedMany => {
            occurrences == 0 || counts.causal_parent_count() >= occurrences
        }
    };
    if !parent_count_valid
        || counts.affected_artifact_count() > occurrences
        || counts.recovery_relevant_count() > occurrences
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::InvalidDecisionSummary,
            schema.kind().as_str(),
        ));
    }
    Ok(())
}

fn validate_records(
    schemas: &[WorthQueryDecisionSchema],
    summaries: &[WorthQueryAdmittedDecisionSummary],
    records: &[WorthQueryDecisionRecord],
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let expected_count = summaries
        .iter()
        .map(|summary| summary.counts().occurrence_count())
        .sum::<u64>();
    if records.len() as u64 != expected_count {
        return Err(sidecar_denial("decision-record-count"));
    }
    let schemas = schemas
        .iter()
        .map(|schema| (schema.kind().as_str(), schema))
        .collect::<BTreeMap<_, _>>();
    let mut derived = BTreeMap::<&str, DerivedCounts>::new();
    for record in records {
        let schema = schemas
            .get(record.kind().as_str())
            .copied()
            .ok_or_else(|| sidecar_denial(record.kind().as_str()))?;
        if record.reason_family() != schema.reason_family().as_str()
            || record.artifact_key_family() != schema.affected_artifact_key_family().as_str()
            || record.payload_version() != schema.payload_version().get()
            || !portable(record.artifact_key())
            || !parent_matches(schema.causal_parent(), record.causal_parent())
        {
            return Err(sidecar_denial(record.kind().as_str()));
        }
        derived
            .entry(record.kind().as_str())
            .or_default()
            .retain(record);
    }
    for summary in summaries {
        let actual = derived
            .get(summary.schema().kind().as_str())
            .cloned()
            .unwrap_or_default();
        if actual.counts() != summary.counts() {
            return Err(sidecar_denial(summary.schema().kind().as_str()));
        }
    }
    Ok(())
}

#[derive(Clone, Default)]
struct DerivedCounts {
    occurrences: u64,
    parents: u64,
    artifacts: BTreeSet<String>,
    recovery: u64,
}

impl DerivedCounts {
    fn retain(&mut self, record: &WorthQueryDecisionRecord) {
        self.occurrences += 1;
        self.parents += match record.causal_parent() {
            WorthQueryDecisionCausalParent::None => 0,
            WorthQueryDecisionCausalParent::Single(_) => 1,
            WorthQueryDecisionCausalParent::Ordered(parents) => parents.len() as u64,
        };
        self.artifacts.insert(record.artifact_key().to_owned());
        self.recovery += u64::from(record.recovery_relevant());
    }

    fn counts(&self) -> WorthQueryDecisionSummaryCounts {
        WorthQueryDecisionSummaryCounts::new(
            self.occurrences,
            self.parents,
            self.artifacts.len() as u64,
            self.recovery,
        )
    }
}

fn parent_matches(
    shape: WorthQueryDecisionCausalParentShape,
    parent: &WorthQueryDecisionCausalParent,
) -> bool {
    matches!(
        (shape, parent),
        (
            WorthQueryDecisionCausalParentShape::None,
            WorthQueryDecisionCausalParent::None
        ) | (
            WorthQueryDecisionCausalParentShape::OptionalSingle,
            WorthQueryDecisionCausalParent::None | WorthQueryDecisionCausalParent::Single(_)
        ) | (
            WorthQueryDecisionCausalParentShape::RequiredSingle,
            WorthQueryDecisionCausalParent::Single(_)
        ) | (
            WorthQueryDecisionCausalParentShape::OrderedMany,
            WorthQueryDecisionCausalParent::Ordered(_)
        )
    ) && match parent {
        WorthQueryDecisionCausalParent::None => true,
        WorthQueryDecisionCausalParent::Single(identity) => portable(identity),
        WorthQueryDecisionCausalParent::Ordered(identities) => {
            !identities.is_empty() && identities.iter().all(|identity| portable(identity))
        }
    }
}

fn sidecar_denial(subject: impl Into<String>) -> WorthQueryDomainEvidenceAdmissionDenial {
    denial(
        WorthQueryDomainEvidenceAdmissionDenialKind::DecisionSidecarMismatch,
        subject,
    )
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

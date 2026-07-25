use std::collections::BTreeSet;

use worth_query_installation::facade::{
    WorthQuerySourceOutputCorrespondence, WorthQueryTransformationEvidenceContract,
};

use super::{
    WorthQueryDomainEvidenceAdmissionDenial, WorthQueryDomainEvidenceAdmissionDenialKind,
    WorthQueryTransformationRecord, WorthQueryTransformationSummary,
};

pub(super) fn admit_transformation(
    contract: &WorthQueryTransformationEvidenceContract,
    summary: Option<WorthQueryTransformationSummary>,
    records: Option<&[WorthQueryTransformationRecord]>,
    output_occurrence_identity: &str,
) -> Result<Option<WorthQueryTransformationSummary>, WorthQueryDomainEvidenceAdmissionDenial> {
    let WorthQueryTransformationEvidenceContract::Declared {
        source_occurrence,
        transformation,
        outcome,
    } = contract
    else {
        if summary.is_none() && records.is_none() {
            return Ok(None);
        }
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::UnexpectedTransformationSummary,
            "transformation-not-declared",
        ));
    };
    let summary = summary.ok_or_else(|| {
        denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::MissingTransformationSummary,
            transformation.family(),
        )
    })?;
    let parts = summary.parts();
    if parts.source_occurrence.family() != source_occurrence.identity_family()
        || !portable(parts.source_occurrence.value())
        || parts.output_occurrence_identity != output_occurrence_identity
        || parts.transformation_family != transformation.family()
        || parts.transformation_version != transformation.version()
        || parts.correspondence != outcome.correspondence()
        || parts.disposition != outcome.disposition()
        || parts.error != outcome.error()
        || parts.loss != outcome.loss()
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSummaryMismatch,
            transformation.family(),
        ));
    }
    if let Some(records) = records {
        validate_records(&summary, records)?;
    }
    Ok(Some(summary))
}

fn validate_records(
    summary: &WorthQueryTransformationSummary,
    records: &[WorthQueryTransformationRecord],
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let parts = summary.parts();
    let sources = records
        .iter()
        .map(WorthQueryTransformationRecord::source_occurrence_identity)
        .collect::<BTreeSet<_>>();
    let outputs = records
        .iter()
        .flat_map(WorthQueryTransformationRecord::output_occurrence_identities)
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let valid = !records.is_empty()
        && records.iter().all(|record| {
            portable(record.source_occurrence_identity())
                && !record.output_occurrence_identities().is_empty()
                && record
                    .output_occurrence_identities()
                    .iter()
                    .all(|identity| portable(identity))
                && record.disposition() == parts.disposition
                && record.error() == parts.error
        })
        && sources.contains(parts.source_occurrence.value())
        && outputs.contains(parts.output_occurrence_identity.as_str())
        && correspondence_matches(parts.correspondence, sources.len(), outputs.len());
    valid.then_some(()).ok_or_else(|| {
        denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::TransformationSidecarMismatch,
            &parts.transformation_family,
        )
    })
}

fn correspondence_matches(
    correspondence: WorthQuerySourceOutputCorrespondence,
    source_count: usize,
    output_count: usize,
) -> bool {
    match correspondence {
        WorthQuerySourceOutputCorrespondence::OneToOne => source_count == 1 && output_count == 1,
        WorthQuerySourceOutputCorrespondence::OneToMany => source_count == 1 && output_count > 1,
        WorthQuerySourceOutputCorrespondence::ManyToOne => source_count > 1 && output_count == 1,
        WorthQuerySourceOutputCorrespondence::ManyToMany => source_count > 1 && output_count > 1,
        WorthQuerySourceOutputCorrespondence::Partial => true,
    }
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

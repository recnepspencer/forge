use std::collections::BTreeSet;

use super::journey_audit::workspace_consumer_journey_audit;
use super::{
    audit_consumer_journey_sources, worth_query_consumer_journey_rows,
    WorthQueryConsumerJourneyFindingKind, WorthQueryConsumerJourneySource,
    WorthQueryDeclarativeCapabilityFamily,
};

#[test]
fn required_query_and_reference_consumer_journeys_are_source_backed() {
    let audit = workspace_consumer_journey_audit();

    assert!(
        audit.is_complete(),
        "consumer journey findings: {:?}",
        audit.findings()
    );
    assert_eq!(
        audit.classified_journey_count(),
        worth_query_consumer_journey_rows().len()
    );
}

#[test]
fn every_journey_carries_the_complete_refactor_boundary() {
    let rows = worth_query_consumer_journey_rows();
    let ids = rows
        .iter()
        .map(|row| row.journey_id())
        .collect::<BTreeSet<_>>();
    let families = rows
        .iter()
        .map(|row| row.capability_family())
        .collect::<BTreeSet<_>>();

    assert_eq!(ids.len(), rows.len());
    for required in [
        WorthQueryDeclarativeCapabilityFamily::Read,
        WorthQueryDeclarativeCapabilityFamily::Aggregate,
        WorthQueryDeclarativeCapabilityFamily::Live,
        WorthQueryDeclarativeCapabilityFamily::Historical,
        WorthQueryDeclarativeCapabilityFamily::Comparison,
        WorthQueryDeclarativeCapabilityFamily::Preview,
        WorthQueryDeclarativeCapabilityFamily::Mutation,
        WorthQueryDeclarativeCapabilityFamily::Workflow,
        WorthQueryDeclarativeCapabilityFamily::Inspection,
        WorthQueryDeclarativeCapabilityFamily::DomainExtension,
    ] {
        assert!(
            families.contains(&required),
            "missing journey family {required:?}"
        );
    }
    for row in rows {
        for field in [
            row.consumer(),
            row.source_path(),
            row.source_probe(),
            row.declared_intent(),
            row.required_context(),
            row.admitted_capability(),
            row.query_owned_phase_chain(),
            row.result(),
            row.receipts(),
            row.diagnostics(),
            row.cost_counters(),
            row.local_ceremony(),
            row.replacement(),
        ] {
            assert!(
                !field.trim().is_empty(),
                "incomplete journey {}",
                row.journey_id()
            );
        }
    }
}

#[test]
fn missing_and_ambiguous_source_evidence_fail_closed() {
    let row = &worth_query_consumer_journey_rows()[0];
    let wrong_source = WorthQueryConsumerJourneySource::new(row.source_path(), "not the probe");
    let wrong_audit = audit_consumer_journey_sources(&[wrong_source]);
    assert!(wrong_audit.findings().iter().any(|finding| {
        finding.journey_id() == row.journey_id()
            && finding.kind() == WorthQueryConsumerJourneyFindingKind::MissingSourceProbe
    }));

    let repeated = format!("{}\n{}", row.source_probe(), row.source_probe());
    let repeated_source = WorthQueryConsumerJourneySource::new(row.source_path(), &repeated);
    let repeated_audit = audit_consumer_journey_sources(&[repeated_source]);
    assert!(repeated_audit.findings().iter().any(|finding| {
        finding.journey_id() == row.journey_id()
            && finding.kind() == WorthQueryConsumerJourneyFindingKind::AmbiguousSourceProbe
    }));
}

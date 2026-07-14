use std::collections::BTreeSet;

use super::{
    audit_reference_consumer_adoption_sources, workspace_reference_consumer_adoption_audit,
    worth_query_reference_consumer_adoption_rows, worth_query_reference_consumer_deleted_residue,
    WorthQueryReferenceConsumerAdoptionFindingKind, WorthQueryReferenceConsumerResidueKind,
    WorthQueryReferenceConsumerSource,
};

#[test]
fn reference_consumers_have_current_journeys_and_deleted_residue() {
    let audit = workspace_reference_consumer_adoption_audit();
    assert!(audit.is_complete(), "findings: {:?}", audit.findings());
    assert_eq!(
        audit.adopted_consumer_count(),
        worth_query_reference_consumer_adoption_rows().len()
    );
    assert_eq!(
        audit.deleted_residue_count(),
        worth_query_reference_consumer_deleted_residue().len()
    );
    assert!(audit.after_ceremony_count() < audit.before_ceremony_count());
}

#[test]
fn dx_transcripts_remove_manual_transitions_and_local_authority_decisions() {
    for row in worth_query_reference_consumer_adoption_rows() {
        assert!(row.after().ceremony_count() < row.before().ceremony_count());
        assert_eq!(row.after().manual_transition_count(), 0);
        assert_eq!(row.after().backend_decision_count(), 0);
        assert_eq!(row.after().local_authority_decision_count(), 0);
    }
}

#[test]
fn deletion_manifest_covers_every_required_residue_kind() {
    let kinds = worth_query_reference_consumer_deleted_residue()
        .iter()
        .map(|row| row.kind())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        kinds,
        BTreeSet::from([
            WorthQueryReferenceConsumerResidueKind::LocalType,
            WorthQueryReferenceConsumerResidueKind::LocalHelper,
            WorthQueryReferenceConsumerResidueKind::LocalTransition,
            WorthQueryReferenceConsumerResidueKind::DeepImport,
            WorthQueryReferenceConsumerResidueKind::BackendDecision,
        ])
    );
}

#[test]
fn each_deleted_residue_category_has_exact_sabotage_detection() {
    for kind in [
        WorthQueryReferenceConsumerResidueKind::LocalType,
        WorthQueryReferenceConsumerResidueKind::LocalHelper,
        WorthQueryReferenceConsumerResidueKind::LocalTransition,
        WorthQueryReferenceConsumerResidueKind::DeepImport,
        WorthQueryReferenceConsumerResidueKind::BackendDecision,
    ] {
        let residue = worth_query_reference_consumer_deleted_residue()
            .iter()
            .find(|row| row.kind() == kind)
            .expect("each residue kind must have a registry row");
        let mut source_text = std::collections::BTreeMap::new();
        for row in worth_query_reference_consumer_adoption_rows() {
            source_text.insert(row.source_path(), row.current_probe());
        }
        for row in worth_query_reference_consumer_deleted_residue() {
            source_text.entry(row.source_path()).or_insert("");
        }
        source_text.insert(residue.source_path(), residue.probe());
        let sources = source_text
            .iter()
            .map(|(path, text)| WorthQueryReferenceConsumerSource::new(path, text))
            .collect::<Vec<_>>();
        let audit = audit_reference_consumer_adoption_sources(&sources);
        let findings = audit
            .findings()
            .iter()
            .filter(|finding| {
                finding.kind()
                    == WorthQueryReferenceConsumerAdoptionFindingKind::DeletedResiduePresent
            })
            .collect::<Vec<_>>();
        assert_eq!(findings.len(), 1, "kind: {}", kind.as_str());
        assert_eq!(findings[0].residue_kind(), Some(kind));
        assert_eq!(findings[0].probe(), residue.probe());
    }
}

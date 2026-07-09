use crate::grouped_authoring::{
    worth_query_grouped_declaration_checked_on_handle,
    worth_query_grouped_orchestration_checked_on_handle, WorthQueryGroupedDeclarationChecked,
    WorthQueryGroupedDeclarationInput, WorthQueryGroupedOrchestrationChecked,
};
use crate::platform_entry_closeout::alignment::{
    docs_coverage_alignment_audit, docs_coverage_alignment_audit_from_audit,
};
use crate::public_doc_coverage::{
    WorthQueryPublicDocCoverageAudit, WorthQueryPublicDocCoverageInventory,
    WorthQueryPublicGoldenTranscriptKind,
};

use super::support::{admitted_handle, CloseoutInput};

#[test]
fn grouped_member_stop_remains_distinct_from_group_alignment_stop() {
    let primary = admitted_handle("primary");
    let other = admitted_handle("other");
    let declaration = match worth_query_grouped_declaration_checked_on_handle(
        &primary,
        WorthQueryGroupedDeclarationInput::local_neighborhood(CloseoutInput::new("edge-a"))
            .with_member(CloseoutInput::new("edge-b")),
    ) {
        WorthQueryGroupedDeclarationChecked::Bound(value) => value,
        WorthQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit before orchestration")
        }
    };

    let member_stop =
        worth_query_grouped_orchestration_checked_on_handle(&primary, declaration.clone());
    let alignment_stop = worth_query_grouped_orchestration_checked_on_handle(&other, declaration);

    assert!(matches!(
        member_stop,
        WorthQueryGroupedOrchestrationChecked::MemberStopped(_)
    ));
    assert!(matches!(
        alignment_stop,
        WorthQueryGroupedOrchestrationChecked::WrongWorld(_)
    ));
}

#[test]
fn coverage_boundary_readout_remains_distinct_from_surface_coverage_readout() {
    let rows = crate::public_doc_coverage::worth_query_public_doc_coverage_golden_transcripts();
    let boundary = rows
        .iter()
        .find(|row| row.kind() == WorthQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout)
        .expect("coverage boundary readout should exist");

    assert_eq!(boundary.label(), "public_doc_coverage_surface_readout");
    assert!(boundary.journey().is_none());
    assert!(rows.iter().any(|row| {
        row.kind() == WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage
            && row.journey().is_some()
    }));
}

#[test]
fn docs_coverage_gap_remains_distinct_from_closed_alignment() {
    let current = WorthQueryPublicDocCoverageInventory::current();
    let broken = WorthQueryPublicDocCoverageInventory::new(
        current.source_inventory_digest().to_string(),
        current.rows()[1..].to_vec(),
    );
    let broken_audit = WorthQueryPublicDocCoverageAudit::from_inventory(&broken);
    let broken_alignment = docs_coverage_alignment_audit_from_audit(&broken_audit);

    assert!(!broken_alignment.is_aligned());
    assert!(
        broken_alignment
            .gaps()
            .iter()
            .any(|gap| gap.starts_with("undocumented:")),
        "expected a documentation gap, got {:?}",
        broken_alignment.gaps()
    );
    assert!(docs_coverage_alignment_audit().is_aligned());
}

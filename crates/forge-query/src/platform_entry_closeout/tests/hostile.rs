use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput, ForgeQueryGroupedOrchestrationChecked,
};
use crate::platform_entry_closeout::alignment::{
    docs_coverage_alignment_audit, docs_coverage_alignment_audit_from_audit,
};
use crate::public_doc_coverage::{
    ForgeQueryPublicDocCoverageAudit, ForgeQueryPublicDocCoverageInventory,
    ForgeQueryPublicGoldenTranscriptKind,
};

use super::support::{admitted_handle, CloseoutInput};

#[test]
fn grouped_member_stop_remains_distinct_from_group_alignment_stop() {
    let primary = admitted_handle("primary");
    let other = admitted_handle("other");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &primary,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(CloseoutInput::new("edge-a"))
            .with_member(CloseoutInput::new("edge-b")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit before orchestration")
        }
    };

    let member_stop =
        forge_query_grouped_orchestration_checked_on_handle(&primary, declaration.clone());
    let alignment_stop = forge_query_grouped_orchestration_checked_on_handle(&other, declaration);

    assert!(matches!(
        member_stop,
        ForgeQueryGroupedOrchestrationChecked::MemberStopped(_)
    ));
    assert!(matches!(
        alignment_stop,
        ForgeQueryGroupedOrchestrationChecked::WrongWorld(_)
    ));
}

#[test]
fn coverage_boundary_readout_remains_distinct_from_surface_coverage_readout() {
    let rows = crate::public_doc_coverage::forge_query_public_doc_coverage_golden_transcripts();
    let boundary = rows
        .iter()
        .find(|row| row.kind() == ForgeQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout)
        .expect("coverage boundary readout should exist");

    assert_eq!(boundary.label(), "public_doc_coverage_surface_readout");
    assert!(boundary.journey().is_none());
    assert!(rows.iter().any(|row| {
        row.kind() == ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage
            && row.journey().is_some()
    }));
}

#[test]
fn docs_coverage_gap_remains_distinct_from_closed_alignment() {
    let current = ForgeQueryPublicDocCoverageInventory::current();
    let broken = ForgeQueryPublicDocCoverageInventory::new(
        current.source_inventory_digest().to_string(),
        current.rows()[1..].to_vec(),
    );
    let broken_audit = ForgeQueryPublicDocCoverageAudit::from_inventory(&broken);
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

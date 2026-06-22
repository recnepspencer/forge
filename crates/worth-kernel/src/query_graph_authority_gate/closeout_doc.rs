use super::closeout_counters::WorthGraphAuthorityCloseoutCounters;
use super::closeout_report::WorthGraphAuthorityCloseoutViolation;
use super::closeout_types::WorthGraphAuthorityDeletionClassCloseoutEvidence;

pub(crate) const CLOSEOUT_DOC: &str =
    include_str!("../../../../_docs/worth/worth-query-graph-authority-hardening-closeout.md");

pub(crate) fn validate_closeout_doc(
    counters: &WorthGraphAuthorityCloseoutCounters,
    deletion_class_evidence: &[WorthGraphAuthorityDeletionClassCloseoutEvidence],
    closeout_doc: &str,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    assert_doc_line(
        closeout_doc,
        "Audited sources covered",
        counters.audited_sources_covered(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Inventory matrix rows",
        counters.inventory_matrix_rows(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deletion target classes found",
        counters.deletion_target_classes(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deleted surfaces",
        counters.deleted_surfaces(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Collapsed canonical Query proof rows",
        counters.collapsed_canonical_query_proofs(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Collapsed split ledger receipt rows",
        counters.collapsed_split_ledger_receipts(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Collapsed loop ledger receipt rows",
        counters.collapsed_loop_ledger_receipts(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Certification-only boundaries",
        counters.certification_only_boundaries(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Explicit residue rows",
        counters.explicit_residue_rows(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Query capability gaps",
        counters.query_capability_gaps(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Lower-authority promotion fixtures",
        counters.lower_authority_rejection_fixtures(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Rejected bypass classes",
        counters.rejected_bypass_classes(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Public facade proof surfaces",
        counters.public_facade_proofs(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deletion line/removal classes",
        counters.deletion_line_removal_classes(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deletion removal ledger rows",
        counters.deletion_removal_ledger_rows(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deletion affected source files",
        counters.deletion_affected_source_files(),
    )?;
    assert_doc_line(
        closeout_doc,
        "Deletion affected source lines",
        counters.deletion_affected_source_lines(),
    )?;
    for evidence in deletion_class_evidence {
        assert_deletion_class_doc_line(closeout_doc, evidence)?;
    }
    if !closeout_doc.contains("Zero silent covered-lane bypass: yes") {
        return Err(
            WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim(
                "zero silent covered-lane bypass",
            ),
        );
    }
    if !closeout_doc.contains("Broad 7.5 overlap-region extraction remains blocked") {
        return Err(
            WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim("broad 7.5 block"),
        );
    }
    Ok(())
}

fn assert_deletion_class_doc_line(
    closeout_doc: &str,
    evidence: &WorthGraphAuthorityDeletionClassCloseoutEvidence,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let expected = format!(
        "{:?}: removal rows {}, source files {}, source lines {}",
        evidence.deletion_target(),
        evidence.removal_ledger_rows(),
        evidence.affected_source_files(),
        evidence.affected_source_lines()
    );
    if closeout_doc.contains(&expected) {
        Ok(())
    } else {
        Err(
            WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim(
                "deletion class line/removal count",
            ),
        )
    }
}

fn assert_doc_line(
    closeout_doc: &str,
    label: &'static str,
    value: usize,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let expected = format!("{label}: {value}");
    if closeout_doc.contains(&expected) {
        Ok(())
    } else {
        Err(WorthGraphAuthorityCloseoutViolation::CloseoutDocMissingClaim(label))
    }
}

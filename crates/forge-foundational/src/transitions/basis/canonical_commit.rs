use crate::canonicalization::CanonicalBasisEntry;
use crate::transitions::{
    FoundationalAuthorityTransitionClass, FoundationalCommitReceiptArtifact,
    FoundationalCommittedAuthorityArtifact, FoundationalNoOpCause,
    FoundationalTransitionIssuanceCause,
};

use super::canonical_merge::{append_strategy_and_basis_entries, strategy_ownership_token};
use super::canonical_shared::{bool_entry, text_entry, u64_entry};

pub(super) fn committed_authority_entries<T>(
    committed: &FoundationalCommittedAuthorityArtifact<T>,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![
        text_entry(
            "commit.transition_class",
            transition_class_token(committed.transition_class()),
        ),
        text_entry("commit.source_branch", committed.source_branch().as_str()),
        text_entry("commit.target_branch", committed.target_branch().as_str()),
        u64_entry(
            "commit.parent_basis",
            committed.parent_basis().basis_id().get(),
        ),
    ];
    entries.extend(
        committed
            .parentage()
            .parents()
            .iter()
            .enumerate()
            .map(|(index, basis)| {
                u64_entry(&format!("commit.parentage.{index}"), basis.basis_id().get())
            }),
    );
    entries.push(bool_entry(
        "commit.has_merge_ancestry_basis",
        committed.merge_ancestry_basis().is_some(),
    ));
    if let Some(basis) = committed.merge_ancestry_basis() {
        entries.push(u64_entry(
            "commit.merge_ancestry_basis",
            basis.basis_id().get(),
        ));
    }
    entries.push(bool_entry(
        "commit.has_noop_cause",
        committed.no_op_cause().is_some(),
    ));
    if let Some(cause) = committed.no_op_cause() {
        entries.push(text_entry("commit.noop_cause", noop_cause_token(cause)));
    }
    append_strategy_and_basis_entries("commit", committed.merge_verdict(), &mut entries);
    entries.extend(
        committed
            .committed_delta_summary()
            .loci()
            .iter()
            .enumerate()
            .flat_map(|(index, locus)| {
                [
                    text_entry(&format!("commit.delta.{index}.category"), locus.category()),
                    text_entry(&format!("commit.delta.{index}.detail"), locus.detail()),
                ]
            }),
    );
    entries
}

pub(super) fn receipt_entries(
    receipt: &FoundationalCommitReceiptArtifact,
) -> Vec<CanonicalBasisEntry> {
    let provenance = &receipt.transition_provenance_rows()[0];
    let mut entries = vec![
        u64_entry("receipt.commit_id", receipt.commit_id().handle().get()),
        text_entry("receipt.branch_id", receipt.branch_id().as_str()),
        u64_entry(
            "receipt.parent_basis",
            receipt.parent_basis().basis_id().get(),
        ),
        text_entry(
            "receipt.transition_class",
            transition_class_token(receipt.transition_class()),
        ),
        u64_entry(
            "receipt.receipt_identity",
            receipt.receipt_identity().handle().get(),
        ),
        u64_entry(
            "receipt.strategy.id",
            receipt.strategy_identity().id().handle().get(),
        ),
        text_entry(
            "receipt.strategy.family",
            receipt.strategy_identity().family().as_str(),
        ),
        text_entry(
            "receipt.strategy.semantic_name",
            receipt.strategy_identity().semantic_name().as_str(),
        ),
        text_entry(
            "receipt.strategy.version",
            receipt.strategy_identity().version().as_str(),
        ),
        text_entry(
            "receipt.strategy.ownership",
            strategy_ownership_token(receipt.strategy_identity().ownership()),
        ),
        text_entry(
            "receipt.strategy.descriptor_digest",
            &digest_text(receipt.strategy_descriptor_digest().digest_id().bytes()),
        ),
        u64_entry(
            "receipt.transition_basis_identity",
            receipt.transition_basis_identity().basis_id().get(),
        ),
        text_entry(
            "receipt.merge_basis.family",
            provenance.merge_basis().family().as_str(),
        ),
        text_entry(
            "receipt.merge_basis.version",
            provenance.merge_basis().version().as_str(),
        ),
    ];
    entries.extend(
        receipt
            .parentage()
            .parents()
            .iter()
            .enumerate()
            .map(|(index, basis)| {
                u64_entry(
                    &format!("receipt.parentage.{index}"),
                    basis.basis_id().get(),
                )
            }),
    );
    entries.push(bool_entry(
        "receipt.has_noop_cause",
        receipt.no_op_cause().is_some(),
    ));
    if let Some(cause) = receipt.no_op_cause() {
        entries.push(text_entry("receipt.noop_cause", noop_cause_token(cause)));
    }
    entries.push(bool_entry(
        "receipt.has_merge_ancestry_basis",
        receipt.merge_ancestry_basis().is_some(),
    ));
    if let Some(basis) = receipt.merge_ancestry_basis() {
        entries.push(u64_entry(
            "receipt.merge_ancestry_basis",
            basis.basis_id().get(),
        ));
    }
    entries.push(text_entry(
        "receipt.issuance_cause",
        issuance_cause_token(
            provenance
                .issuance_cause()
                .expect("receipt provenance rows always carry issuance cause"),
        ),
    ));
    entries.extend(receipt.delta_evidence().loci().iter().enumerate().flat_map(
        |(index, locus)| {
            [
                text_entry(&format!("receipt.delta.{index}.category"), locus.category()),
                text_entry(&format!("receipt.delta.{index}.detail"), locus.detail()),
            ]
        },
    ));
    entries
}

fn transition_class_token(class: FoundationalAuthorityTransitionClass) -> &'static str {
    match class {
        FoundationalAuthorityTransitionClass::NoOp => "no-op",
        FoundationalAuthorityTransitionClass::Commit => "commit",
        FoundationalAuthorityTransitionClass::MetadataOnlyCommit => "metadata-only-commit",
        FoundationalAuthorityTransitionClass::PromotionCommit => "promotion-commit",
        FoundationalAuthorityTransitionClass::ReplayRevalidatedCommit => {
            "replay-revalidated-commit"
        }
    }
}

fn noop_cause_token(cause: FoundationalNoOpCause) -> &'static str {
    match cause {
        FoundationalNoOpCause::AlreadyConverged => "already-converged",
        FoundationalNoOpCause::BasisEquivalent => "basis-equivalent",
        FoundationalNoOpCause::StrategySuppressed => "strategy-suppressed",
        FoundationalNoOpCause::ChangeDenied => "change-denied",
        FoundationalNoOpCause::ReplayEquivalent => "replay-equivalent",
    }
}

fn issuance_cause_token(cause: FoundationalTransitionIssuanceCause) -> &'static str {
    match cause {
        FoundationalTransitionIssuanceCause::CommitAttested => "commit-attested",
        FoundationalTransitionIssuanceCause::MetadataOnlyCommitAttested => {
            "metadata-only-commit-attested"
        }
        FoundationalTransitionIssuanceCause::PromotionCommitAttested => "promotion-commit-attested",
        FoundationalTransitionIssuanceCause::ReplayRevalidatedCommitAttested => {
            "replay-revalidated-commit-attested"
        }
        FoundationalTransitionIssuanceCause::NoOpAttested => "no-op-attested",
    }
}

fn digest_text(bytes: &[u8; 32]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

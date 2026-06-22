use crate::workload_platform::planar_boolean_edge_splitting::{
    recover_source_edge_carriers_for_tests, source_carriers_for_tests,
    split_event_ledger_for_tests, split_pair_receipt_for_tests,
    split_subject_with_ledger_for_tests,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    duplicate_overlap_chain_identity_set, empty_recovered_source_carriers_for,
    foreign_fragment_membership_set, missing_first_fragment_from_set,
    missing_first_overlap_chain_from_set, overlap_chain_set_with_missing_member_membership,
    overlap_chain_set_with_topology_truth, prepared_loop_reconstruction_subject,
    prepared_loop_reconstruction_subject_with_tag, uncertified_coordinate_only_fragment_set,
    with_duplicate_first_fragment, LoopFixtureEntryOrder,
};

use super::super::{
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSourceProvenanceDenialKind,
    PlanarBooleanLoopSourceProvenanceRecoveryInput,
};

#[test]
fn loop_source_provenance_rejects_foreign_recovered_source_carrier_lineage() {
    let canonical = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let foreign_carriers = source_carriers_for_tests();
    let foreign_segment_pairs = split_pair_receipt_for_tests(&foreign_carriers);
    let foreign_ledger = split_event_ledger_for_tests(
        foreign_segment_pairs.segment_pair_enumeration_identity(),
        foreign_carriers,
        Vec::new(),
        "foreign-event-ledger",
    );
    let foreign_subject = split_subject_with_ledger_for_tests(foreign_ledger);
    let foreign_recovered_carriers = recover_source_edge_carriers_for_tests(&foreign_subject);
    let request = canonical.admit_loop_request();

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            canonical.split_ledger_result.ledger(),
            canonical.split_ledger_result.receipt(),
            &foreign_recovered_carriers,
            &canonical.fragments,
            &canonical.overlap_chains,
        ),
    )
    .expect_err("foreign recovered carriers must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignSourceCarrierLineage
    );
}

#[test]
fn loop_source_provenance_rejects_foreign_split_ledger_receipt_lineage() {
    let canonical = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let foreign = prepared_loop_reconstruction_subject_with_tag(
        LoopFixtureEntryOrder::Canonical,
        "phase-3-foreign-receipt",
    );
    let request = canonical.admit_loop_request();

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            canonical.split_ledger_result.ledger(),
            foreign.split_ledger_result.receipt(),
            &canonical.recovered_source_carriers,
            &canonical.fragments,
            &canonical.overlap_chains,
        ),
    )
    .expect_err("foreign split ledger receipt lineage must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignSplitLedgerReceipt
    );
}

#[test]
fn loop_source_provenance_rejects_missing_recovered_source_carriers() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let empty_carriers = empty_recovered_source_carriers_for(&subject.recovered_source_carriers);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &empty_carriers,
            &subject.fragments,
            &subject.overlap_chains,
        ),
    )
    .expect_err("missing recovered carriers must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::MissingRecoveredSourceCarrier
    );
}

#[test]
fn loop_source_provenance_rejects_foreign_overlap_chain_lineage_before_recovery() {
    let canonical = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let foreign = prepared_loop_reconstruction_subject_with_tag(
        LoopFixtureEntryOrder::Canonical,
        "phase-3-foreign",
    );
    let request = canonical.admit_loop_request();

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            canonical.split_ledger_result.ledger(),
            canonical.split_ledger_result.receipt(),
            &canonical.recovered_source_carriers,
            &canonical.fragments,
            &foreign.overlap_chains,
        ),
    )
    .expect_err("foreign overlap chains must deny before recovery proceeds");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignOverlapChainLineage
    );
}

#[test]
fn loop_source_provenance_rejects_missing_fragment_membership_before_recovery_continues() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let missing_fragment_set = missing_first_fragment_from_set(&subject.fragments);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &missing_fragment_set,
            &subject.overlap_chains,
        ),
    )
    .expect_err("missing ledger fragment membership must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::MissingLedgerFragment
    );
}

#[test]
fn loop_source_provenance_rejects_foreign_fragment_membership_lineage() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let foreign_membership = foreign_fragment_membership_set(&subject.fragments);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &foreign_membership,
            &subject.overlap_chains,
        ),
    )
    .expect_err("foreign fragment membership lineage must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignFragmentLineage
    );
}

#[test]
fn loop_source_provenance_rejects_uncertified_coordinate_only_fragment_set() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let coordinate_only = uncertified_coordinate_only_fragment_set(&subject.fragments);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &coordinate_only,
            &subject.overlap_chains,
        ),
    )
    .expect_err("uncertified coordinate-only fragment set must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignFragmentLineage
    );
}

#[test]
fn loop_source_provenance_rejects_duplicate_fragment_identity_before_lookup() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let duplicate_fragments = with_duplicate_first_fragment(&subject.fragments);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &duplicate_fragments,
            &subject.overlap_chains,
        ),
    )
    .expect_err("duplicate fragment identities must deny before lookup continues");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::DuplicateFragmentIdentity
    );
}

#[test]
fn loop_source_provenance_rejects_missing_overlap_chain_before_recovery_continues() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let missing_overlap_chains = missing_first_overlap_chain_from_set(&subject.overlap_chains);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &missing_overlap_chains,
        ),
    )
    .expect_err("missing overlap chain must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::MissingLedgerOverlapChain
    );
}

#[test]
fn loop_source_provenance_rejects_overlap_chain_member_without_fragment_membership() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let malformed_overlap_chains =
        overlap_chain_set_with_missing_member_membership(&subject.overlap_chains);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &malformed_overlap_chains,
        ),
    )
    .expect_err("overlap chain members without fragment membership must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::OverlapChainMemberMissingFragmentMembership
    );
}

#[test]
fn loop_source_provenance_rejects_duplicate_overlap_chain_identity_before_lookup() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let duplicate_overlap_chains = duplicate_overlap_chain_identity_set(&subject.overlap_chains);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &duplicate_overlap_chains,
        ),
    )
    .expect_err("duplicate overlap chain identities must deny before lookup continues");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::DuplicateOverlapChainIdentity
    );
}

#[test]
fn loop_source_provenance_rejects_overlap_chain_topology_truth_products() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();
    let topology_truth_overlap_chains =
        overlap_chain_set_with_topology_truth(&subject.overlap_chains);

    let denial = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &topology_truth_overlap_chains,
        ),
    )
    .expect_err("topology truth overlap chains must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopSourceProvenanceDenialKind::ForeignOverlapChainLineage
    );
}

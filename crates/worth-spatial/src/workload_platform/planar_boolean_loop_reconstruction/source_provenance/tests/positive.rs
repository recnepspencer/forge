use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_reconstruction_subject, LoopFixtureEntryOrder,
};

use super::super::{
    PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSourceProvenanceRecoveryInput,
};

#[test]
fn loop_source_provenance_recovery_preserves_carrier_fragment_and_overlap_lineage() {
    let subject = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let request = subject.admit_loop_request();

    let bundle = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &subject.overlap_chains,
        ),
    )
    .expect("source provenance recovery should admit");

    assert_eq!(bundle.request_identity(), request.request_identity());
    assert_eq!(
        bundle.split_ledger_receipt_identity(),
        subject.split_ledger_result.receipt().receipt_identity()
    );

    let expected_carriers = subject
        .split_ledger_result
        .ledger()
        .chains()
        .iter()
        .map(|chain| chain.carrier_identity().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        bundle.source_loop_carriers().rows().len(),
        expected_carriers.len()
    );

    for chain in subject.split_ledger_result.ledger().chains() {
        let source_loop_carrier = bundle
            .source_loop_carriers()
            .carrier_for_identity(chain.carrier_identity())
            .expect("every ledger chain carrier should recover a source-loop carrier");
        let recovered_carrier = subject
            .recovered_source_carriers
            .carrier_for_identity(chain.carrier_identity())
            .expect("recovered carrier set should contain every ledger chain carrier");
        assert_eq!(
            source_loop_carrier.recovered_carrier_identity(),
            recovered_carrier.recovered_carrier_identity()
        );
        assert_eq!(
            source_loop_carrier.source_face_identity(),
            recovered_carrier.source_face_identity()
        );
        assert_eq!(
            source_loop_carrier.source_loop_identity(),
            recovered_carrier.source_loop_identity()
        );
        assert_eq!(
            source_loop_carrier.source_edge_identity(),
            recovered_carrier.source_edge_identity()
        );
        assert_eq!(
            source_loop_carrier.loop_role(),
            recovered_carrier.loop_role()
        );

        for fragment_identity in chain.fragment_identities() {
            let membership = bundle
                .fragment_membership_map()
                .membership_for_fragment_identity(fragment_identity)
                .expect("every ledger fragment should recover membership");
            let fragment = subject
                .fragments
                .fragments()
                .find(|fragment| fragment.fragment_identity() == fragment_identity)
                .expect("fragment set should contain every ledger fragment");
            assert_eq!(membership.carrier_identity(), chain.carrier_identity());
            assert_eq!(
                membership.source_loop_carrier_identity(),
                source_loop_carrier.source_loop_carrier_identity()
            );
            assert_eq!(
                membership.recovered_carrier_identity(),
                source_loop_carrier.recovered_carrier_identity()
            );
            assert_eq!(
                membership.source_face_identity(),
                source_loop_carrier.source_face_identity()
            );
            assert_eq!(
                membership.source_loop_identity(),
                source_loop_carrier.source_loop_identity()
            );
            assert_eq!(
                membership.source_edge_identity(),
                source_loop_carrier.source_edge_identity()
            );
            assert_eq!(
                membership.local_frame_identity(),
                fragment.local_frame_identity()
            );
            assert_eq!(
                membership.precision_basis_identity(),
                fragment.precision_basis_identity()
            );
            assert_eq!(membership.source_senses(), fragment.source_senses());
        }

        for overlap_chain_identity in chain.overlap_chain_identities() {
            let lineage = bundle
                .overlap_chain_lineage_map()
                .lineage_for_chain_identity(overlap_chain_identity)
                .expect("every ledger overlap chain should recover lineage");
            let overlap_chain = subject
                .overlap_chains
                .chains()
                .iter()
                .find(|candidate| candidate.chain_identity() == overlap_chain_identity)
                .expect("overlap chain set should contain every ledger overlap chain");
            assert_eq!(
                lineage.fragment_identities().len(),
                overlap_chain.members().len()
            );
            assert_eq!(
                lineage.member_identities().len(),
                overlap_chain.members().len()
            );
            assert_eq!(
                lineage.boundary_roles().len(),
                overlap_chain.members().len()
            );
            for (member, fragment_identity) in overlap_chain
                .members()
                .iter()
                .zip(lineage.fragment_identities().iter())
            {
                assert_eq!(fragment_identity, member.fragment_identity());
            }
            for (member, member_identity) in overlap_chain
                .members()
                .iter()
                .zip(lineage.member_identities().iter())
            {
                assert_eq!(member_identity, member.member_identity());
            }
            for (member, source_loop_identity) in overlap_chain
                .members()
                .iter()
                .zip(lineage.source_loop_identities().iter())
            {
                let membership = bundle
                    .fragment_membership_map()
                    .membership_for_fragment_identity(member.fragment_identity())
                    .expect("every overlap member fragment should recover fragment membership");
                assert_eq!(source_loop_identity, membership.source_loop_identity());
            }
            for (member, source_edge_identity) in overlap_chain
                .members()
                .iter()
                .zip(lineage.source_edge_identities().iter())
            {
                let membership = bundle
                    .fragment_membership_map()
                    .membership_for_fragment_identity(member.fragment_identity())
                    .expect("every overlap member fragment should recover fragment membership");
                assert_eq!(source_edge_identity, membership.source_edge_identity());
            }
            for (member, boundary_role) in overlap_chain
                .members()
                .iter()
                .zip(lineage.boundary_roles().iter())
            {
                assert_eq!(*boundary_role, member.boundary_role());
            }
        }
    }

    assert_eq!(
        bundle.counters().split_chains_consumed(),
        subject.split_ledger_result.ledger().chains().len()
    );
    assert_eq!(
        bundle.counters().source_carriers_recovered(),
        bundle.source_loop_carriers().rows().len()
    );
    assert_eq!(
        bundle.counters().fragment_memberships_recovered(),
        bundle.fragment_membership_map().rows().len()
    );
    assert_eq!(
        bundle.counters().overlap_chain_lineages_recovered(),
        bundle.overlap_chain_lineage_map().rows().len()
    );
    assert_eq!(bundle.counters().dangling_reference_denials(), 0);
    assert_eq!(bundle.counters().foreign_lineage_denials(), 0);
}

#[test]
fn loop_source_provenance_identity_is_replay_stable_for_equivalent_split_lineage() {
    let canonical = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Canonical);
    let replayed = prepared_loop_reconstruction_subject(LoopFixtureEntryOrder::Replayed);
    let canonical_request = canonical.admit_loop_request();
    let replayed_request = replayed.admit_loop_request();

    let canonical_bundle = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &canonical_request,
            canonical.split_ledger_result.ledger(),
            canonical.split_ledger_result.receipt(),
            &canonical.recovered_source_carriers,
            &canonical.fragments,
            &canonical.overlap_chains,
        ),
    )
    .expect("canonical provenance should admit");
    let replayed_bundle = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &replayed_request,
            replayed.split_ledger_result.ledger(),
            replayed.split_ledger_result.receipt(),
            &replayed.recovered_source_carriers,
            &replayed.fragments,
            &replayed.overlap_chains,
        ),
    )
    .expect("replayed provenance should admit");

    assert_eq!(
        canonical_bundle.bundle_identity(),
        replayed_bundle.bundle_identity()
    );
    assert_eq!(
        canonical_bundle
            .fragment_membership_map()
            .membership_map_identity(),
        replayed_bundle
            .fragment_membership_map()
            .membership_map_identity()
    );
    assert_eq!(
        canonical_bundle
            .overlap_chain_lineage_map()
            .lineage_map_identity(),
        replayed_bundle
            .overlap_chain_lineage_map()
            .lineage_map_identity()
    );
    assert_eq!(
        canonical_bundle.source_loop_carriers().rows(),
        replayed_bundle.source_loop_carriers().rows()
    );
    assert_eq!(
        canonical_bundle.fragment_membership_map().rows(),
        replayed_bundle.fragment_membership_map().rows()
    );
    assert_eq!(
        canonical_bundle.overlap_chain_lineage_map().rows(),
        replayed_bundle.overlap_chain_lineage_map().rows()
    );
    assert_eq!(canonical_bundle.counters(), replayed_bundle.counters());
}

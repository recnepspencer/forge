use super::super::{read_repository_document, repository_root};

const SPEC: &str = "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md";

#[test]
fn specification_names_every_phase_one_destination_and_proof_boundary() {
    let spec = read_repository_document(SPEC).expect("read C.7 specification");
    for required in [
        "PhysicalDurabilityAdmissionBasis",
        "AdmittedPhysicalDurabilityPolicy",
        "PhysicalMutationRequestFingerprint",
        "PhysicalMutationAttemptBinding",
        "PhysicalMutationHandle",
        "CertifiedPriorPageBasis",
        "PageWalBasis",
        "PhysicalWritebackSettlement",
        "physical_runtime/durability/",
        "record_serving/work_semantics/durability/",
        "Store aspect-native canonical registry",
    ] {
        assert!(
            spec.contains(required),
            "C.7 destination contract lost `{required}`"
        );
    }
}

#[test]
fn phase_two_populates_the_destination_with_admission_and_runtime_owners() {
    let durability_root = repository_root()
        .join("workspaces/worth-store/crates/worth-store/src/physical_runtime/durability");
    assert!(
        durability_root.join("admission/policy.rs").is_file(),
        "Phase 2 durability admission must own the populated destination"
    );
    assert!(
        durability_root
            .join("admission/platform_basis_join.rs")
            .is_file(),
        "Phase 2 runtime binding must occupy the declared platform join"
    );
    assert!(
        durability_root.join("mod.rs").is_file()
            && durability_root.join("admission/mod.rs").is_file(),
        "the populated durability destination must retain its stable facade topology"
    );
}

#[test]
fn phase_five_replaces_scalar_barrier_with_group_owned_barrier_and_member_derivation() {
    let durability_root = repository_root()
        .join("workspaces/worth-store/crates/worth-store/src/physical_runtime/durability");
    for relative in [
        "grouping/wal_barrier/declaration.rs",
        "grouping/wal_barrier/outcome.rs",
        "grouping/wal_barrier/port.rs",
        "grouping/wal_barrier/settlement.rs",
        "grouping/member_settlement.rs",
        "data/dispatch_outcome.rs",
        "data/frame_identity.rs",
        "data/page_wal_basis.rs",
        "data/prepared_plan.rs",
        "data/prior_page_basis.rs",
        "data/writeback_join.rs",
        "mutation/progression/data_dispatched.rs",
        "mutation/progression/data_settled.rs",
        "mutation/progression/wal_durable.rs",
    ] {
        assert!(
            durability_root.join(relative).is_file(),
            "Phase 4 durability destination lost `{relative}`"
        );
    }

    let durability = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mod.rs",
    )
    .expect("read durability facade");
    for module in ["mod grouping;", "mod data;", "mod mutation;"] {
        assert!(
            durability.contains(module),
            "Phase 4 durability facade lost `{module}`"
        );
    }
    for surface in [
        "PhysicalWalGroupBarrierDeclaration",
        "PhysicalWalGroupBarrierOutcome",
        "WalDurablePhysicalMutationMembers",
        "PageWalBasis",
        "WalDurablePhysicalMutation",
        "DataDispatchedPhysicalMutation",
        "DataSettledPhysicalMutation",
    ] {
        assert!(
            durability.contains(surface),
            "Phase 4 durability facade lost `{surface}`"
        );
    }
}

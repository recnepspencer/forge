use super::test_support::{
    active_read_hold, authority, compacted_rewritten_publication, intent,
    intent_without_reachability, mismatched_read_hold, mismatched_verdict_for_rewrite, pacing,
    physical_interlock_denial, quarantine_guard, rewritten_publication_with_bytes,
    stale_dedupe_reference, unavailable_cold, verdict_for_plan, verdict_for_rewrite,
    verified_read_for_rewritten,
};
use crate::{
    BlobCompactionDenial, BlobCompactionEquivalence, BlobCompactionPacingAdmission,
    BlobCompactionRestartOutcome,
};

#[test]
fn compaction_plan_admits_blob_owned_rewrite_basis() {
    let plan = authority("phase18-plan")
        .plan_compaction(intent("phase18-plan").with_pacing_admission(pacing()))
        .expect("compaction plan should admit");

    assert_eq!(plan.counters().chunks_scanned(), 1);
    assert_eq!(plan.counters().references_transferred(), 1);
    assert_eq!(plan.counters().foreground_yields(), 2);
    assert_eq!(plan.counters().physical().publication_swaps(), 0);
}

#[test]
fn compaction_denies_missing_reachability_active_read_cold_and_pacing() {
    assert!(matches!(
        authority("phase18-no-reachability")
            .plan_compaction(intent_without_reachability("phase18-no-reachability")),
        Err(BlobCompactionDenial::MissingReachabilityProof { .. })
    ));

    assert!(matches!(
        authority("phase18-active-read")
            .plan_compaction(intent("phase18-active-read").with_read_hold(active_read_hold())),
        Err(BlobCompactionDenial::ActiveReadHold { .. })
    ));

    assert!(matches!(
        authority("phase18-wrong-read")
            .plan_compaction(intent("phase18-wrong-read").with_read_hold(mismatched_read_hold())),
        Err(BlobCompactionDenial::ReadHoldPlanMismatch { .. })
    ));

    assert!(matches!(
        authority("phase18-cold")
            .plan_compaction(intent("phase18-cold").with_cold_readiness(unavailable_cold())),
        Err(BlobCompactionDenial::UnavailableColdChunk { .. })
    ));

    assert!(matches!(
        authority("phase18-pacing").plan_compaction(
            intent("phase18-pacing")
                .with_pacing_admission(BlobCompactionPacingAdmission::Unsupported)
        ),
        Err(BlobCompactionDenial::UnsupportedSchedulerPacing { .. })
    ));

    assert!(matches!(
        authority("phase18-quarantine").plan_compaction(
            intent("phase18-quarantine")
                .with_quarantine_holds([quarantine_guard("phase18-quarantine")])
        ),
        Err(BlobCompactionDenial::QuarantineHold { .. })
    ));

    assert!(matches!(
        authority("phase18-physical-denial").plan_compaction(
            intent("phase18-physical-denial")
                .with_physical_interlock_denial(physical_interlock_denial())
        ),
        Err(BlobCompactionDenial::PhysicalInterlockDenied { .. })
    ));

    assert!(matches!(
        authority("phase18-stale-dedupe").plan_compaction(
            intent("phase18-stale-dedupe")
                .with_dedupe_references([stale_dedupe_reference("phase18-stale-dedupe")])
        ),
        Err(BlobCompactionDenial::StaleDedupeReference { .. })
    ));
}

#[test]
fn compacted_and_uncompacted_roots_preserve_blob_generation_basis() {
    let plan = authority("phase18-equivalence")
        .plan_compaction(intent("phase18-equivalence"))
        .expect("compaction plan should admit");
    let rewritten = compacted_rewritten_publication("phase18-equivalence");
    let read = verified_read_for_rewritten(&plan, &rewritten);
    assert_ne!(plan.old_root(), rewritten.chunk_tree_root());
    assert_eq!(read.object_id(), plan.basis().object_id());
    assert_eq!(read.generation(), plan.basis().generation());
    assert_eq!(read.chunk_tree_root(), rewritten.chunk_tree_root());
    assert_eq!(read.logical_content_digest(), plan.basis().logical_digest());
    assert_eq!(
        rewritten.logical_content_digest(),
        plan.basis().logical_digest()
    );
    assert_eq!(
        plan.reachability().security_metadata(),
        plan.basis().security()
    );
    assert_eq!(
        plan.placement().security_metadata(),
        plan.basis().security()
    );
    assert_eq!(
        plan.placement().stored_digest(),
        plan.basis().stored_digest()
    );
    let equivalence =
        BlobCompactionEquivalence::from_rewritten_root_and_verified_read(&plan, &rewritten, &read)
            .expect("same bytes and generation basis should prove equivalent");

    assert_eq!(equivalence.old_root(), plan.old_root());
    assert_eq!(equivalence.new_root(), rewritten.chunk_tree_root());
    assert_eq!(equivalence.object_id(), plan.basis().object_id());
    assert_eq!(equivalence.generation(), plan.basis().generation());
    assert_eq!(equivalence.security_metadata(), plan.basis().security());
    assert_eq!(equivalence.reachable_chunks(), 1);
    assert_eq!(
        equivalence.verified_bytes(),
        rewritten.canonical_basis().total_bytes()
    );
    assert_eq!(
        equivalence.canonical_basis().canonical_digest(),
        rewritten.canonical_basis().canonical_digest()
    );
    assert_eq!(
        equivalence.uncompacted_canonical_basis().canonical_digest(),
        equivalence.canonical_basis().canonical_digest()
    );
    assert_eq!(
        plan.old_canonical_basis().canonical_digest(),
        equivalence.canonical_basis().canonical_digest()
    );
}

#[test]
fn wrong_rewritten_basis_denies_equivalence_before_publication() {
    let plan = authority("phase18-wrong-equivalence")
        .plan_compaction(intent("phase18-wrong-equivalence"))
        .expect("compaction plan should admit");
    let rewritten =
        rewritten_publication_with_bytes("phase18-wrong-equivalence-other", b"other-bytes");
    let read = verified_read_for_rewritten(&plan, &rewritten);

    assert!(matches!(
        BlobCompactionEquivalence::from_rewritten_root_and_verified_read(&plan, &rewritten, &read),
        Err(BlobCompactionDenial::EquivalenceBasisMismatch { .. })
    ));
}

#[test]
fn admitted_compaction_executes_and_publishes_through_lower_physical_verdict() {
    let authority = authority("phase18-execute-publish");
    let plan = authority
        .plan_compaction(intent("phase18-execute-publish"))
        .expect("compaction plan should admit");
    let rewritten = compacted_rewritten_publication("phase18-execute-publish");
    let read = verified_read_for_rewritten(&plan, &rewritten);
    let equivalence =
        BlobCompactionEquivalence::from_rewritten_root_and_verified_read(&plan, &rewritten, &read)
            .expect("rewritten root should prove equivalent to admitted basis");
    let execution = authority
        .execute_rewrite(
            plan.clone(),
            equivalence,
            verdict_for_rewrite(&plan, &rewritten),
        )
        .expect("matching lower physical verdict should execute rewrite");
    let published = authority
        .publish_rewrite(execution)
        .expect("executed rewrite should publish observation");

    assert_eq!(published.object_id(), plan.basis().object_id());
    assert_eq!(published.generation(), plan.basis().generation());
    assert_eq!(published.old_root(), plan.old_root());
    assert_eq!(published.new_root(), rewritten.chunk_tree_root());
    assert_eq!(published.logical_digest(), plan.basis().logical_digest());
    assert_eq!(published.security_metadata(), plan.basis().security());
    assert_eq!(
        published.equivalence().uncompacted_canonical_basis(),
        plan.old_canonical_basis()
    );
}

#[test]
fn same_plan_wrong_lower_verdict_cannot_publish_rewritten_root() {
    let authority = authority("phase18-wrong-lower-verdict");
    let plan = authority
        .plan_compaction(intent("phase18-wrong-lower-verdict"))
        .expect("plan should admit");
    let rewritten = compacted_rewritten_publication("phase18-wrong-lower-verdict");
    let read = verified_read_for_rewritten(&plan, &rewritten);
    let equivalence =
        BlobCompactionEquivalence::from_rewritten_root_and_verified_read(&plan, &rewritten, &read)
            .expect("equivalence should admit for current plan");

    assert!(matches!(
        authority.execute_rewrite(
            plan.clone(),
            equivalence,
            mismatched_verdict_for_rewrite(&plan, &rewritten)
        ),
        Err(BlobCompactionDenial::MixedChunkTreePublication { .. })
    ));
}

#[test]
fn copied_equivalence_from_same_old_root_cannot_execute_another_plan() {
    let authority = authority("phase18-copied-equivalence");
    let equivalence_plan = authority
        .plan_compaction(intent("phase18-copied-equivalence-a"))
        .expect("source plan should admit");
    let execution_plan = authority
        .plan_compaction(intent("phase18-copied-equivalence-b"))
        .expect("target plan should admit");
    let rewritten = compacted_rewritten_publication("phase18-copied-equivalence-a");
    let read = verified_read_for_rewritten(&equivalence_plan, &rewritten);
    let copied_equivalence = BlobCompactionEquivalence::from_rewritten_root_and_verified_read(
        &equivalence_plan,
        &rewritten,
        &read,
    )
    .expect("source equivalence should admit");

    assert!(matches!(
        authority.execute_rewrite(
            execution_plan.clone(),
            copied_equivalence,
            verdict_for_plan(&execution_plan)
        ),
        Err(BlobCompactionDenial::MixedChunkTreePublication { .. })
    ));
}

#[test]
fn restart_outcomes_do_not_publish_mixed_chunk_tree_state() {
    let plan = authority("phase18-restart")
        .plan_compaction(intent("phase18-restart"))
        .expect("compaction plan should admit");
    let rollback = BlobCompactionRestartOutcome::roll_back(&plan);
    let residue = BlobCompactionRestartOutcome::localize_residue(&plan);

    assert!(matches!(
        rollback,
        BlobCompactionRestartOutcome::RollBackToPreCompactionPlacement { .. }
    ));
    assert!(matches!(
        residue,
        BlobCompactionRestartOutcome::ResidueLocalized(_)
    ));
}

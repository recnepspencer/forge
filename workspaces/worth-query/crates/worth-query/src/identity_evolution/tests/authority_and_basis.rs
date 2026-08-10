use crate::identity_evolution::{
    admit_identity_evolution_query, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionQueryContext, IdentityEvolutionReplayArtifact, InspectorIdentityArtifact,
    LineageTraversalDescriptor,
};

#[test]
fn query_minted_authority_survives_planning_replay_and_inspection() {
    let canonical = crate::harness::fixtures::canonical_bundles::runtime_detail_bundle();
    let validated = crate::harness::fixtures::validated_bundles::runtime_detail_bundle();
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();

    let direct = canonical.query().authority();
    let fluent = validated.query().canonical_authority();
    let planned = preflight.plan().query().canonical_authority();
    assert_eq!(direct, fluent);
    assert_eq!(fluent, planned);

    let context = IdentityEvolutionQueryContext::lineage_traversal(
        &planned,
        preflight.basis(),
        LineageTraversalDescriptor::direct_replacement("entity:authority-continuity"),
    );
    let admitted = admit_identity_evolution_query(context).expect("authority-bound admission");
    let execution = execute_admitted_identity_evolution_query(&admitted)
        .expect("authority-bound identity evolution");
    let evidence =
        crate::identity_evolution::IdentityEvolutionCertificationResultEvidence::from_execution_artifact(
            &execution,
        );
    let replay = IdentityEvolutionReplayArtifact::from_result_evidence(&evidence);
    let inspection = InspectorIdentityArtifact::from_result_evidence(&evidence);

    assert_eq!(evidence.query_authority(), &planned);
    assert_eq!(replay.query_authority(), Some(&planned));
    assert_eq!(inspection.query_authority(), &planned);
    assert_eq!(evidence.basis_proof(), preflight.basis().proof());
    assert_eq!(replay.basis_proof(), Some(preflight.basis().proof()));
    assert_eq!(inspection.basis_proof(), preflight.basis().proof());
    assert_eq!(replay.query_digest(), planned.digest().as_str());
}

#[test]
fn equal_digest_text_cannot_collapse_distinct_basis_generations() {
    let left_preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let right_preflight =
        crate::harness::fixtures::execution_preflights::runtime_preflight_with_snapshot_identity(
            crate::harness::fixtures::resolved_bases::alternate_snapshot_identity(),
        );
    let collision_label = "hostile-equal-rendered-basis-digest";
    let left_digest = crate::identity::BasisDigest::from_collision_for_test(
        collision_label,
        left_preflight.basis().proof().identity().clone(),
    );
    let right_digest = crate::identity::BasisDigest::from_collision_for_test(
        collision_label,
        right_preflight.basis().proof().identity().clone(),
    );
    let left = left_preflight.basis().clone().replace_proof_for_test(
        crate::basis::ResolvedBasisProof::from_identity_and_digest_for_test(
            left_preflight.basis().proof().identity().clone(),
            left_digest,
        ),
    );
    let right = right_preflight.basis().clone().replace_proof_for_test(
        crate::basis::ResolvedBasisProof::from_identity_and_digest_for_test(
            right_preflight.basis().proof().identity().clone(),
            right_digest,
        ),
    );

    assert_eq!(
        left.proof().digest().as_str(),
        right.proof().digest().as_str()
    );
    assert_ne!(left.proof(), right.proof());
    assert!(!crate::basis::snapshot_resolution_report(&left).certifies(&right));

    let authority = left_preflight.plan().query().canonical_authority();
    let context = IdentityEvolutionQueryContext::correspondence_identity_comparison(
        &authority,
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        &left,
        &right,
        CorrespondenceIdentityComparison::advisory_between("left", "right"),
    );
    let (left_proof, right_proof) = context
        .correspondence_basis_proofs()
        .expect("comparison retains both sealed generations");
    assert_ne!(left_proof.identity(), right_proof.identity());

    let admitted = admit_identity_evolution_query(context).expect("structurally distinct bases");
    let execution = execute_admitted_identity_evolution_query(&admitted)
        .expect("collision-safe comparison execution");
    let inspection = InspectorIdentityArtifact::from_result_bundle(execution.result_bundle());
    assert_eq!(inspection.basis_proof().identity(), left.proof().identity());
}

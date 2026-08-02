use super::{export_block, read_repository_document};

const DURABILITY_EXPORTS: [&str; 27] = [
    "CompletedPhysicalRootPublication",
    "IndeterminatePhysicalCurrentRootAdvance",
    "IndeterminatePhysicalRootNamespaceDurability",
    "IndeterminatePhysicalRootPublicationPreparation",
    "IndeterminatePhysicalRootReplacement",
    "PhysicalCurrentRootAdvanceFailureCause",
    "PhysicalCurrentRootAdvanceOutcome",
    "PhysicalRootCandidateSynchronizationFailureCause",
    "PhysicalRootCandidateWriteFailureCause",
    "PhysicalRootCandidateWriteFailurePosture",
    "PhysicalRootNamespaceDurabilityFailureCause",
    "PhysicalRootNamespaceDurabilityNotStarted",
    "PhysicalRootNamespaceDurabilityOutcome",
    "PhysicalRootPublicationMemberIdentity",
    "PhysicalRootPublicationPreparationFailureCause",
    "PhysicalRootPublicationPreparationNotStarted",
    "PhysicalRootPublicationPreparationOutcome",
    "PhysicalRootPublicationTransitionDenial",
    "PhysicalRootPublicationWorkFailureCause",
    "PhysicalRootReplacementFailureCause",
    "PhysicalRootReplacementNotStarted",
    "PhysicalRootReplacementOutcome",
    "RetainedPhysicalRoot",
    "RootNamespaceDurablePhysicalMutationMembers",
    "RootPublicationPhysicalMutationMember",
    "RootPublicationPreparedPhysicalMutationMembers",
    "RootReplacedPhysicalMutationMembers",
];

const SUBMISSION_METHODS: [&str; 6] = [
    "pub fn prepare_root_publication",
    "pub fn continue_root_publication_preparation",
    "pub fn continue_root_publication_candidate",
    "pub fn replace_prepared_root",
    "pub fn synchronize_replaced_root_namespace",
    "pub fn advance_namespace_durable_root",
];

pub(super) fn assert_reachability(durability_exports: &str) {
    for expected in DURABILITY_EXPORTS {
        assert!(
            durability_exports.contains(expected),
            "Phase 7 durability surface `{expected}` is hidden from the physical runtime facade"
        );
    }

    let record_serving = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/mod.rs",
    )
    .expect("read record-serving facade");
    let publication_exports = export_block(&record_serving, "pub use publication::{");
    for expected in [
        "PhysicalRecordSubmission",
        "RootPublicationCandidatePlan",
        "RootPublicationPlanningMembers",
    ] {
        assert!(
            publication_exports.contains(expected),
            "Phase 7 record-serving surface `{expected}` is hidden from its facade"
        );
    }
    assert!(
        record_serving.contains("RecordRootPlanningObservation"),
        "Phase 7 shared root-planning observation is hidden from the physical runtime facade"
    );

    let submission = read_repository_document(
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    )
    .expect("read physical record submission owner");
    for expected in SUBMISSION_METHODS {
        assert!(
            !submission.contains(expected),
            "Phase 7 phase-driving operation `{expected}` remains on the ordinary facade"
        );
    }
}
